use crate::{
    VALID_LAMBDA_CHARACTERS, find_closing_delim,
    nodes::{ParsingError, Span, node::Node, variable::VariableNode},
};

#[derive(Debug)]
pub struct AbstractionNode {
    bound: Box<VariableNode>,
    body: Box<Node>,
    pub(crate) span: Span,
}

impl AbstractionNode {
    pub fn new<S: Into<Span>>(bound: VariableNode, body: Node, span: S) -> Self {
        Self {
            bound: Box::new(bound),
            body: Box::new(body),
            span: span.into(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &str, start: usize) -> Result<Self, ParsingError> {
        if !input.starts_with(VALID_LAMBDA_CHARACTERS) {
            return Err(ParsingError::new(
                input,
                Some("Missing a valid abstraction identifier!"),
                0..input.len(),
            ));
        }

        let mut chars = input.chars().enumerate().skip(1);

        let (i, bound): (Vec<usize>, Vec<char>) =
            chars.clone().take_while(|(_, c)| *c != '.').unzip();
        let bound = bound.iter().collect::<String>();
        if bound.is_empty() {
            return Err(ParsingError::new(
                input,
                Some("Expected abstraction body seperator '.', found EOL"),
                start..input.len(),
            ));
        }

        let bound = Node::parse_string(&bound, start + 1)?;

        let Node::Variable(bound) = bound else {
            return Err(ParsingError::new(
                input,
                Some("Bound of a abstraction must be a variable"),
                0..bound.get_span().end,
            ));
        };

        let (i, body_start) = chars.nth(i.len()).ok_or_else(|| {
            ParsingError::new(
                input,
                Some("Expected abstraction body, found EOL"),
                start..input.len(),
            )
        })?;

        let body = if body_start == '(' {
            let range = find_closing_delim(&input[i..], ['('], ')')
                .map_err(|_| ParsingError::missing_closing_delimiter(input, '(', i))?;

            Node::parse_string(&input[range.start + 1..range.end], start + i)?
        } else {
            Node::parse_string(&input[i..], start + i)?
        };

        Ok(AbstractionNode {
            span: (start..body.get_span().end).into(),
            bound: Box::new(bound),
            body: Box::new(body),
        })
    }
}
