use std::io::{empty};

use derive_more::derive;
use miette::{Diagnostic, SourceSpan};
use serde::Serialize;
use thiserror::Error;
use wasm_bindgen::prelude::*;

use crate::{
    Lambda, LambdaKind, LambdaStatement, UnwrapExpressions, opts::{CreateDefaultOpts, DefaultOpts, GetDefaultOpt, Opts}, repr_wasm, types::{CreatedAt, Node, ParsingError, ReductionError, WasmNode},
};

#[wasm_bindgen]
#[derive(Clone, Error, Debug, Diagnostic, Serialize)]
#[error("InterpretingError")]
pub struct InterpretingError {
    #[source_code]
    pub(crate) src: String,
    pub(crate) msg: Option<String>,

    #[serde(skip)]
    #[label("{}", msg.as_deref().unwrap_or("here"))]
    pub(crate) error_span: SourceSpan,
    pub(crate) created_at: Option<CreatedAt>,

    #[help]
    pub(crate) help: Option<String>
}

repr_wasm!(InterpretingError);

impl From<std::io::Error> for InterpretingError {
    fn from(value: std::io::Error) -> Self {
        Self {
            src: "".to_string(),
            msg: Some(format!("std::io error! Error: {:?}", value)),
            error_span: (0..0).into(),
            created_at: None,
            help: None
        }
    }
}

impl From<ReductionError> for InterpretingError {
    fn from(val: ReductionError) -> InterpretingError {
        InterpretingError {
            src: val.src,
            msg: val.msg,
            error_span: val.error_span,
            created_at: val.created_at,
            help: None
        }
    }
}

impl From<ParsingError> for InterpretingError {
    fn from(value: ParsingError) -> Self {
        Self {
            src: value.src,
            msg: value.msg,
            error_span: value.error_span,
            created_at: value.created_at,
            help: None
        }
    }
}

impl InterpretingError {
    pub fn new<S: Into<String>, N: Into<SourceSpan>>(
        src: S,
        msg: Option<S>,
        error_span: N,
        created_at: Option<CreatedAt>,
        help: Option<String>
    ) -> Self {
        Self {
            src: src.into(),
            msg: msg.map(|f| f.into()),
            error_span: error_span.into(),
            created_at,
            help
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, derive::Display)]
#[display("{:?}", self.0)]
pub struct WasmFrames(Vec<Vec<Node>>);

repr_wasm!(WasmFrames);

#[wasm_bindgen]
impl WasmFrames {
    #[wasm_bindgen(js_name = getFrames)]
    pub fn get_frames(&self) -> Vec<WasmNode> {
        self.0
            .iter()
            .flatten()
            .map(Into::<WasmNode>::into)
            .collect()
    }

    #[wasm_bindgen(js_name = getFrameLengths)]
    pub fn get_frames_lengths(&self) -> Vec<usize> {
        self.0.iter().map(|f| f.len()).collect()
    }
}

#[wasm_bindgen]
pub fn wasm_interpret_raw(expressions: String) -> Result<WasmFrames, InterpretingError> {
    let expressions: Vec<Lambda> = Lambda::parse(expressions)
        .unwrap_expressions()
        .map_err(Into::<InterpretingError>::into)?
        .collect();

    Ok(WasmFrames(interpret(expressions, &mut empty(), None)?))
}

// TODO: Standardize output format of results
pub fn interpret<L, O>(
    lambdas: L,
    stdout: &mut O,
    opts: Option<&mut Opts>,
) -> Result<Vec<Vec<Node>>, InterpretingError>
where
    L: IntoIterator<Item = Lambda>,
    O: std::io::Write,
{
    let opts = if let Some(opts) = opts {
        opts
    } else {
        &mut Opts::create_default_options()
    };

    let lambdas: Vec<Lambda> = lambdas.into_iter().collect();

    let mut expression_frames = Vec::new();

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
                if opts
                    .get_default_opt(&DefaultOpts::ShouldPrintEveryLine)
                    .get_current_as::<bool>()
                    .unwrap()
                {
                    writeln!(stdout, "(Variable Assignment) {}\n", statement)?;
                }
            }

            LambdaKind::Statement(LambdaStatement { mut body }) => {
                if opts
                    .get_default_opt(&DefaultOpts::ShouldCaptureAllChanges)
                    .get_current_as::<bool>()
                    .unwrap()
                {
                    let old = body.clone();
                    if let Some(ref assignments) = assignments {
                        body = body.replace_assignments(assignments)?;
                    }

                    writeln!(stdout, "(Variable Substitutions) {} => {}", old, body)?;
                } else {
                    if let Some(ref assignments) = assignments {
                        body = body.replace_assignments(assignments)?;
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
                    writeln!(stdout, "(Expression Reduction)\n\t{}", frames[0])?;

                    for frame in frames.iter().skip(1) {
                        writeln!(stdout, "\t=> {}", frame)?;
                    }

                    writeln!(stdout)?;
                }

                expression_frames.push(frames);
            }
        }
    }

    Ok(expression_frames)
}
