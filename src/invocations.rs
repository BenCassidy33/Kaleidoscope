use enum_iterator::{Sequence, all};
use serde::Serialize;

use crate::{
    types::{CreatedAt, ParsingError, Span},
    utils::find_closing_delim,
};

#[derive(Debug, Serialize, Sequence)]
pub enum BuiltinInvocations {
    Print,
    Json,
}

impl TryFrom<&str> for BuiltinInvocations {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let upper = value.to_uppercase();

        all::<BuiltinInvocations>()
            .find(|builtin| format!("{:?}", builtin).to_uppercase() == upper)
            .ok_or(())
    }
}

#[derive(Debug, Serialize)]
pub enum InvocationKind {
    Builtin(BuiltinInvocations),
    Custom(String),
}

impl From<&str> for InvocationKind {
    fn from(value: &str) -> Self {
        if let Ok(builtin) = BuiltinInvocations::try_from(value) {
            return InvocationKind::Builtin(builtin);
        }

        InvocationKind::Custom(value.to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct Invocation {
    pub(crate) kind: InvocationKind,
    pub(crate) ident: String,
    pub(crate) args: Vec<String>,
    pub(crate) span: Span,
}

impl Invocation {
    pub fn parse(mut expr: &str, start: usize) -> Result<(Vec<Invocation>, &str), ParsingError> {
        let mut invocations = Vec::new();

        while let Some(delim_idx) = expr.find('!') {
            let invok_start_idx = expr[..delim_idx]
                .rfind(|c: char| c.is_ascii_whitespace() || c == ')')
                .unwrap_or(0);

            let mut rest = expr[delim_idx + 1..].trim().chars().peekable();

            if rest.peek().is_none_or(|c| *c != '(') {
                return Err(ParsingError::new(
                    expr,
                    Some("Unexpected end of invocation. Expected '('."),
                    0..expr.len(),
                    Some(CreatedAt::new()),
                ));
            }

            let rest: String = rest.collect();
            let arg_range = find_closing_delim(&rest, ['('], ')').map_err(|_| {
                ParsingError::missing_closing_delimiter(expr, '(', 0, Some(CreatedAt::new()))
            })?;

            let ident = &expr[invok_start_idx..delim_idx];
            let args: Vec<String> = rest[1..arg_range.end]
                .split(",")
                .map(|s| s.to_string())
                .collect();

            invocations.push(Invocation {
                kind: InvocationKind::from(ident),
                ident: ident.to_string(),
                args,
                span: (start..arg_range.end).into(),
            });

            expr = &expr[delim_idx + arg_range.end + 2..];
        }

        Ok((invocations, expr))
    }
}
