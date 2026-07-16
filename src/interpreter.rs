use std::fmt::Write;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{
    Lambda, LambdaKind,
    opts::{CreateDefaultOpts, DefaultOpts, GetDefaultOpt, Opts},
    types::CreatedAt,
};

#[derive(Clone, Error, Debug, Diagnostic)]
#[error("Parsing Error")]
pub struct IterpertingError {
    #[source_code]
    src: String,
    msg: Option<String>,

    #[label("{msg:?}")]
    error_span: SourceSpan,

    created_at: Option<CreatedAt>,
}

impl IterpertingError {
    pub fn new<S: Into<String>, N: Into<SourceSpan>>(
        src: S,
        msg: Option<S>,
        error_span: N,
        created_at: Option<CreatedAt>,
    ) -> Self {
        Self {
            src: src.into(),
            msg: msg.map(|f| f.into()),
            error_span: error_span.into(),
            created_at,
        }
    }
}

pub fn interpret<L, O>(lambdas: L, out: &mut O)
where
    L: IntoIterator<Item = Lambda>,
    O: std::io::Write,
{
    let mut opts = Opts::create_default_options();
    let lambdas: Vec<Lambda> = lambdas.into_iter().collect();

    let assignment_expressions: Vec<Lambda> = lambdas
        .iter()
        .filter(|f| matches!(f.kind, LambdaKind::Assignment { .. }))
        .cloned()
        .collect();

    let assignments = Lambda::generate_assignment_map(&assignment_expressions);

    'outer: for statement in lambdas {
        match statement.kind {
            LambdaKind::Assignment { .. } => {
                if statement.invocations.is_some() {
                    todo!();
                }

                if opts
                    .get_default_opt(&DefaultOpts::ShouldPrintEveryLine)
                    .get_current_as::<bool>()
                    .unwrap()
                {
                    out.write_all(format!("{}", statement).as_bytes()).unwrap();
                }
            }

            LambdaKind::Statement { body } => {
                todo!("")
            }

            LambdaKind::StandaloneInvocation => {
                for invocation in statement
                    .invocations
                    .expect("Standalone Inovaction with no invocations!")
                {
                    invocation.invoke();
                    todo!("Invocation Error!");
                }

                todo!()
            }
        }
    }
}
