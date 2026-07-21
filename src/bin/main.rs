#![allow(dead_code, unused)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write, stdout},
};

use clap::Parser;
use kaleidoscope::{
    Lambda, LambdaAssignment, LambdaKind, LambdaStatement, UnwrapExpressions, UnzipExpressions,
    args::{Args, Subcommands},
    interpreter::InterpretingError,
    repl::run_repl,
    stdlib::{self, generate_lambda_number, stdlib_assignments},
    types::{Node, VariableNode},
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
    let Some(Subcommands::Run(ref run_args)) = args.subcommands else {
        unreachable!()
    };

    let mut out: Box<dyn Write> = if !run_args.stdout {
        let f = File::options()
            .create(true)
            .truncate(true)
            .open(run_args.out.clone())
            .map_err(ErrReport::from_err)?;

        Box::new(BufWriter::new(f))
    } else {
        Box::new(stdout().lock())
    };

    let mut content = String::new();

    for file in run_args.files.clone() {
        content.push_str(&std::fs::read_to_string(file).map_err(ErrReport::from_err)?);
    }

    let expressions = Lambda::parse(content).unwrap_expressions()?;
    let mut assignments = if run_args.nostdlib {
        HashMap::new()
    } else {
        stdlib::stdlib_assignments()
    };

    let mut originals = Vec::new();
    let mut replaced = Vec::new();
    let mut reduced = Vec::new();

    for node in expressions {
        match node.kind() {
            LambdaKind::Assignment(LambdaAssignment { ident, body }) => {
                if let Some(assignment) = assignments.insert(ident.clone(), body.clone()) {
                    if let Some(stdlib_key) = assignments
                        .keys()
                        .find(|k| k.to_string() == ident.to_string() && *k.is_stdlib())
                    {
                        Err(InterpretingError::new(
                            format!("{} := {}", ident, body),
                            Some(format!(
                                "Cannot redefine variable '{}' defined in the standard library. '{}' is defined as:\n{} := {}",
                                ident, ident, stdlib_key, assignment
                            )),
                            0..ident.to_string().len(),
                            None,
                            None,
                        ))?;
                    }

                    println!(
                        "Warning! Reassigning previously declared variable: {}",
                        ident
                    )
                };
            }

            LambdaKind::Statement(LambdaStatement { body }) => {
                let original = body.clone();

                let rep = body.clone().replace_assignments(&assignments)?;

                let red = if let Node::Application(application) = rep.clone() {
                    let mut last = application.reduce_self()?;

                    while let next = last.clone().reduce_self()?
                        && next != last
                    {
                        last = next;
                    }

                    last
                } else {
                    original.clone()
                };

                originals.push(original);
                replaced.push(rep);
                reduced.push(red);
            }
        }
    }

    let variables = assignments
        .keys()
        .zip(assignments.values())
        .filter(|(k, v)| !*k.is_stdlib() || (*k.is_stdlib() && run_args.print_stdlib_vars))
        .map(|(k, v)| format!("{} := {}", k, v))
        .collect::<Vec<_>>()
        .join("\n");

    let originals = originals
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let replaced = replaced
        .iter()
        .map(|n: &Node| n.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let reduced = reduced
        .iter()
        .map(|n| format!("{}", n))
        .collect::<Vec<_>>()
        .join("\n");

    if !variables.is_empty() {
        writeln!(
            out,
            "// {dashes} Variables {dashes}\n{variables}\n",
            dashes = "=".repeat(20),
        )
        .map_err(ErrReport::from_err)?;
    }

    if !originals.is_empty() {
        writeln!(
            out,
            "// {dashes} Original {dashes}\n{originals}\n",
            dashes = "=".repeat(20),
        )
        .map_err(ErrReport::from_err)?;
    }

    if !replaced.is_empty() {
        writeln!(
            out,
            "// {dashes} Variable Replacements {dashes}\n{replaced}\n",
            dashes = "=".repeat(20),
        )
        .map_err(ErrReport::from_err)?;
    }

    if !reduced.is_empty() {
        writeln!(
            out,
            "// {dashes} Reductions {dashes}\n{reduced}\n",
            dashes = "=".repeat(20),
        )
        .map_err(ErrReport::from_err)?;
    }

    Ok(())
}
