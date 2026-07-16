use crate::types::{Node, VariableNode};

pub mod args;
pub mod repl;
pub mod types;
pub mod parsing;
pub mod utils;

pub use parsing::*;

pub const LAMBDA_CHAR: char = 'λ';
pub const VALID_LAMBDA_CHARACTERS: [char; 2] = ['L', LAMBDA_CHAR];
pub const EXTENDED_SYNTAX: bool = true;

#[derive(Debug)]
pub enum Lambda {
    Assignment { ident: VariableNode, body: Node },
    Statement { body: Node },
}

