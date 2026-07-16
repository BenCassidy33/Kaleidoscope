use std::panic::Location;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

pub mod abstraction;
pub mod node;
pub mod variable;
pub mod application;

pub use abstraction::*;
pub use node::*;
pub use variable::*;
pub use application::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

#[derive(Debug, Clone)]
pub struct CreatedAt {
    file: &'static str,
    line: u32,
    column: u32
}

impl CreatedAt {
    #[inline]
    #[track_caller]
    pub fn new() -> Self {
        let loc = Location::caller();
        Self {
            file: loc.file(),
            line: loc.line(),
            column: loc.column()
        }
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

    created_at: Option<CreatedAt>
}

impl ParsingError {
    pub fn new<S: Into<String>, N: Into<SourceSpan>>(
        src: S,
        msg: Option<S>,
        error_span: N,
        created_at: Option<CreatedAt>
    ) -> ParsingError {
        ParsingError {
            src: src.into(),
            msg: msg.map(|f| f.into()),
            error_span: error_span.into(),
            created_at
        }
    }

    pub fn missing_closing_delimiter<S: Into<String>>(
        src: S,
        open_delim: char,
        open_idx: usize,
        created_at: Option<CreatedAt>
    ) -> Self {
        let s = src.into();
        ParsingError {
            error_span: (open_idx..s.len()).into(),
            src: s,
            msg: Some(format!("Missing closing delimiter for {open_delim}.")),
            created_at
        }
    }

}

#[derive(Clone, Error, Debug, Diagnostic)]
#[error("Parsing Error")]
pub struct ReductionError {
    #[source_code]
    src: String,
    msg: Option<String>,

    #[label("{msg:?}")]
    error_span: SourceSpan,

    created_at: Option<CreatedAt>
}

impl ReductionError {
    pub fn new<S: Into<String>, N: Into<SourceSpan>>(
        src: S,
        msg: Option<S>,
        error_span: N,
        created_at: Option<CreatedAt>
    ) -> Self {
        Self {
            src: src.into(),
            msg: msg.map(|f| f.into()),
            error_span: error_span.into(),
            created_at
        }
    }
}
