use crate::{
    find_closing_delim,
    nodes::{ParsingError, Span},
};

#[derive(Debug, Clone)]
pub struct VariableNode {
    base_ident: char,
    subscript: Option<String>,
    span: Span,
}

impl VariableNode {
    #[inline]
    pub fn new(base_ident: char, subscript: Option<String>, start: usize, had_curly: bool) -> Self {
        Self {
            base_ident,
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

    pub fn from_str(s: &str, start: usize) -> Result<Self, ParsingError> {
        match s.len() {
            0 => Err(ParsingError::new(
                s,
                Some("Expected identifier."),
                start..start + 1,
            ))?,

            1 => Ok(VariableNode::new(s.chars().next().unwrap(), None, start, false)),

            n => {
                let mut chars = s.chars().enumerate();
                let (_, base) = chars.next().unwrap();

                let (_, delim) = chars.next().ok_or_else(|| {
                    ParsingError::new(
                        s,
                        Some("Expected Variable Subscript Delimiter, found end of expression."),
                        start..n,
                    )
                })?;

                if delim != '_' {
                    return Err(ParsingError::new(
                        s,
                        Some(&format!(
                            "Expected Variable Subscript Delimiter, found {}.",
                            delim
                        )),
                        start..n,
                    ));
                }

                if let Some((idx, next)) = chars.next() {
                    if next == '{' {
                        let range = find_closing_delim(&s[idx..], ['{'], '}').map_err(|_| {
                            ParsingError::new(
                                s,
                                Some("Closing '}' not found in variable decleration"),
                                start..n,
                            )
                        })?;

                        let subscript = &s[idx + range.start + 1..idx + range.clone().end];

                        return Ok(VariableNode::new(base, Some(subscript.to_string()), start, true));
                    }

                    Ok(VariableNode::new(base, Some(next.to_string()), start, false))
                } else {
                    Err(ParsingError::new(
                        "Expected '{' or identifier after '_' found EOL.",
                        None,
                        start..n,
                    ))
                }
            }
        }
    }
}
