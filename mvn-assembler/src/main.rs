use clap::Parser;
use utils::Executor;

use mvn_assembler::Args;

fn main() {
    let args = Args::parse();
    args.execute();
}
