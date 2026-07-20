#![allow(dead_code, unused_variables)]

use clap::Parser;
use kaleidoscope::{
    args::{Args, Subcommands, read_files}, invocations::include::IncludeInvocation, repl::run_repl,
};
use miette::ErrReport;

fn main() -> miette::Result<()> {

    dbg!(IncludeInvocation::parse_include_statement("include! vars lib with G"));
    return Ok(());


    let args = Args::parse();

    if let Some(fp) = args.file_path {
        let _ = read_files(fp);
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
