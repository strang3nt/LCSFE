use std::io::BufReader;

use clap::{Parser, Subcommand};
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
