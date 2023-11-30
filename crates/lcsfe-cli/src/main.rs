use std::{io::BufReader, time::Instant};

use clap::{Parser, Subcommand};
use rustc_hash::FxHashMap as HashMap;
use lcsfe_algorithm::normalizer::normalize_system;
use lcsfe_common::{InputFlags, PreProcOutput, SpecOutput, VerificationOutput};
use lcsfe_pg::ParityGameSpec;

#[derive(Debug, Parser)]
#[command(about = "A local model checker which leverages parity games and symbolic exists-moves", long_about = None)]
struct Cli {
    /// If enabled, the underlying system of fixpoint equations is normalized
    /// during the preprocessing phase
    #[arg(short, long)]
    normalize: bool,
    #[arg(short, long)]
    /// If enabled, prints to stdout the underlying system of fixpoint equations
    /// before and after normalization, and the symbolic exists-moves, before
    /// and after composition. It does so only after the computation
    explain: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    /// A user should choose this command only if willing to deal with the
    /// underlying engine.
    Debug {
        /// A path to a file containing the operators and arity
        arity: std::path::PathBuf,
        /// A path to a file containing a system of fixpoint equations
        fix_system: std::path::PathBuf,
        /// A path to a file containing the basis
        basis: std::path::PathBuf,
        /// A path to a file containing the symbolic exists-moves, for each
        /// basis element and function, to be composed
        moves_system: std::path::PathBuf,
        /// A string representing the element of the basis whose membership in
        /// the solution you want to verify
        element_of_basis: String,
        /// The index of the fixpoint equation from which you want to start the
        /// analysis, it starts from 1
        index: usize,
    },

    #[command(arg_required_else_help = true)]
    /// A solver for parity games.
    Pg {
        /// A path to a file containing a parity game, in PGSolver format
        game_path: std::path::PathBuf,

        /// The node from which is verified whether if the selected player
        /// has a winning strategy
        node: String,
    },
    #[command(arg_required_else_help = true)]
    MuAld {
        /// Path to a file containing an Aldebaran specification
        lts_ald: std::path::PathBuf,
        /// Path to a file containing a mu-calculus formula
        mu_calc: std::path::PathBuf,
        /// The state of the Aldebaran specification from which the
        /// verification starts
        state: String,
    },
}

fn main() {
    let args = Cli::parse();

    let normalize = args.normalize;
    let explain = args.explain;

    match args.command {
        Commands::Debug {
            arity,
            fix_system,
            basis,
            moves_system,
            element_of_basis: basis_element,
            index: position,
        } => {
            let arity_src = std::fs::read_to_string(arity);
            let fix_system_src = std::fs::read_to_string(fix_system);
            let basis_src = std::fs::read_to_string(basis);
            let moves_src = std::fs::read_to_string(moves_system);

            let arity = lcsfe_algorithm::parse::parse_fun_arity(arity_src.unwrap()).unwrap();
            let fix_system =
                lcsfe_algorithm::parse::parse_fixpoint_system(&arity, fix_system_src.unwrap())
                    .unwrap();

            let var_name = fix_system
                .iter()
                .enumerate()
                .find(|(i, _)| *i == position - 1)
                .map(|(_, x)| x.var.to_owned())
                .unwrap();

            let basis = lcsfe_algorithm::parse::parse_basis(basis_src.unwrap()).unwrap();
            let moves_system =
                lcsfe_algorithm::parse::parse_symbolic_system(&arity, &basis, moves_src.unwrap())
                    .unwrap();

            let start = Instant::now();

            let fix_system = if normalize {
                normalize_system(fix_system)
            } else {
                (fix_system, HashMap::default())
            };
            let composed_system =
                lcsfe_algorithm::ast::symbolic_moves_composed::SymbolicExistsMoves::compose(
                    &fix_system.0,
                    &moves_system,
                    &basis,
                );
            let preproc_time = start.elapsed();

            let pos = (
                basis_element,
                if normalize {
                    fix_system
                        .0
                        .iter()
                        .enumerate()
                        .find_map(|(i, fix_eq)| {
                            if fix_system.1.get(&var_name).unwrap_or_else(|| {
                                panic!("Cannot find variable with index {}", position)
                            }) == &fix_eq.var
                            {
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| panic!("Cannot find variable with index {}", position))
                } else {
                    position - 1
                },
            );

            let preproc = PreProcOutput {
                preproc_time,
                moves: composed_system,
                fix_system: fix_system.0,
                var_map: fix_system.1,
                var: var_name,
            };

            if explain {
                preproc.print_explain()
            } else {
                println!("{}", preproc)
            }

            let parity_game = lcsfe_algorithm::algorithm::LocalAlgorithm {
                symbolic_moves: &preproc.moves,
                fix_system: &preproc.fix_system,
            };

            let start = Instant::now();
            let result = parity_game.local_check(pos.0, pos.1);
            let algo_time = start.elapsed();

            let result = VerificationOutput {
                algorithm_time: algo_time,
                result: format!("The winner is the {}", result),
            };

            println!("{}", result)
        }

        Commands::Pg { game_path, node } => {
            let p = ParityGameSpec::new(
                &mut BufReader::new(std::fs::File::open(game_path.as_path()).unwrap()),
                node,
            );

            print_results(p, explain, InputFlags { normalize })
        }
        Commands::MuAld {
            lts_ald,
            mu_calc: fix_system,
            state,
        } => {
            let mu_ald = lcsfe_mu_ald::MuAld::new(
                &mut BufReader::new(std::fs::File::open(lts_ald.as_path()).unwrap()),
                &mut BufReader::new(std::fs::File::open(fix_system.as_path()).unwrap()),
                state,
            );

            print_results(mu_ald.unwrap(), explain, InputFlags { normalize })
        }
    };
}

fn print_results(results: impl SpecOutput, explain: bool, input_flags: InputFlags) {
    let preproc = results
        .pre_proc(&input_flags)
        .expect("Preprocessing failed");
    if explain {
        preproc.print_explain();
    } else {
        println!("{}", preproc);
    }

    let result = results
        .verify(&input_flags, &preproc)
        .expect("Something unexpected happened");
    println!("{}", result);
}
