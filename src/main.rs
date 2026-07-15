use kaleidoscope::nodes::node::Node;

fn main() -> miette::Result<()> {
    let mut n = Node::parse_str("w_1x_2z_3", 0)?;

    dbg!(n.find_mut(|node| match node {
        Node::Variable(variable_node) => variable_node == "w_1",
        _ => false,
    }));
    dbg!(n);

    Ok(())
}
