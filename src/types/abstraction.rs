use std::fmt::Display;

use getset::Getters;

use crate::{
    LAMBDA_CHAR, VALID_LAMBDA_CHARACTERS, find_closing_delim,
    types::{CreatedAt, ParsingError, Span, node::Node, variable::VariableNode},
};

#[derive(Debug, Getters, PartialEq, Clone)]
#[getset(get = "pub")]
pub struct AbstractionNode {
    pub(crate) bound: Box<Node>,
    pub(crate) body: Box<Node>,
    pub(crate) span: Span,
}

impl Display for AbstractionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}.{}", LAMBDA_CHAR, self.bound, self.body)
    }
}

impl AbstractionNode {
    pub fn new<S: Into<Span>>(bound: VariableNode, body: Node, span: S) -> Self {
        Self {
            bound: Box::new(Node::Variable(bound)),
            body: Box::new(body),
            span: span.into(),
        }
    }

    pub fn parse_str(input: &str, start: usize) -> Result<Self, ParsingError> {
        if !input.starts_with(VALID_LAMBDA_CHARACTERS) {
            return Err(ParsingError::new(
                input,
                Some("Missing a valid abstraction identifier!"),
                0..input.len(),
                Some(CreatedAt::new()),
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
                Some(CreatedAt::new()),
            ));
        }

        let bound = Node::parse_str(&bound, start + 1)?;

        let Node::Variable(bound) = bound else {
            return Err(ParsingError::new(
                input,
                Some("Bound of a abstraction must be a variable"),
                0..bound.span().end,
                Some(CreatedAt::new()),
            ));
        };

        let (i, _) = chars.nth(i.len()).ok_or_else(|| {
            ParsingError::new(
                input,
                Some("Expected abstraction body, found EOL"),
                start..input.len(),
                Some(CreatedAt::new()),
            )
        })?;

        let body = Node::parse_str(&input[i + 1..], start + i + 1)?;

        Ok(AbstractionNode {
            span: (start..body.span().end).into(),
            bound: Box::new(Node::Variable(bound)),
            body: Box::new(body),
        })
    }

    pub fn find_mut<F: Fn(&Node) -> bool>(&mut self, f: F) -> Option<&mut Node> {
        if f(self.bound.as_mut()) {
            return Some(self.bound.as_mut());
        }

        if f(self.body.as_mut()) {
            return Some(self.body.as_mut());
        }

        None
    }

    pub fn replace<F: Fn((&Node, Option<&VariableNode>)) -> bool>(
        mut self,
        f: &F,
        bound: Option<&VariableNode>,
        with: Node,
    ) -> Node {
        if f((&Node::Abstraction(self.clone()), bound)) {
            return with;
        }

        // replace entire thing if body is just one variable and that is the variable that is being
        // looked for
        // if let Node::Variable(var) = self.body().as_ref()
        //     && f((self.body(), Some(var)))
        // {
        //     return with;
        // }

        if f((self.body(), bound)) {
            return match *self.body {
                Node::Variable(_) => with,
                _ => {
                    self.body = Box::new(with);
                    Node::Abstraction(self)
                }
            };
        } else {
            let Node::Variable(var) = self.bound.as_ref() else {
                panic!("Extend Syntax not supported yet!");
            };

            self.body = Box::new(self.body.replace(f, Some(var), with));
        }

        Node::Abstraction(self)
    }
}
