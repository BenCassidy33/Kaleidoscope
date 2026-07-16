pub mod args;
pub mod repl;
pub mod types;
pub mod parsing;
pub mod utils;
pub mod interperter;

pub use parsing::*;

pub const LAMBDA_CHAR: char = 'λ';
pub const VALID_LAMBDA_CHARACTERS: [char; 2] = ['L', LAMBDA_CHAR];
pub const EXTENDED_SYNTAX: bool = true;
