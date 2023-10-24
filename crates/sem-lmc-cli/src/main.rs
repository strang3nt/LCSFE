use std::io::BufReader;

use clap::{Parser, Subcommand};
use sem_lmc_algorithm::{
    algorithm::{EvePos, Position},
    normalizer::normalize_system,
};
use sem_lmc_common::{InputFlags, SpecOutput};
use sem_lmc_pg::ParityGameSpec;

#[derive(Debug, Parser)]
#[command(name = "semlmc")]
#[command(about = "A local model checker which leverages parity games and Symbolic Existential Moves", long_about = None)]
struct Cli {
    /// If enabled, the underlying system of fixpoint equations is normalized during the preprocessing phase
    #[arg(short, long)]
    normalize: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Debug {
        /// A path to a file containing the operators and arity
        #[arg(short, long)]
        arity: std::path::PathBuf,
        /// A path to a file containing a system of fixpoint equations
        #[arg(short, long)]
        fix_system: std::path::PathBuf,
        /// A path to a file containing the basis
        #[arg(short, long)]
        basis: std::path::PathBuf,
        /// A path to a file containing the symbolic existential moves, for each
        /// basis element and function, to be composed
        #[arg(short, long)]
        moves_system: std::path::PathBuf,
        /// A string representing the element of the basis whose membership in
        /// the solution you want to verify
        #[arg(short, long)]
        element_of_basis: String,
        /// The index of the fixpoint equation from which you want to start the
        /// analysis, it starts from 1
        #[arg(short, long)]
        index: usize,
    },

    #[command(arg_required_else_help = true)]
    Pg {
        /// A path to a file containing a parity game
        #[arg(short, long)]
        game_path: std::path::PathBuf,

        /// The node from which is verified whether if the selected player has a winning strategy
        #[arg(short, long)]
        node: String,
    },
    #[command(arg_required_else_help = true)]
    MuAld { lts_ald: std::path::PathBuf, fix_system: std::path::PathBuf },
}

fn main() {
    let args = Cli::parse();

    let normalize = args.normalize;

    match args.command {
        Commands::Debug {
            arity,
            fix_system,
            basis,
            moves_system,
            element_of_basis: basis_element,
            index: position,
        } => {
            let pos = Position::Eve(EvePos { b: basis_element, i: position });

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

            let fix_system = if normalize {
                normalize_system(&fix_system)
            } else {
                fix_system
            };

            let basis =
                sem_lmc_algorithm::parse::parse_basis(basis_src.unwrap())
                    .unwrap();
            let moves_system = sem_lmc_algorithm::parse::parse_symbolic_system(
                arity,
                basis.clone(),
                moves_src.unwrap(),
            )
            .unwrap();

            let parity_game = sem_lmc_algorithm::algorithm::LocalAlgorithm {
                symbolic_moves: &sem_lmc_algorithm::moves_compositor::compose_moves::compose_moves(&fix_system, &moves_system, &basis),
                fix_system: &fix_system,
                basis: &basis
            };

            println!("{:?}", parity_game.local_check(pos));
        }

        Commands::Pg { game_path, node } => {
            let p = ParityGameSpec::new(
                &mut BufReader::new(
                    std::fs::File::open(game_path.as_path()).unwrap(),
                ),
                node,
            );
            println!(
                "{}",
                p.verify(&InputFlags { normalize })
                    .expect("Something unexpected happened")
            )
        }
        Commands::MuAld { .. } => {
            unimplemented!()
        }
    };
}
