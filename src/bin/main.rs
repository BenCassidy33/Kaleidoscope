#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, fs::File, io::{BufWriter, Write, stdout}};

use clap::Parser;
use kaleidoscope::{
    Lambda, LambdaKind, UnwrapExpressions,
    args::{Args, Subcommands},
    repl::run_repl,
    types::Node,
};
use miette::ErrReport;

fn main() -> miette::Result<()> {
    let args = Args::parse();

    if args.subcommands.is_none() {
        run_repl(true).map_err(ErrReport::from_err)?
    }

    match &args.subcommands.clone().unwrap() {
        Subcommands::Interactive { show_hello } => {
            run_repl(*show_hello).map_err(ErrReport::from_err)?
        }

        Subcommands::Run { .. } => run(&args)?,
    };

    Ok(())
}

pub fn run(args: &Args) -> miette::Result<()> {
    let Some(Subcommands::Run { ref files, ref out, stdout: stout, only_show_reductions }) = args.subcommands else { unreachable!() };

    let mut out: Box<dyn Write> = if !stout {
        let f = File::options()
        .create(true)
        .truncate(true)
        .open(out.clone())
        .map_err(ErrReport::from_err)?;

        Box::new(BufWriter::new(f))
    } else {
        Box::new(stdout().lock())
    };

    let mut content = String::new();

    for file in files {
        content.push_str(&std::fs::read_to_string(file).map_err(ErrReport::from_err)?);
    }

    let expressions = Lambda::parse(content).unwrap_expressions()?;
    let mut assignments = HashMap::new();

    let mut originals = Vec::new();
    let mut replaced = Vec::new();
    let mut reduced = Vec::new();

    for node in expressions {
        match node.kind() {
            LambdaKind::Assignment { ident, body } => {
                assignments.insert(ident.clone(), body.clone());
            }

            LambdaKind::Statement { body } => {
                let original = body.clone();

                let rep = body.clone().replace_assignments(&assignments);

                let red = if let Node::Application(application) = rep.clone() {
                    application.reduce_self()?
                } else {
                    original.clone()
                };

                originals.push(original);
                replaced.push(rep);
                reduced.push(red);
            }
        }
    };


    let originals = originals.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("\n");
    let replaced = replaced.iter().map(|n: &Node| n.to_string()).collect::<Vec<_>>().join("\n");
    let reduced = reduced.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("\n");
    writeln!(
        out,
        "// {dashes} Original {dashes}\n{originals}\n",
        dashes = "=".repeat(20),
    )
        .map_err(ErrReport::from_err)?;

    writeln!(
        out,
        "// {dashes} Variable Replacements {dashes}\n{replaced}\n",
        dashes = "=".repeat(20),
    )
        .map_err(ErrReport::from_err)?;

    writeln!(
        out,
        "// {dashes} Reductions {dashes}\n{reduced}\n",
        dashes = "=".repeat(20),
    )
        .map_err(ErrReport::from_err)?;

    Ok(())
}
