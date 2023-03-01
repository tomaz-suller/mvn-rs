use clap::Parser;
use utils::Executor;

use mvn_relocator::Args;

fn main() {
    let args = Args::parse();
    args.execute();
}
