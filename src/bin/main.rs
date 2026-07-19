#![allow(dead_code, unused_variables)]

use std::ops::Sub;

use clap::Parser;
use kaleidoscope::{
    args::{Args, Subcommands},
    repl::run_repl,
};
use miette::ErrReport;

fn main() -> miette::Result<()> {
    let args = Args::parse();

    if args.file.is_some() {
        todo!();
    }

    if args.subcommands.is_none() {
        run_repl(true).map_err(ErrReport::from_err)?
    }

    match &args.subcommands.clone().unwrap() {
        Subcommands::Interactive { show_hello } => {
            Ok(run_repl(*show_hello).map_err(ErrReport::from_err)?)
        }

        _ => todo!(),
    }
}
