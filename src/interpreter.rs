use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{
    Lambda, LambdaKind,
    invocations::InvocationError,
    opts::{CreateDefaultOpts, DefaultOpts, GetDefaultOpt, Opts},
    types::{CreatedAt, Node, ReductionError},
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
            created_at: None,
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

// TODO: Standardize output format of results
pub fn interpret<L, O>(lambdas: L, stdout: &mut O) -> Result<(), IterpertingError>
where
    L: IntoIterator<Item = Lambda>,
    O: std::io::Write,
{
    let opts = Opts::create_default_options();
    let lambdas: Vec<Lambda> = lambdas.into_iter().collect();

    let assignment_expressions: Vec<Lambda> = lambdas
        .iter()
        .filter(|f| matches!(f.kind, LambdaKind::Assignment { .. }))
        .cloned()
        .collect();

    let assignments = Lambda::generate_assignment_map(&assignment_expressions);

    for statement in lambdas {
        let mut frames = Vec::new();
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
                    writeln!(stdout, "(Variable Assignment) {}\n", statement)?;
                }
            }

            LambdaKind::Statement { mut body } => {
                if opts
                    .get_default_opt(&DefaultOpts::ShouldCaptureAllChanges)
                    .get_current_as::<bool>()
                    .unwrap()
                {
                    let old = body.clone();
                    if let Some(ref assignments) = assignments {
                        body = body.replace_assignments(assignments);
                    }

                    writeln!(stdout, "(Variable Substitutions) {} => {}", old, body)?;
                } else {
                    if let Some(ref assignments) = assignments {
                        body = body.replace_assignments(assignments);
                    }
                }

                frames.push(body.clone());

                loop {
                    body = match body {
                        Node::Application(ap) => ap.reduce_self()?,
                        _ => body,
                    };

                    if frames[frames.len() - 1] == body {
                        break;
                    }

                    frames.push(body.clone());
                }

                if frames.len() == 1 {
                    writeln!(
                        stdout,
                        "(Expression Reduction)\n{} => {}",
                        frames[0], frames[0]
                    )?;
                } else if frames.len() == 2 {
                    writeln!(
                        stdout,
                        "(Expression Reduction) \t {} => {}",
                        frames[0], frames[1]
                    )?;
                } else {
                    writeln!(
                        stdout,
                        "(Expression Reduction)\n\t{}",
                        frames[0]
                    )?;

                    for frame in frames.iter().skip(1) {
                        writeln!(stdout, "\t=> {}", frame)?;
                    }

                    writeln!(stdout)?;
                }
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
