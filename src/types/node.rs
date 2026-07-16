use std::fmt::Display;

use derive_more::IsVariant;

use crate::{
    VALID_LAMBDA_CHARACTERS, find_closing_delim,
    types::{
        ApplicationNode, CreatedAt, ParsingError, ReductionError, Span,
        abstraction::AbstractionNode, node::Node::Application, variable::VariableNode,
    },
};

#[derive(Debug, IsVariant, PartialEq, Clone)]
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
            Application(application_node) => application_node.to_string(),
        };

        write!(f, "{}", s)
    }
}

impl Node {
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

            if ab.span().len() < s.len() {
                let r = Node::parse_str(&s[ab.span().len()..], ab.span().end)?;
                let sp = start..r.span().end;
                let ap = ApplicationNode::new(Node::Abstraction(ab), r, sp);

                return Ok(Application(ap));
            }

            return Ok(Node::Abstraction(ab));
        }

        if let Ok(var) = VariableNode::parse_str(s, start) {
            if var.span().len() < s.len() {
                let r = Node::parse_str(&s[var.span().len()..], var.span().end)?;
                let sp = start..r.span().end;
                let ap = ApplicationNode::new(Node::Variable(var), r, sp);

                return Ok(Application(ap));
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
                if bound.is_none_or(|bound| bound == variable_node) {
                    return Ok(with);
                }

                Ok(self)
            }

            Node::Abstraction(abstraction_node) => abstraction_node.reduce(with, bound),
            Node::Application(application_node) => application_node.reduce(with, bound),
        }
    }
}
