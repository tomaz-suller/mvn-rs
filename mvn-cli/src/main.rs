use std::path::PathBuf;

use clap::{Parser, Subcommand};
use utils::Executor;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Assemble(assembler::Args),
    Link(linker::Args),
    Relocate(relocator::Args),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Assemble(args) => args.execute(),
        Commands::Link(args) => args.execute(),
        Commands::Relocate(args) => args.execute(),
    }
}

