use std::{fmt::Display, hash::Hash};
use wasm_bindgen::prelude::*;

use getset::Getters;
use serde::Serialize;

use crate::{
    VALID_LAMBDA_CHARACTERS, repr_wasm,
    types::{CreatedAt, Node, ParsingError, Span},
    utils::find_closing_delim,
};

#[wasm_bindgen]
#[derive(Debug, Clone, Getters, Eq, Serialize)]
#[getset(get = "pub")]
pub struct VariableNode {
    pub(crate) ident: String,
    pub(crate) subscript: Option<String>,
    pub(crate) span: Span,
    pub(crate) is_stdlib: bool,
}

repr_wasm!(VariableNode);

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

            None => self.ident.to_string(),
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

            None => self.ident.to_string(),
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

#[wasm_bindgen]
impl VariableNode {
    pub fn new(ident: String, subscript: Option<String>, start: usize, had_curly: bool) -> Self {
        Self {
            ident: ident.to_string(),
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
            is_stdlib: false,
        }
    }

    #[wasm_bindgen(getter, js_name = ident)]
    pub fn get_ident(&self) -> String {
        self.ident.clone()
    }

    #[wasm_bindgen(getter, js_name = subscript)]
    pub fn get_subscript(&self) -> Option<String> {
        self.subscript.clone()
    }

    pub fn parse_str(s: &str, start: usize) -> Result<Self, ParsingError> {
        if s.starts_with('_')
            || s.starts_with(VALID_LAMBDA_CHARACTERS) && (s.len() == 1 || !s.contains('_'))
        {
            Err(ParsingError::new(
                s,
                Some(
                    "Variable contains a lambda character. Try using a substring such as L_1 to differentiate the name.",
                ),
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

                if !ch.is_alphanumeric() {
                    Err(ParsingError::new(
                        s,
                        Some("Invalid Variable Identifier"),
                        0..s.len(),
                        Some(CreatedAt::new()),
                    ))?
                }

                Ok(VariableNode::new(s.to_string(), None, start, false))
            }

            n => {
                let mut chars = s.chars().enumerate().peekable();
                let (_, base_c) = chars.next().unwrap();
                let mut base = base_c.to_string();

                if base_c.is_uppercase() {
                    while let Some((_, c)) = chars.peek() {
                        if c.is_uppercase() && *c != '_' {
                            let (_, c) = chars.next().unwrap();
                            base.push(c);
                        } else {
                            break;
                        }
                    }
                }

                let Some((_, delim)) = chars.next() else {
                    return Ok(VariableNode::new(base, None, start, false));
                };

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
}
