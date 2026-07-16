#![allow(dead_code)]

use std::collections::HashMap;

use crate::types::{Node, ParsingError, VariableNode};

pub mod args;
pub mod repl;
pub mod types;

pub const LAMBDA_CHAR: char = 'λ';
pub const VALID_LAMBDA_CHARACTERS: [char; 2] = ['L', LAMBDA_CHAR];
pub const EXTENDED_SYNTAX: bool = true;

#[derive(Debug)]
pub enum Lambda {
    Assignment { ident: VariableNode, body: Node },
    Statement { body: Node },
}

pub fn parse<I>(input: I) -> impl Iterator<Item = Result<Lambda, ParsingError>>
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
                &raw_ident.replace(|c: char| c.is_ascii_whitespace(), ""),
                0,
            )?;
            let body = Node::parse_str(
                &raw_body.replace(|c: char| c.is_ascii_whitespace(), ""),
                idx + 2,
            )?;

            Ok(Lambda::Assignment { ident, body })
        } else {
            Node::parse_str(&expr, 0).map(|e| Lambda::Statement { body: e })
        }
    })
}

pub fn find_closing_delim<const N: usize>(
    input: &str,
    opening: [char; N],
    closing: char,
) -> Result<std::ops::Range<usize>, isize> {
    let mut count = 0isize;
    let mut first = -1isize;

    for (i, c) in input.chars().enumerate() {
        if opening.contains(&c) {
            if first == -1 {
                first = i as isize;
            }
            count += 1;
            continue;
        }

        if c == closing {
            count -= 1;
        }

        if count == 0 {
            if first != -1 {
                return Ok(first as usize..i);
            }

            return Err(count);
        }
    }

    Err(count)
}

pub fn generate_assignment_map<'a, E>(expressions: E) -> Option<HashMap<VariableNode, Node>>
where
    E: IntoIterator<Item = &'a Lambda>,
{
    let mut map = HashMap::new();

    for expression in expressions {
        if let Lambda::Assignment { ident, body } = expression {
            map.insert(ident.clone(), body.clone());
        }
    }

    if map.is_empty() {
        return None;
    }

    Some(map)
}
