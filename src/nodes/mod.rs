use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

pub mod variable;

#[derive(Debug, Clone)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(value: std::ops::Range<usize>) -> Self {
        Span::new(value.start, value.end)
    }
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Span::new(value.0, value.1)
    }
}

#[derive(Clone, Error, Debug, Diagnostic)]
#[error("Parsing Error")]
pub struct ParsingError {
    #[source_code]
    src: String,
    msg: Option<String>,

    #[label("{msg:?}")]
    error_span: SourceSpan,
}

impl ParsingError {
    pub fn new<S: Into<String>, N: Into<SourceSpan>>(
        src: S,
        msg: Option<S>,
        error_span: N,
    ) -> ParsingError {
        ParsingError {
            src: src.into(),
            msg: msg.map(|f| f.into()),
            error_span: error_span.into(),
        }
    }
}
