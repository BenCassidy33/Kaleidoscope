use std::fmt::Display;

use getset::Getters;

use crate::{
    VALID_LAMBDA_CHARACTERS,
    nodes::{
        CreatedAt, ParsingError, Span, abstraction::AbstractionNode, node::Node,
        variable::VariableNode,
    },
};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct ApplicationNode {
    left: Box<Node>,
    right: Box<Node>,
    span: Span,
}

impl Display for ApplicationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.left, self.right)
    }
}

impl ApplicationNode {
    pub fn new<S: Into<Span>>(left: Node, right: Node, span: S) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            span: span.into(),
        }
    }

    pub fn parse_str(s: &str, start: usize) -> Result<Self, ParsingError> {
        let left: Node = if s.starts_with(VALID_LAMBDA_CHARACTERS) {
            Node::Abstraction(AbstractionNode::parse_str(s, start)?)
        } else {
            Node::Variable(VariableNode::parse_str(s, start)?)
        };

        if left.span().end == s.len() - 1 {
            return Err(ParsingError::new(
                s,
                Some("Attempt to parse application with only single node"),
                start..s.len(),
                Some(CreatedAt::new()),
            ));
        }

        let end = left.span().end;
        let right = Node::parse_str(&s[end..], end)?;
        let s = right.span().end;

        Ok(ApplicationNode::new(left, right, start..s))
    }

    pub fn find_mut<F: Fn(&Node) -> bool>(&mut self, f: F) -> Option<&mut Node> {
        if f(self.left.as_mut()) {
            return Some(self.left.as_mut());
        };

        if f(self.right.as_mut()) {
            return Some(self.right.as_mut());
        }

        None
    }
}
