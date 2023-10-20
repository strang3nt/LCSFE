use std::io::BufReader;

use clap::{Parser, Subcommand};
use sem_lmc_algorithm::algorithm::{Position, EvePos};
use sem_lmc_common::SpecOutput;
use sem_lmc_pg::ParityGameSpec;

#[derive(Debug, Parser)]
#[command(name = "semlmc")]
#[command(about = "A local model checker which leverages parity games and Symbolic Existential Moves", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {

    #[command(arg_required_else_help = true)]
    Debug {
        arity: std::path::PathBuf,
        fix_system: std::path::PathBuf,
        basis: std::path::PathBuf,
        moves_system: std::path::PathBuf,
        basis_element: String,
        position: usize,
    },

    #[command(arg_required_else_help = true)]
    Pg {
        #[arg(short, long)]
        game_path: std::path::PathBuf,
        #[arg(short, long)]
        player: bool,
        #[arg(short, long)]
        node: String,
    },
    #[command(arg_required_else_help = true)]
    MuAld { lts_ald: std::path::PathBuf, fix_system: std::path::PathBuf },
}

fn main() {
    let args = Cli::parse();

    match args.command {

        Commands::Debug { arity, fix_system, basis, moves_system, basis_element, position } => {
            let pos = Position::Eve(EvePos{ b: basis_element, i: position });

            let arity_src = std::fs::read_to_string(arity);
            let fix_system_src = std::fs::read_to_string(fix_system);
            let basis_src = std::fs::read_to_string(basis);
            let moves_src = std::fs::read_to_string(moves_system);

            let arity = sem_lmc_algorithm::parse::parse_fun_arity(arity_src.unwrap()).unwrap();
            let fix_system = sem_lmc_algorithm::parse::parse_fixpoint_system(arity.clone(), fix_system_src.unwrap()).unwrap();
            let basis = sem_lmc_algorithm::parse::parse_basis(basis_src.unwrap()).unwrap();
            let moves_system = sem_lmc_algorithm::parse::parse_symbolic_system(arity, basis.clone(), moves_src.unwrap()).unwrap();

            let parity_game = sem_lmc_algorithm::algorithm::ParityGame {
                symbolic_moves: &sem_lmc_algorithm::moves_compositor::compose_moves::compose_moves(&fix_system, &moves_system, &basis),
                fix_system: &fix_system,
                basis: &basis
            };

            println!("{:?}", parity_game.local_check(pos));
        
        }

        Commands::Pg { game_path, player, node } => {
            let p = ParityGameSpec::new(
                &mut BufReader::new(
                    std::fs::File::open(game_path.as_path()).unwrap(),
                ),
                player,
                node,
            );
            println!("{}", p.get_ver())
        }
        Commands::MuAld { .. } => {
            unimplemented!()
        }
    };
}
