pub mod processor;
pub mod writer;

pub use machine_code::*;

use std::path::PathBuf;

use clap::Parser;
use utils::io::{file_exists, read_file_to_string, read_stdin_to_string};
use utils::Executor;

use crate::{processor::process, writer::print};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_parser = file_exists)]
    input: Option<PathBuf>,
    #[arg(short, long, value_parser = clap_num::maybe_hex::<u16>)]
    base: u16,
}

impl Executor for Args {
    fn execute(&self) {
        let program = if let Some(path) = &self.input {
            read_file_to_string(path)
        } else {
            read_stdin_to_string()
        };
        let process_result = process(&program, self.base);
        print(process_result);
    }
}
