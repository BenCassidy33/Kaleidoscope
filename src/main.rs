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

    let mut assignments = HashMap::new();
    let Lambda::Assignment { ident, body } = kaleidoscope::parse("G := Lm.m")? else {
        unimplemented!();
    };

    assignments.insert(ident, body);

    let Lambda::Statement { mut body } = kaleidoscope::parse("Lm.G")? else {
        unimplemented!();
    };

    dbg!(&body.to_string());
    body = body.replace_assignments(&assignments);
    dbg!(&body.to_string());

    Ok(())
}
