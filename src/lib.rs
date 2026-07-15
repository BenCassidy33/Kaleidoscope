#![allow(dead_code)]

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

pub fn parse<I: Into<String>>(input: I) -> Result<Lambda, ParsingError> {
    let input = input.into().replace([' ', '\t', '\n'], "");

    if let Some(idx) = input.find(":=") {
        let (raw_ident, raw_body) = input.split_once(":=").unwrap();
        let ident = VariableNode::parse_str(raw_ident, 0)?;
        let body = Node::parse_str(raw_body, idx + 2)?;

        return Ok(Lambda::Assignment { ident, body });
    }

    Ok(Lambda::Statement {
        body: Node::parse_str(&input, 0)?,
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
