use kaleidoscope::nodes::variable::VariableNode;

fn main() {
    dbg!(VariableNode::from_str("x_{y}", 0));
    dbg!(VariableNode::from_str("x_{abc}", 0));
    dbg!(VariableNode::from_str("x_a", 0));
    dbg!(VariableNode::from_str("x", 0));
}
