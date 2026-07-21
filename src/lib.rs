pub mod args;
pub mod interpreter;
pub mod opts;
pub mod parsing;
pub mod repl;
pub mod types;
pub mod utils;

use miette::{Diagnostic, SourceSpan};
pub use parsing::*;
use serde::Serialize;
use thiserror::Error;

use crate::{interpreter::InterpretingError, types::{CreatedAt, ParsingError}};

pub const LAMBDA_CHAR: char = 'λ';
pub const VALID_LAMBDA_CHARACTERS: [char; 2] = ['L', LAMBDA_CHAR];
pub const EXTENDED_SYNTAX: bool = true;
pub const MAIN_FILE_PATH: &str = "/src/main.lmda";

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone, Error, Debug, Diagnostic, Serialize)]
#[error("KalidoscopeError")]
pub struct KalidoscopeError {
    #[source_code]
    pub(crate) src: String,
    pub(crate) msg: Option<String>,

    #[label("{}", msg.as_deref().unwrap_or("here"))]
    #[serde(skip)]
    pub(crate) error_span: SourceSpan,
    pub(crate) created_at: Option<CreatedAt>,
}

impl KalidoscopeError {
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


    pub fn msg(msg: String) -> Self {
        Self {
            msg: Some(msg),
            error_span: SourceSpan::from(0..0),
            src: "".to_string(),
            created_at: None
        }
    }
}

#[macro_export]
macro_rules! kalidoscope_error {
    ($msg:literal) => {
        KalidoscopeError::msg(String::from($msg))
    };

    ($msg:literal, $($arg:tt)*) => {
        KalidoscopeError::msg(format!($msg, $($arg)*))
    };
}

macro_rules! impl_into_kalidoscope_error {
    ($ident:ident) => {
        impl From<$ident> for KalidoscopeError {
            fn from(value: $ident) -> Self {
                KalidoscopeError {
                    src: value.src,
                    msg: value.msg,
                    error_span: value.error_span,
                    created_at: value.created_at,
                }
            }
        }
    };
}

impl_into_kalidoscope_error!(ParsingError);
impl_into_kalidoscope_error!(InterpretingError);
