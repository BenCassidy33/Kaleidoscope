use std::{collections::HashMap, fmt::Display};
use wasm_bindgen::prelude::*;

use derive_more::IsVariant;
use serde::Serialize;

use crate::{
    VALID_LAMBDA_CHARACTERS, repr_wasm,
    types::{
        ApplicationNode, CreatedAt, ParsingError, ReductionError, Span,
        abstraction::AbstractionNode, variable::VariableNode,
    },
    utils::find_closing_delim,
};

#[derive(Debug, IsVariant, PartialEq, Clone, Serialize)]
pub enum Node {
    Variable(VariableNode),
    Abstraction(AbstractionNode),
    Application(ApplicationNode),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Node::Variable(variable_node) => variable_node.to_string(),
            Node::Abstraction(abstraction_node) => abstraction_node.to_string(),
            Node::Application(application_node) => application_node.to_string(),
        };

        write!(f, "{}", s)
    }
}

impl Node {
    pub fn span(&self) -> &Span {
        match self {
            Node::Variable(variable_node) => variable_node.span(),
            Node::Abstraction(abstraction_node) => abstraction_node.span(),
            Node::Application(application) => application.span(),
        }
    }

    pub fn replace<F: Fn((&Node, Option<&VariableNode>)) -> bool>(
        self,
        f: &F,
        bound: Option<&VariableNode>,
        with: Node,
    ) -> Self {
        match self {
            Node::Abstraction(abstraction_node) => abstraction_node.replace(f, bound, with),
            Node::Application(application_node) => application_node.replace(f, bound, with),

            ref var => {
                if f((var, bound)) {
                    with
                } else {
                    self
                }
            }
        }
    }

    pub fn find_mut<F: Fn(&Node) -> bool>(&mut self, f: F) -> Option<&mut Node> {
        match self {
            Node::Abstraction(abstraction_node) => abstraction_node.find_mut(f),
            Node::Application(application_node) => application_node.find_mut(f),
            variable_node => {
                if f(variable_node) {
                    Some(variable_node)
                } else {
                    None
                }
            }
        }
    }

    pub fn find_all<F: Fn(&Node) -> bool>(&mut self, f: &F) -> Vec<&Node> {
        let mut all = Vec::new();
        self.find_mut_all_into(f, &mut all);
        all
    }

    fn find_mut_all_into<'a, F: Fn(&Node) -> bool>(&'a self, f: &F, all: &mut Vec<&'a Node>) {
        let matched = f(self);
        match self {
            Node::Abstraction(abstraction_node) => {
                // recurse into whatever child field holds the inner Node
                abstraction_node.body.find_mut_all_into(f, all);
            }

            Node::Application(application_node) => {
                application_node.left.find_mut_all_into(f, all);
                application_node.right.find_mut_all_into(f, all);
            }

            Node::Variable(_) => {}
        }

        if matched && !all.contains(&self) {
            all.push(self);
        }
    }

    pub fn reduce(self, with: Node, bound: Option<&VariableNode>) -> Result<Node, ReductionError> {
        match self {
            Node::Variable(ref variable_node) => {
                if bound.is_some_and(|bound| bound == variable_node) {
                    return Ok(with);
                }

                Ok(self)
            }

            Node::Abstraction(abstraction_node) => abstraction_node.reduce(with, bound),
            Node::Application(application_node) => application_node.reduce(with, bound),
        }
    }

    pub fn replace_assignments(self, assignments: &HashMap<VariableNode, Node>) -> Node {
        match self {
            Node::Variable(ref variable_node) => {
                if let Some(n) = assignments.get(variable_node) {
                    return n.to_owned();
                }

                self
            }

            Node::Abstraction(mut abstraction_node) => {
                abstraction_node.body =
                    Box::new(abstraction_node.body.replace_assignments(assignments));

                Node::Abstraction(abstraction_node)
            }

            Node::Application(mut application_node) => {
                application_node.left =
                    Box::new(application_node.left.replace_assignments(assignments));
                application_node.right =
                    Box::new(application_node.right.replace_assignments(assignments));

                Node::Application(application_node)
            }
        }
    }

    pub fn parse_str(mut s: &str, start: usize) -> Result<Self, ParsingError> {
        let mut offset = 0;
        while s.starts_with('(') {
            let range = find_closing_delim(s, ['('], ')').map_err(|_| {
                ParsingError::missing_closing_delimiter(s, '(', 0, Some(CreatedAt::new()))
            })?;

            if range.start == 0 && range.end == s.len() - 1 {
                s = &s[1..s.len() - 1];
                offset += 1;
            } else {
                let app_left = Node::parse_str(&s[range.start + 1..range.end], start)?;
                let app_right = Node::parse_str(&s[range.end + 1..], start + range.end + 1)?;
                let s = start..app_right.span().end;

                return Ok(Node::Application(ApplicationNode::new(
                    app_left, app_right, s,
                )));
            }
        }

        if s.starts_with(VALID_LAMBDA_CHARACTERS) {
            let ab = AbstractionNode::parse_str(s, start + offset)?;

            if ab.span().len() < s.len() && s[ab.span().len()..] != *")" {
                dbg!(&s[ab.span().len()..]);
                let r = Node::parse_str(&s[ab.span().len()..], ab.span().end)?;
                let sp = start..r.span().end;
                let ap = ApplicationNode::new(Node::Abstraction(ab), r, sp);

                return Ok(Node::Application(ap));
            }

            return Ok(Node::Abstraction(ab));
        }

        if let Ok(var) = VariableNode::parse_str(s, start) {
            if var.span().len() < s.len() {
                let r = Node::parse_str(&s[var.span().len()..], var.span().end)?;
                let sp = start..r.span().end;
                let ap = ApplicationNode::new(Node::Variable(var), r, sp);

                return Ok(Node::Application(ap));
            }

            return Ok(Node::Variable(var));
        }

        Err(ParsingError::new(
            s,
            Some("Invalid Input"),
            0..s.len(),
            Some(CreatedAt::new()),
        ))?
    }
}

impl From<WasmNode> for Node {
    fn from(value: WasmNode) -> Self {
        value.inner
    }
}

impl From<Node> for WasmNode {
    fn from(value: Node) -> Self {
        Self { inner: value }
    }
}

impl From<&Node> for WasmNode {
    fn from(value: &Node) -> Self {
        Self {
            inner: value.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct WasmNode {
    inner: Node,
}

impl Display for WasmNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

repr_wasm!(WasmNode);

#[wasm_bindgen]
impl WasmNode {
    #[wasm_bindgen(js_name = parseString)]
    pub fn parse_str(s: &str) -> Result<Self, ParsingError> {
        Node::parse_str(s, 0).map(|n| n.into())
    }

    pub fn reduce(self) -> Result<WasmNode, ReductionError> {
        match self.inner {
            Node::Application(application_node) => application_node.reduce_self().map(Into::into),
            _ => Ok(self),
        }
    }

    pub fn inner(&self) -> WasmNodeInner {
        self.inner.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, derive_more::Display)]
pub enum WasmNodeInnerKind {
    Variable,
    Abstraction,
    Application,
}

pub fn wasm_node_inner_kind_is_variable(inner: WasmNodeInnerKind) -> bool {
    matches!(inner, WasmNodeInnerKind::Variable)
}

pub fn wasm_node_inner_kind_is_abstraction(inner: WasmNodeInnerKind) -> bool {
    matches!(inner, WasmNodeInnerKind::Abstraction)
}

pub fn wasm_node_inner_kind_is_application(inner: WasmNodeInnerKind) -> bool {
    matches!(inner, WasmNodeInnerKind::Application)
}


#[wasm_bindgen(js_name = wasmNodeInnerKindToString)]
pub fn wasm_node_inner_kind_to_js_string(inner: WasmNodeInnerKind) -> String {
    format!("{}", inner)
}

// TODO: Make this an actual error type
#[wasm_bindgen(js_name = wasmNodeInnerKindtoJson)]
pub fn to_json(inner: WasmNodeInnerKind, pretty: bool) -> Option<String> {
    if pretty {
        return serde_json::to_string_pretty(&inner).ok();
    }

    serde_json::to_string(&inner).ok()
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize)]
pub struct WasmNodeInner {
    kind: WasmNodeInnerKind,
    variable: Option<VariableNode>,
    abstraction: Option<AbstractionNode>,
    application: Option<ApplicationNode>,
}

#[wasm_bindgen]
impl WasmNodeInner {
    pub fn kind(&self) -> WasmNodeInnerKind {
        self.kind.clone()
    }

    pub fn is_variable(&self) -> bool {
        self.variable.is_some()
    }

    pub fn is_abstraction(&self) -> bool {
        self.abstraction.is_some()
    }

    pub fn is_application(&self) -> bool {
        self.application.is_some()
    }

    pub fn variable(&self) -> VariableNode {
        self.variable.clone().unwrap()
    }

    pub fn abstraction(&self) -> AbstractionNode {
        self.abstraction.clone().unwrap()
    }

    pub fn application(&self) -> ApplicationNode {
        self.application.clone().unwrap()
    }
}

impl From<Node> for WasmNodeInner {
    fn from(value: Node) -> Self {
        match value {
            Node::Variable(variable_node) => Self {
                kind: WasmNodeInnerKind::Variable,
                variable: Some(variable_node),
                abstraction: None,
                application: None,
            },

            Node::Abstraction(abstraction_node) => Self {
                kind: WasmNodeInnerKind::Variable,
                variable: None,
                abstraction: Some(abstraction_node),
                application: None,
            },

            Node::Application(application_node) => Self {
                kind: WasmNodeInnerKind::Application,
                variable: None,
                abstraction: None,
                application: Some(application_node)
            },
        }
    }
}

