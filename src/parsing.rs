use serde::Serialize;

use crate::{
    invocations::{self, Invocation},
    types::{Node, ParsingError, VariableNode},
};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Lambda {
    pub(crate) kind: LambdaKind,
    pub(crate) invocations: Option<Vec<Invocation>>,
}

#[derive(Debug, Serialize)]
pub enum LambdaKind {
    Assignment { ident: VariableNode, body: Node },
    Statement { body: Node },
    StandaloneInvocation,
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

        raw_exprs.into_iter().map(|mut expr| {
            let (invocations, e) = Invocation::parse(&expr, 0)?;
            expr = e.to_string();

            let invocations = if !invocations.is_empty() {
                Some(invocations)
            } else {
                None
            };

            if let Some(idx) = expr.find(":=") {
                let (raw_ident, raw_body) = expr.split_once(":=").unwrap();
                let ident = VariableNode::parse_str(
                    &raw_ident.replace(|c: char| c.is_ascii_whitespace(), ""),
                    0,
                )?;

                let body = Node::parse_str(
                    &raw_body.replace(|c: char| c.is_ascii_whitespace(), ""),
                    idx + 2,
                )?;

                Ok(Lambda {
                    kind: LambdaKind::Assignment { ident, body },
                    invocations,
                })
            } else {
                if expr.is_empty() && invocations.is_some() {
                    return Ok(Lambda {
                        kind: LambdaKind::StandaloneInvocation,
                        invocations,
                    });
                }

                Ok(Lambda {
                    kind: Node::parse_str(&expr, 0).map(|e| LambdaKind::Statement { body: e })?,
                    invocations,
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
            dbg!(expression);
            if let LambdaKind::Assignment { ident, body } = &expression.kind {
                map.insert(ident.clone(), body.clone());
            }
        }

        if map.is_empty() {
            return None;
        }

        Some(map)
    }
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

pub trait UnzipExpressions
where
    Self: Iterator,
{
    /// Unzips expression stream into vectors of assignments and statements
    fn unzip_expressions(self) -> Result<(Vec<Lambda>, Vec<Lambda>), ParsingError>;
}

impl<T> UnzipExpressions for T
where
    T: Iterator<Item = Result<Lambda, ParsingError>>,
{
    fn unzip_expressions(self) -> Result<(Vec<Lambda>, Vec<Lambda>), ParsingError> {
        Ok(self
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .partition(|l| matches!(l.kind, LambdaKind::Assignment { .. })))
    }
}
