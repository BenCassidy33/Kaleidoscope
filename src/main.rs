use std::collections::HashMap;

use clap::Parser;
use kaleidoscope::{
    Lambda,
    args::{Args, Subcommands},
    generate_assignment_map,
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
    let input = r#"
G := Lm.mX
H := (Lx.G)Y
"#;

    let expressions = kaleidoscope::parse(input)
        .collect::<Result<Vec<_>, _>>()?;

    let assignments = kaleidoscope::generate_assignment_map(&expressions);

    Ok(())
}
