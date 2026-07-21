use getset::Getters;
use serde::Serialize;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

use crate::{
    VALID_LAMBDA_CHARACTERS, repr_wasm, types::{
        CreatedAt, Node::Application, ParsingError, ReductionError, Span, WasmNode, abstraction::AbstractionNode, node::Node, variable::VariableNode,
    },
};

#[wasm_bindgen]
#[derive(Debug, Getters, PartialEq, Clone, Serialize)]
#[getset(get = "pub")]
pub struct ApplicationNode {
    pub(crate) left: Box<Node>,
    pub(crate) right: Box<Node>,
    pub(crate) span: Span,
}

repr_wasm!(ApplicationNode);

impl From<ApplicationNode> for Node {
    fn from(val: ApplicationNode) -> Self {
        Node::Application(val)
    }
}

impl Display for ApplicationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Node::Variable(_), Node::Variable(_)) => write!(f, "({})({})", self.left, self.right),
            (_, Node::Variable(_)) => write!(f, "({}){}", self.left, self.right),
            (Node::Variable(_), _) => write!(f, "{}({})", self.left, self.right),

            (Node::Application(_), Node::Application(_))
            | (Node::Abstraction(_), Node::Abstraction(_)) => {
                write!(f, "({})({})", self.left, self.right)
            }

            _ => write!(f, "({}{})", self.left, self.right),
        }
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

    pub fn find_mut<F: Fn(&Node) -> bool>(&mut self, f: F) -> Option<&mut Node> {
        if f(self.left.as_ref()) {
            return Some(self.left.as_mut());
        };

        if f(self.right.as_ref()) {
            return Some(self.right.as_mut());
        }

        None
    }

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
        self,
        with: Node,
        bound: Option<&VariableNode>,
    ) -> Result<Node, ReductionError> {
        let Some(bound) = bound else {
            return Ok(self.into());
        };

        Ok(Node::Application(self).substitute(bound, &with))

    }

    pub fn reduce_self(self) -> Result<Node, ReductionError> {
        match *self.left {
            Node::Abstraction(ab) => {
                let Node::Variable(ref bound_var) = *ab.bound else {
                    unreachable!()
                };

                Ok(ab.body.substitute(bound_var, &self.right))
            }

            Node::Application(ap) => {
                let reduced_left = ap.clone().reduce_self()?;
                if reduced_left == Node::Application(ap) {
                    let reduced_right = self.right.reduce_self()?;
                    Ok(ApplicationNode::new(reduced_left, reduced_right, self.span).into())
                } else {
                    Ok(ApplicationNode::new(reduced_left, *self.right, self.span).into())
                }
            }

            left => {
                let reduced_right = self.right.reduce_self()?;
                Ok(ApplicationNode::new(left, reduced_right, self.span).into())
            }
        }
    }
}

#[wasm_bindgen]
impl ApplicationNode {
    #[wasm_bindgen(js_name = parseString)]
    pub fn parse_str(s: &str, start: usize) -> Result<ApplicationNode, ParsingError> {
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

    #[wasm_bindgen(js_name = reduce)]
    pub fn reduce_wasm(
        self,
        with: WasmNode,
        bound: Option<VariableNode>,
    ) -> Result<WasmNode, ReductionError> {
        self.reduce(with.into(), bound.as_ref()).map(Into::into)
    }

    #[wasm_bindgen(getter, js_name = left)]
    pub fn get_left(&self) -> WasmNode {
        (*self.left.clone()).into()
    }

    #[wasm_bindgen(getter, js_name = right)]
    pub fn get_right(&self) -> WasmNode {
        (*self.right.clone()).into()
    }
}
