use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[derive(Parser, Clone, Debug, PartialEq)]
pub struct RunArgs {
    #[arg(default_value = "./main.lmda")]
    pub files: Vec<PathBuf>,

    // The file to write the results to
    #[arg(short, long, default_value = "./out.lmda")]
    pub out: PathBuf,

    // Write to stdout instead of a file.
    #[arg(long, default_value_t = false)]
    pub stdout: bool,

    // only write the end results of reductions to file/stdout
    #[arg(long, default_value_t = false)]
    pub only_show_reductions: bool,

    // do not include stdlib when running
    #[arg(long, default_value_t = false)]
    pub nostdlib: bool,

    // do not include stdlib when running
    #[arg(long, default_value_t = false)]
    pub print_stdlib_vars: bool,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Subcommands {
    Interactive {
        // Shows the hello message when starting the interactive repl
        #[arg(long, default_value_t = true)]
        show_hello: bool,
    },

    Run(RunArgs),
}
