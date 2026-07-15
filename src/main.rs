use kaleidoscope::nodes::node::Node;

fn main() -> miette::Result<()> {
    let mut n = Node::parse_str("w_1x_2z_3", 0)?;

    let r = n.find_mut(|node| {
        dbg!(node);
        match node {
            Node::Variable(variable_node) => *variable_node.ident() == 'w',
            _ => false
        }
    });

    dbg!(r);

    Ok(())
}
