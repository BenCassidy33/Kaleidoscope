use miette::{Diagnostic, SourceSpan};
use serde::Serialize;
use std::panic::Location;
use thiserror::Error;
use wasm_bindgen::prelude::*;
use crate::repr_wasm;

pub mod abstraction;
pub mod application;
pub mod node;
pub mod variable;

pub use abstraction::*;
pub use application::*;
pub use node::*;
pub use variable::*;

use crate::opts::{DefaultOpts, GetDefaultOpt, Opts};

#[allow(dead_code)]
pub struct NodeFormattingOptions {
    extra_delimiters: bool,
}

impl From<Opts> for NodeFormattingOptions {
    fn from(value: Opts) -> Self {
        let extra_delimiters =
            value.get_default_opt_current::<bool>(&DefaultOpts::FormatWithExtraDelimiters);

        Self { extra_delimiters }
    }
}

impl NodeFormattingOptions {
    pub fn new(extra_delimiters: bool) -> Self {
        Self { extra_delimiters }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

#[allow(dead_code)]
#[derive(Debug, Clone, derive_more::Display, Serialize)]
#[display("{file:?} {line:?} {column:?}")]
pub struct CreatedAt {
    file: &'static str,
    line: u32,
    column: u32,
}

impl CreatedAt {
    #[inline]
    #[track_caller]
    pub fn new() -> Self {
        let loc = Location::caller();
        Self {
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
        }
    }
}

impl Default for CreatedAt {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
#[derive(Clone, Error, Debug, Diagnostic, Serialize)]
#[error("Parsing Error")]
pub struct ParsingError {
    #[source_code]
    pub(crate) src: String,
    pub(crate) msg: Option<String>,

    #[label("{}", msg.as_deref().unwrap_or("here"))]
    #[serde(skip)]
    pub(crate) error_span: SourceSpan,

    pub(crate) created_at: Option<CreatedAt>,
}

repr_wasm!(ParsingError);

impl ParsingError {
    pub fn new<S: Into<String>, N: Into<SourceSpan>>(
        src: S,
        msg: Option<S>,
        error_span: N,
        created_at: Option<CreatedAt>,
    ) -> ParsingError {
        ParsingError {
            src: src.into(),
            msg: msg.map(|f| f.into()),
            error_span: error_span.into(),
            created_at,
        }
    }

    pub fn missing_closing_delimiter<S: Into<String>>(
        src: S,
        open_delim: char,
        open_idx: usize,
        created_at: Option<CreatedAt>,
    ) -> Self {
        let s = src.into();
        ParsingError {
            error_span: (open_idx..s.len()).into(),
            src: s,
            msg: Some(format!("Missing closing delimiter for {open_delim}.")),
            created_at,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Error, Debug, Diagnostic, Serialize)]
#[error("Parsing Error")]
pub struct ReductionError {
    #[source_code]
    pub(crate) src: String,
    pub(crate) msg: Option<String>,

    #[label("{}", msg.as_deref().unwrap_or("here"))]
    #[serde(skip)]
    pub(crate) error_span: SourceSpan,

    pub(crate) created_at: Option<CreatedAt>,
}

repr_wasm!(ReductionError);

impl ReductionError {
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

#[macro_export]
macro_rules! repr_wasm {
    ($ident:ident) => {
        #[wasm_bindgen]
        impl $ident {
            #[wasm_bindgen(js_name = toString)]
            pub fn to_js_string(&self) -> String {
                format!("{}", self)
            }

            // TODO: Make this an actual error type
            #[wasm_bindgen(js_name = toJson)]
            pub fn to_json(&self, pretty: bool) -> Option<String> {
                if pretty {
                    return serde_json::to_string_pretty(self).ok();
                }

                serde_json::to_string(self).ok()
            }
        }
    };
}
