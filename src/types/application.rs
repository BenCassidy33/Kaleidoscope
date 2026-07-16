use std::fmt::Display;

use getset::Getters;
use serde::Serialize;

use crate::{
    VALID_LAMBDA_CHARACTERS,
    types::{
        CreatedAt, ParsingError, ReductionError, Span, abstraction::AbstractionNode, node::Node,
        variable::VariableNode,
    },
};

#[derive(Debug, Getters, PartialEq, Clone, Serialize)]
#[getset(get = "pub")]
pub struct ApplicationNode {
    pub(crate) left: Box<Node>,
    pub(crate) right: Box<Node>,
    pub(crate) span: Span,
}

impl From<ApplicationNode> for Node {
    fn from(val: ApplicationNode) -> Self {
        Node::Application(val)
    }
}

impl Display for ApplicationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})({})", self.left, self.right)
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
        if f(self.left.as_ref()) {
            return Some(self.left.as_mut());
        };

        if f(self.right.as_ref()) {
            return Some(self.right.as_mut());
        }

        None
    }

    // TODO: Should replace one at a time or both at once?
    // Both at once for now...
    pub fn replace<F: Fn((&Node, Option<&VariableNode>)) -> bool>(
        mut self,
        f: &F,
        bound: Option<&VariableNode>,
        with: Node,
    ) -> Node {
        if f((&Node::Application(self.clone()), bound)) {
            return with;
        }

        if f((self.left(), bound)) {
            self.left = Box::new(with.clone());
        } else {
            self.left = Box::new(self.left.replace(f, bound, with.clone()))
        }

        if f((self.right(), bound)) {
            self.right = Box::new(with);
        } else {
            self.right = Box::new(self.right.replace(f, bound, with.clone()));
        }

        Node::Application(self)
    }

    pub fn reduce(
        mut self,
        with: Node,
        bound: Option<&VariableNode>,
    ) -> Result<Node, ReductionError> {
        if let Some(bound) = bound {
            match *self.left {
                Node::Variable(ref variable_node) => {
                    if variable_node == bound {
                        self.left = Box::new(with.clone())
                    }
                }

                Node::Abstraction(abstraction_node) => {
                    self.left = Box::new(abstraction_node.reduce(with.clone(), Some(bound))?)
                }

                Node::Application(application_node) => {
                    self.left = Box::new(application_node.reduce(with.clone(), Some(bound))?)
                }
            };

            match *self.right {
                Node::Variable(ref variable_node) => {
                    if variable_node == bound {
                        self.right = Box::new(with)
                    }
                }

                Node::Abstraction(abstraction_node) => {
                    self.right = Box::new(abstraction_node.reduce(with, Some(bound))?)
                }

                Node::Application(application_node) => {
                    self.right = Box::new(application_node.reduce(with, Some(bound))?)
                }
            };

            return Ok(self.into());
        }

        self.left = Box::new(self.left.reduce(with.clone(), bound)?);
        self.right = Box::new(self.right.reduce(with, bound)?);

        Ok(self.into())
    }
}
