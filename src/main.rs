use clap::Parser;
use kaleidoscope::{
    args::{Args, Subcommands},
    repl::run_repl,
    types::node::Node,
};

// TODO: Inline this for actual releases
#[inline]
fn do_stuff() -> miette::Result<()> {
    let args = Args::parse();

    if let Some(subcmds) = args.subcommands
        && matches!(subcmds, Subcommands::Interactive { .. })
    {
        todo!();
    }

    run_repl(&args).map_err(|e| miette::miette!("Repl exited unexpectedly. Reason: {}", e))?;
    Ok(())
}

fn main() -> miette::Result<()> {
    let mut n = Node::parse_str("(Lm.m)y", 0)?;

    dbg!(n.to_string());
    Ok(())
}
