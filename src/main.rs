use clap::Parser;
use kaleidoscope::{
    Lambda,
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
        run_repl(&args).map_err(|e| miette::miette!("Repl exited unexpectedly. Reason: {}", e))?;
        todo!();
    }

    Ok(())
}

fn main() -> miette::Result<()> {
    let Lambda::Statement { mut body } = kaleidoscope::parse("Lm.mx")? else {
        unreachable!()
    };

    dbg!(&body.to_string());

    body = body.replace(
        &|(node, bound)| match node {
            Node::Variable(var) => {
                if let Some(bound) = bound
                    && bound.ident() == var.ident()
                {
                    return true;
                }

                false
            }

            _ => false,
        },
        None,
        Node::parse_str("x", 0)?,
    );

    dbg!(&body.to_string());

    Ok(())
}
