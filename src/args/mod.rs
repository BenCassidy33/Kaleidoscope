use clap::{Parser, Subcommand};
use std::{fs::File, path::PathBuf};

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
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
        #[arg(default_value = "./main.lmda")]
        files: Vec<PathBuf>,

        // The file to write the results to
        #[arg(short, long, default_value = "./out.lmda")]
        out: PathBuf,

        // Write to stdout instead of a file.
        #[arg(long, default_value_t = false)]
        stdout: bool,

        // only write the end results of reductions to file/stdout
        #[arg(long, default_value_t = false)]
        only_show_reductions: bool
    },
}
