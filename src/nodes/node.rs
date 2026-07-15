use crate::nodes::{ParsingError, Span, abstraction::AbstractionNode, variable::VariableNode};

#[derive(Debug)]
pub enum Node {
    Variable(VariableNode),
    Abstraction(AbstractionNode)
}

impl Node {
    pub fn parse_string(s: &str, start: usize) -> Result<Self, ParsingError> {
        dbg!(s);
        todo!()
    }

    pub fn get_span(&self) -> Span {
        match self {
            Node::Variable(variable_node) => variable_node.span,
            Node::Abstraction(abstraction_node) => abstraction_node.span,
        }
    }
}
