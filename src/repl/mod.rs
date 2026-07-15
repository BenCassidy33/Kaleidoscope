use std::io::{self, BufRead, Write};
use thiserror::Error;
use derive_more::Display;

use crate::args::Args;

const HELLO_MESSAGE: &'static str = "";

#[derive(Error, Debug)]
pub enum ReplError {
    #[error("IO Error!")]
    IoError(#[from] io::Error),

    #[error("Unknown error...")]
    Unknown
}

pub fn run_repl(args: &Args) -> Result<(), ReplError> {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();

    let mut line = String::new();
    loop {
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;
        println!("{}", line);
    }
}
