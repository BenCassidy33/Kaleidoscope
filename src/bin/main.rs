#![allow(dead_code, unused_variables)]

use std::io::stdout;

use clap::Parser;
use kaleidoscope::{
    Lambda, UnwrapExpressions, UnzipExpressions, args::{Args, Subcommands}, repl::run_repl,
};

// TODO: Inline this for actual releases
#[inline]
fn do_stuff() -> miette::Result<()> {
    let args = Args::parse();

    if let Some(subcmds) = args.subcommands
        && matches!(subcmds, Subcommands::Interactive { .. })
    {
        run_repl(&args).map_err(|e| miette::miette!("Repl exited unexpectedly. Reason: {}", e))?;
        todo!();
    }

    Ok(())
}

fn main() -> miette::Result<()> {
    let input = r#"
G := Lm.mx
k(Lx.G)
"#;

    let expressions = Lambda::parse(input);
    // let (assignments, statements) = Lambda::parse(input).unzip_expressions().unwrap();
    // let assignments = Lambda::generate_assignment_map(&assignments);

    kaleidoscope::interpreter::interpret(expressions.unwrap_expressions()?, &mut stdout());

    // dbg!(assignments, statements);

    Ok(())
}
