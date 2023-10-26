use std::{io::BufReader, time::Instant};

use clap::{Parser, Subcommand};
use sem_lmc_algorithm::{
    algorithm::{EvePos, Position},
    normalizer::normalize_system,
};
use sem_lmc_common::{InputFlags, SpecOutput, VerificationOutput};
use sem_lmc_pg::ParityGameSpec;

#[derive(Debug, Parser)]
#[command(about = "A local model checker which leverages parity games and symbolic exists-moves", long_about = None)]
struct Cli {
    /// If enabled, the underlying system of fixpoint equations is normalized during the preprocessing phase
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
        /// A path to a file containing the symbolic existential moves, for each
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

        /// The node from which is verified whether if the selected player has a winning strategy
        node: String,
    },
    #[command(arg_required_else_help = true)]
    MuAld { lts_ald: std::path::PathBuf, fix_system: std::path::PathBuf },
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

            let arity =
                sem_lmc_algorithm::parse::parse_fun_arity(arity_src.unwrap())
                    .unwrap();
            let fix_system = sem_lmc_algorithm::parse::parse_fixpoint_system(
                arity.clone(),
                fix_system_src.unwrap(),
            )
            .unwrap();

            let var_name = fix_system
                .iter()
                .enumerate()
                .find(|(i, _)| *i == position - 1)
                .map(|(_, x)| &x.var)
                .unwrap();

            let basis =
                sem_lmc_algorithm::parse::parse_basis(basis_src.unwrap())
                    .unwrap();
            let moves_system = sem_lmc_algorithm::parse::parse_symbolic_system(
                arity,
                basis.clone(),
                moves_src.unwrap(),
            )
            .unwrap();

            let start = Instant::now();

            let normalized_system = if normalize {
                Some(normalize_system(&fix_system))
            } else {
                None
            };
            let composed_system = sem_lmc_algorithm::moves_compositor::compose_moves::compose_moves(if normalize { &normalized_system.as_ref().unwrap().0} else {&fix_system}, &moves_system, &basis);
            let preproc = start.elapsed();

            let parity_game = sem_lmc_algorithm::algorithm::LocalAlgorithm {
                symbolic_moves: &composed_system,
                fix_system: &fix_system,
                basis: &basis,
            };

            let pos = Position::Eve(EvePos {
                b: basis_element,
                i: fix_system
                    .iter()
                    .enumerate()
                    .find(|(_, fix_eq)| {
                        if normalize {
                            normalized_system
                                .as_ref()
                                .unwrap()
                                .1
                                .get(var_name)
                                .expect(&format!(
                                    "Cannot find variable with index {}",
                                    position
                                ))
                                == &fix_eq.var
                        } else {
                            var_name == &fix_eq.var
                        }
                    })
                    .map(|(i, _)| i + 1)
                    .expect(&format!(
                        "Cannot find variable with index {}",
                        position
                    )),
            });

            let start = Instant::now();
            let result = parity_game.local_check(pos);
            let algo_time = start.elapsed();

            let result = VerificationOutput {
                fix_system: fix_system,
                fix_system_normalized: normalized_system.map(|x| x.0),
                moves: moves_system,
                moves_composed: composed_system,
                preproc_time: preproc,
                algorithm_time: algo_time,
                result: format!("The winner is the {}", result),
            };
            println!(
                "{}",
                if explain {
                    result.format_verbose()
                } else {
                    result.to_string()
                }
            )
        }

        Commands::Pg { game_path, node } => {
            let p = ParityGameSpec::new(
                &mut BufReader::new(
                    std::fs::File::open(game_path.as_path()).unwrap(),
                ),
                node,
            );

            let result = p
                .verify(&InputFlags { normalize })
                .expect("Something unexpected happened");
            println!(
                "{}",
                if explain {
                    result.format_verbose()
                } else {
                    result.to_string()
                }
            )
        }
        Commands::MuAld { .. } => {
            unimplemented!()
        }
    };
}
