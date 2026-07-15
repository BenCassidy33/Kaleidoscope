use std::fmt::Display;

use derive_more::IsVariant;

use crate::{
    VALID_LAMBDA_CHARACTERS, find_closing_delim,
    nodes::{
        CreatedAt, ParsingError, Span, abstraction::AbstractionNode, application::ApplicationNode,
        node::Node::Application, variable::VariableNode,
    },
};

#[derive(Debug, IsVariant)]
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

        let n1 = if s.starts_with(VALID_LAMBDA_CHARACTERS) {
            Node::Abstraction(AbstractionNode::parse_str(s, start + offset)?)
        } else {
            if let Ok(var) = VariableNode::parse_str(s, start + offset)
                && var.span().end >= s.len()
            {
                Node::Variable(var)
            } else {
                Node::Application(ApplicationNode::parse_str(s, start + offset)?)
            }
        };

        // dbg!(&n1, s, start, n1.span().len());

        if n1.span().len() < s.len() {
            let right = Node::parse_str(&s[n1.span().len()..], start + n1.span().len())?;
            let range = n1.span().start..right.span().end;

            return Ok(Node::Application(ApplicationNode::new(n1, right, range)));
        }

        Ok(n1)
    }

    pub fn span(&self) -> &Span {
        match self {
            Node::Variable(variable_node) => variable_node.span(),
            Node::Abstraction(abstraction_node) => abstraction_node.span(),
            Node::Application(application) => application.span(),
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
}
