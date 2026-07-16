use miette::{Diagnostic, LabeledSpan, SourceSpan};
use thiserror::Error;

use crate::{
    Lambda, LambdaKind, invocations::InvocationError, opts::{CreateDefaultOpts, DefaultOpts, GetDefaultOpt, Opts}, types::{CreatedAt, Node, ReductionError},
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

impl From<std::io::Error> for IterpertingError {
    fn from(value: std::io::Error) -> Self {
        Self {
            src: "".to_string(),
            msg: Some(format!("std::io error! Error: {:?}", value)),
            error_span: (0..0).into(),
            created_at: None
        }
    }
}

impl From<InvocationError> for IterpertingError {
    fn from(val: InvocationError) -> Self {
        IterpertingError {
            src: val.src,
            msg: val.msg,
            error_span: val.error_span,
            created_at: val.created_at,
        }
    }
}

impl From<ReductionError> for IterpertingError {
    fn from(val: ReductionError) -> IterpertingError {
        IterpertingError {
            src: val.src,
            msg: val.msg,
            error_span: val.error_span,
            created_at: val.created_at,
        }
    }
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

pub fn interpret<L, O>(lambdas: L, stdout: &mut O) -> Result<(), IterpertingError>
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
                    stdout.write_all(format!("{}\n", statement).as_bytes())?;
                }
            }

            LambdaKind::Statement { mut body } => {
                if let Some(ref assignments) = assignments  {
                    body = body.replace_assignments(assignments);
                }

                if opts.get_default_opt(&DefaultOpts::ShouldCaptureAllChanges).get_current_as::<bool>().unwrap() {
                    stdout.write_all(format!("{}\n", body.clone()).as_bytes())?;
                }

                body = match body {
                    Node::Application(ap) => ap.reduce_self()?,
                    _ => body
                };

                stdout.write_all(format!("{}\n", body.clone()).as_bytes())?;
            }

            LambdaKind::StandaloneInvocation => {
                for invocation in statement
                    .invocations
                    .expect("Standalone Inovaction with no invocations!")
                {
                    invocation.invoke()?;
                    todo!("Invocation Error!");
                }

                todo!()
            }
        }
    }

    Ok(())
}
