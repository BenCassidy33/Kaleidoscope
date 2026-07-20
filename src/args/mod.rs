pub mod run;

use std::{fs::File, path::PathBuf};
use clap::{Parser, Subcommand};

pub struct KaleidoscopeError {

}

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[arg(short, long)]
    pub file_path: Option<PathBuf>,

    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Subcommands {
    Interactive {
        // Shows the hello message when starting the interactive repl
        #[arg(long, default_value_t = true)]
        show_hello: bool,
    },

    Run {
        file: PathBuf
    },

    Check {},
}

pub fn read_files(main_file_path: PathBuf) -> anyhow::Result<Vec<String>> {
    let mut main_file = File::open(main_file_path);
    todo!()
}

// fn read_file(buf: &mut String) -> anyhow::Result<(
