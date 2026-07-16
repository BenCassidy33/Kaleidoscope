use std::collections::HashMap;

use clap::Parser;
use kaleidoscope::{
    Lambda,
    args::{Args, Subcommands},
    repl::run_repl,
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
    // let mut assignments = HashMap::new();

    let input = r#"
G := Lm.m
    X
H := (Lx.G)Y
"#;

    kaleidoscope::parse(input).for_each(|f| {
        dbg!(f);
    });

    Ok(())
}
