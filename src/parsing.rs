use derive_more::IsVariant;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    repr_wasm,
    types::{Node, ParsingError, VariableNode, WasmNode},
};
use std::{collections::HashMap, fmt::Display};

#[wasm_bindgen]
#[derive(Debug, Serialize, Clone, getset::Getters)]
pub struct Lambda {
    #[getset(get = "pub")]
    pub(crate) kind: LambdaKind,
}

repr_wasm!(Lambda);

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            LambdaKind::Assignment(LambdaAssignment {
                ref ident,
                ref body,
                ..
            }) => write!(f, "{} := {}", ident, body),
            LambdaKind::Statement(LambdaStatement { ref body }) => write!(f, "{}", body),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct LambdaAssignment {
    pub ident: VariableNode,
    pub body: Node,
}

#[derive(Debug, Serialize, Clone)]
pub struct LambdaStatement {
    pub body: Node,
}

#[derive(Debug, Serialize, Clone, IsVariant)]
pub enum LambdaKind {
    Assignment(LambdaAssignment),
    Statement(LambdaStatement),
}

impl Display for LambdaKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambdaKind::Assignment(assignment) => {
                write!(f, "{} := {}", assignment.ident, assignment.body)
            }
            LambdaKind::Statement(statement) => write!(f, "{}", statement.body),
        }
    }
}

impl Lambda {
    pub fn parse<I>(input: I) -> impl Iterator<Item = Result<Self, ParsingError>>
    where
        I: Into<String>,
    {
        let input = input.into();
        let mut lines = input.lines().peekable();
        let mut raw_exprs = Vec::new();

        while let Some(line) = lines.next() {
            if line.starts_with("//") || line.is_empty() {
                continue;
            }

            let mut line = line.to_string();

            while let Some(ext) = lines.peek()
                && ext.starts_with(|c: char| c.is_ascii_whitespace())
            {
                line.push_str(lines.next().unwrap().trim());
            }

            raw_exprs.push(line);
        }

        raw_exprs.into_iter().map(|expr| {
            if let Some(idx) = expr.find(":=") {
                let (raw_ident, raw_body) = expr.split_once(":=").unwrap();
                let ident = VariableNode::parse_str(
                    raw_ident,
                    0,
                )?;

                let body = Node::parse_str(
                    raw_body,
                    idx + 2,
                )?;

                Ok(Lambda {
                    kind: LambdaKind::Assignment(LambdaAssignment { ident, body }),
                })
            } else {
                Ok(Lambda {
                    kind: Node::parse_str(&expr, 0)
                        .map(|e| LambdaKind::Statement(LambdaStatement { body: e }))?,
                })
            }
        })
    }

    pub fn generate_assignment_map<'a, E>(expressions: E) -> Option<HashMap<VariableNode, Node>>
    where
        E: IntoIterator<Item = &'a Self>,
    {
        let mut map = HashMap::new();

        for expression in expressions {
            if let LambdaKind::Assignment(LambdaAssignment { ident, body }) = &expression.kind {
                map.insert(ident.clone(), body.clone());
            }
        }

        if map.is_empty() {
            return None;
        }

        Some(map)
    }
}

#[wasm_bindgen]
impl Lambda {
    #[wasm_bindgen(js_name = parse)]
    pub fn parse_wasm(input: String) -> Result<Vec<Lambda>, ParsingError> {
        Lambda::parse(input)
            .unwrap_expressions()
            .map(|v| v.collect())
    }

    #[wasm_bindgen(getter, js_name = kind)]
    pub fn get_kind(&self) -> WasmLambdaKind {
        self.kind.clone().into()
    }

    // #[wasm_bindgen(getter, js_name = invocations)]
    // pub fn get_invocations(&self) -> Option<Vec<Invocation>> {
    //     self.invocations.clone()
    // }
}

pub trait UnwrapExpressions
where
    Self: Iterator,
{
    /// wraps a stream of `Result<Lambda, ParsingError>` into a stream of Lambda returning result
    /// `Result<Iterator, ParsingError>` where Iterator is a stream of lambda expressions
    fn unwrap_expressions(self) -> Result<impl Iterator<Item = Lambda>, ParsingError>;
}

impl<T> UnwrapExpressions for T
where
    T: Iterator<Item = Result<Lambda, ParsingError>>,
{
    fn unwrap_expressions(self) -> Result<impl Iterator<Item = Lambda>, ParsingError> {
        Ok(self.collect::<Result<Vec<_>, _>>()?.into_iter())
    }
}

pub struct Unzipped {
    pub assignments: Vec<LambdaAssignment>,
    pub statements: Vec<LambdaStatement>,
}

pub trait UnzipExpressions
where
    Self: Iterator,
{
    /// Unzips expression stream into vectors of assignments and statements
    fn unzip_expressions(self) -> Result<Unzipped, ParsingError>;
}

impl From<(Vec<Lambda>, Vec<Lambda>)> for Unzipped {
    fn from(value: (Vec<Lambda>, Vec<Lambda>)) -> Self {
        Self {
            assignments: value
                .0
                .iter()
                .map(|n| {
                    let LambdaKind::Assignment(ref assignment) = n.kind else {
                        unreachable!()
                    };
                    assignment.clone()
                })
                .collect(),
            statements: value
                .1
                .iter()
                .map(|n| {
                    let LambdaKind::Statement(ref statement) = n.kind else {
                        unreachable!()
                    };
                    statement.clone()
                })
                .collect(),
        }
    }
}

impl<T> UnzipExpressions for T
where
    T: Iterator<Item = Result<Lambda, ParsingError>>,
{
    fn unzip_expressions(self) -> Result<Unzipped, ParsingError> {
        Ok(self
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .partition(|l| matches!(l.kind, LambdaKind::Assignment { .. }))
            .into())
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Clone, derive_more::Display)]
#[display("{ident} := {body}")]
pub struct WasmAssignment {
    ident: VariableNode,
    body: Node,
}

repr_wasm!(WasmAssignment);

#[wasm_bindgen]
impl WasmAssignment {
    #[wasm_bindgen(getter)]
    pub fn ident(&self) -> VariableNode {
        self.ident.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> WasmNode {
        self.body.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Clone)]
pub struct WasmLambdaKind {
    kind: WasmLambdaKindInner,
    assignment: Option<WasmAssignment>,
    statement: Option<Node>,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Clone, Copy)]
pub enum WasmLambdaKindInner {
    Assignment,
    Statement,
    StandaloneInvocation,
}

impl From<LambdaKind> for WasmLambdaKind {
    fn from(val: LambdaKind) -> Self {
        match val {
            LambdaKind::Assignment(LambdaAssignment { ident, body, .. }) => WasmLambdaKind {
                kind: WasmLambdaKindInner::Assignment,
                assignment: Some(WasmAssignment { ident, body }),
                statement: None,
            },

            LambdaKind::Statement(LambdaStatement { body }) => WasmLambdaKind {
                kind: WasmLambdaKindInner::Statement,
                assignment: None,
                statement: Some(body),
            },
        }
    }
}

#[wasm_bindgen]
impl WasmLambdaKind {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_js_string(self) -> String {
        format!("{}", self)
    }

    // TODO: Make this an actual error type
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(self, pretty: bool) -> Option<String> {
        if pretty {
            return serde_json::to_string_pretty(&self).ok();
        }

        serde_json::to_string(&self).ok()
    }

    #[wasm_bindgen(js_name = isAssignment)]
    pub fn is_assignment(&self) -> bool {
        self.assignment.is_some()
    }

    #[wasm_bindgen(js_name = isStatement)]
    pub fn is_statement(&self) -> bool {
        self.statement.is_some()
    }

    #[wasm_bindgen(getter)]
    pub fn assignment(&self) -> Option<WasmAssignment> {
        self.assignment.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn statement(&self) -> Option<WasmNode> {
        self.statement.clone().map(Into::into)
    }
}

impl Display for WasmLambdaKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
