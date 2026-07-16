use std::{fmt::Display, hash::Hash};

use getset::Getters;
use serde::Serialize;

use crate::{
    VALID_LAMBDA_CHARACTERS, types::{CreatedAt, Node, ParsingError, Span}, utils::find_closing_delim,
};

#[derive(Debug, Clone, Getters, Eq, Serialize)]
#[getset(get = "pub")]
pub struct VariableNode {
    pub(crate) ident: char,
    pub(crate) subscript: Option<String>,
    pub(crate) span: Span,
}

impl From<VariableNode> for Node {
    fn from(val: VariableNode) -> Self {
        Node::Variable(val)
    }
}

impl Hash for VariableNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.subscript.hash(state);
    }
}

impl PartialEq<str> for VariableNode {
    fn eq(&self, other: &str) -> bool {
        let s = match self.subscript {
            Some(ref sub) => {
                if sub.len() == 1 {
                    format!("{}_{}", self.ident, sub)
                } else {
                    format!("{}_{{{}}}", self.ident, sub)
                }
            }

            None => format!("{}", self.ident),
        };

        s == *other
    }
}

impl PartialEq<&str> for VariableNode {
    fn eq(&self, other: &&str) -> bool {
        let s = match self.subscript {
            Some(ref sub) => {
                if sub.len() == 1 {
                    format!("{}_{}", self.ident, sub)
                } else {
                    format!("{}_{{{}}}", self.ident, sub)
                }
            }

            None => format!("{}", self.ident),
        };

        s == *other
    }
}

impl PartialEq<VariableNode> for VariableNode {
    fn eq(&self, other: &VariableNode) -> bool {
        self.ident == other.ident && self.subscript == other.subscript
    }
}

impl Display for VariableNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.subscript {
            Some(ref sub) => {
                if sub.len() == 1 {
                    write!(f, "{}_{}", self.ident, sub)
                } else {
                    write!(f, "{}_{{{}}}", self.ident, sub)
                }
            }

            None => write!(f, "{}", self.ident),
        }
    }
}

impl VariableNode {
    #[inline]
    pub fn new(ident: char, subscript: Option<String>, start: usize, had_curly: bool) -> Self {
        Self {
            ident,
            span: match subscript {
                Some(ref sub) => {
                    if !had_curly {
                        Span::new(start, start + sub.len() + 2)
                    } else {
                        Span::new(start, start + sub.len() + 4)
                    }
                }
                None => Span::new(start, start + 1),
            },
            subscript,
        }
    }

    pub fn parse_str(s: &str, start: usize) -> Result<Self, ParsingError> {
        if s.starts_with(VALID_LAMBDA_CHARACTERS) {
            Err(ParsingError::new(
                s,
                Some("Variable contains a lambda character"),
                0..s.len(),
                Some(CreatedAt::new()),
            ))?;
        }

        match s.len() {
            0 => Err(ParsingError::new(
                s,
                Some("Expected identifier, found EOL."),
                start..start + 1,
                Some(CreatedAt::new()),
            ))?,

            1 => {
                let ch = s.chars().next().unwrap();

                if !ch.is_alphabetic() {
                    Err(ParsingError::new(
                        s,
                        Some("Invalid Variable Identifier"),
                        0..s.len(),
                        Some(CreatedAt::new()),
                    ))?
                }

                Ok(VariableNode::new(
                    s.chars().next().unwrap(),
                    None,
                    start,
                    false,
                ))
            }

            n => {
                let mut chars = s.chars().enumerate();
                let (_, base) = chars.next().unwrap();

                if !base.is_alphabetic() {
                    Err(ParsingError::new(
                        s,
                        Some("Invalid Variable Identifier"),
                        0..s.len(),
                        Some(CreatedAt::new()),
                    ))?
                }

                let (_, delim) = chars.next().ok_or_else(|| {
                    ParsingError::new(
                        s,
                        Some("Expected Variable Subscript Delimiter, found end of expression."),
                        start..n,
                        Some(CreatedAt::new()),
                    )
                })?;

                if delim != '_' {
                    return Ok(VariableNode::new(base, None, start, false));
                }

                if let Some((idx, next)) = chars.next() {
                    if !next.is_alphanumeric() && !['{', '}'].contains(&next) {
                        Err(ParsingError::new(
                            s,
                            Some("Invalid Variable Identifier"),
                            0..s.len(),
                            Some(CreatedAt::new()),
                        ))?
                    }

                    if next == '{' {
                        let range = find_closing_delim(&s[idx..], ['{'], '}').map_err(|_| {
                            ParsingError::new(
                                s,
                                Some("Closing '}' not found in variable decleration"),
                                start..n,
                                Some(CreatedAt::new()),
                            )
                        })?;

                        let subscript = &s[idx + range.start + 1..idx + range.clone().end];

                        return Ok(VariableNode::new(
                            base,
                            Some(subscript.to_string()),
                            start,
                            true,
                        ));
                    }

                    Ok(VariableNode::new(
                        base,
                        Some(next.to_string()),
                        start,
                        false,
                    ))
                } else {
                    Err(ParsingError::new(
                        "Expected '{' or identifier after '_' found EOL.",
                        None,
                        start..n,
                        Some(CreatedAt::new()),
                    ))
                }
            }
        }
    }

    // pub fn reduce(self, with: Node) -> Result<Node, ReductionError> {
    // }
}
