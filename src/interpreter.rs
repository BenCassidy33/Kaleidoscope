use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{Lambda, LambdaKind, types::CreatedAt};

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

pub fn interpret<L>(lambdas: L)
where
    L: Iterator<Item = Lambda>,
{
    let (assignments, statements): (Vec<Lambda>, Vec<Lambda>) =
        lambdas.partition(|f| matches!(f.kind, LambdaKind::Assignment { .. }));

    let assignments = Lambda::generate_assignment_map(&assignments);
}
