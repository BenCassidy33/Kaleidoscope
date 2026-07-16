use console::Term;
use std::io::{self, Write};
use thiserror::Error;

use crate::args::{Args, Subcommands};

const HELLO_MESSAGE: &'static str = "Welcome to the Kaleidoscope repl. Use /help for help!";

#[derive(Error, Debug)]
pub enum ReplError {
    #[error("IO Error!")]
    IoError(#[from] io::Error),

    #[error("Unknown error...")]
    Unknown,
}

pub fn run_repl(args: &Args) -> Result<(), ReplError> {
    let mut term = Term::stdout();
    let Some(Subcommands::Interactive { show_hello }) = args.subcommands else {
        unreachable!()
    };

    if show_hello {
        term.write_line(HELLO_MESSAGE)?;
    }

    loop {
        term.write_all(b"> ")?;
        let line = term.read_line()?;

        if line.starts_with("/") {
            run_cmd(&line);
            continue;
        }
    }
}

pub fn run_cmd(_cmd: &str) {
    todo!()
}
