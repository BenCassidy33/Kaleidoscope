use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Copy, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub subcommands: Option<Subcommands>,
}

#[derive(Subcommand, Clone, Copy, Debug, PartialEq)]
pub enum Subcommands {
    Interactive { 
        // Shows the hello message when starting the interactive repl
        #[arg(long)]
        show_hello: bool 
    },
}
