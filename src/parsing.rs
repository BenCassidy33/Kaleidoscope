use std::collections::HashMap;
use crate::{Lambda, types::{Node, ParsingError, VariableNode}};

pub fn parse<I>(input: I) -> impl Iterator<Item = Result<Lambda, ParsingError>>
where
    I: Into<String>,
{
    let input = input.into();

    let mut lines = input.lines().peekable();
    let mut raw_exprs = Vec::new();

    while let Some(line) = lines.next() {
        if line.starts_with("//") || line.is_empty() {
            continue;
        }

        let mut line = line.to_string();

        while let Some(ext) = lines.peek()
            && ext.starts_with(|c: char| c.is_ascii_whitespace())
        {
            line.push_str(lines.next().unwrap().trim());
        }

        raw_exprs.push(line);
    }

    raw_exprs.into_iter().map(|expr| {
        if let Some(idx) = expr.find(":=") {
            let (raw_ident, raw_body) = expr.split_once(":=").unwrap();
            let ident = VariableNode::parse_str(
                &raw_ident.replace(|c: char| c.is_ascii_whitespace(), ""),
                0,
            )?;
            let body = Node::parse_str(
                &raw_body.replace(|c: char| c.is_ascii_whitespace(), ""),
                idx + 2,
            )?;

            Ok(Lambda::Assignment { ident, body })
        } else {
            Node::parse_str(&expr, 0).map(|e| Lambda::Statement { body: e })
        }
    })
}

pub fn generate_assignment_map<'a, E>(expressions: E) -> Option<HashMap<VariableNode, Node>>
where
    E: IntoIterator<Item = &'a Lambda>,
{
    let mut map = HashMap::new();

    for expression in expressions {
        if let Lambda::Assignment { ident, body } = expression {
            map.insert(ident.clone(), body.clone());
        }
    }

    if map.is_empty() {
        return None;
    }

    Some(map)
}
