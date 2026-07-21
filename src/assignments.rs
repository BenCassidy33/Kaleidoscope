use std::collections::HashMap;
use crate::types::{Node, VariableNode};

pub struct Assignments {
    inner: HashMap<VariableNode, Assignment>,
}

struct Assignment {}

enum AssignmentKind {
    Internal { body: Node },
    External { defined_in: String },
}

struct ExternalAssignment {}
struct InternalAssignment {}
