use std::collections::HashMap;

use crate::{
    Lambda, LambdaAssignment, LambdaStatement, UnzipExpressions,
    types::{Node, VariableNode},
};

macro_rules! stdlib_assignments {
    ($({$ident:expr => $($value:tt)+}),* $(,)?) => {
        pub fn stdlib_assignments() -> HashMap<VariableNode, Node> {
            let mut stdlib = HashMap::new();
            $(
                let LambdaAssignment { mut ident, body } =
                Lambda::parse(format!("{} := {}", stringify!($ident), stringify!($($value)*)))
                    .unzip_expressions()
                    .unwrap().assignments[0].clone();

                ident.is_stdlib = true;
                let body = body.replace_assignments(&stdlib).unwrap();

                if let Some(item) = stdlib.insert(ident, body) {
                    panic!("Stdlib cannot override previous assignments. Attempt to reassign item: {}", item);
                };
            )*

            return stdlib;
        }
    };
}

pub fn generate_lambda_number(number: u32) -> Node {
    let mut n = String::from("Lf.Lx.");
    if number == 0 {
        n.push('x');
    } else {
        n.push_str(&"(f".repeat(number as usize));
        n.push('x');
        n.push_str(&")".repeat(number as usize));
    }

    let LambdaStatement { ref body } = Lambda::parse(n).unzip_expressions().unwrap().statements[0];
    body.to_owned()
}

//TODO: Check accuracy of these
// https://en.wikipedia.org/wiki/Lambda_calculus#Logic_and_predicates
stdlib_assignments! {
    { 0 => Lf.Lx.x },
    { I => Lx.x },
    { S => Lx.Ly.Lz.xyz },
    { K => Lx.Ly.x },
    { B => Lx.Ly.Lz.xyz },
    { C => Lx.Ly.Lz.xzy },
    { W => Lx.Ly.xyy },
    { U => (Lx.x)x },
    { OMEGA => U U },
    { Y => BU(C(B(U))) },

    { TRUE => Lx.Ly.x },
    { FALSE => Lx.Ly.y },

    { AND => Lp.Lq.((pq)p) },
    { OR => Lp.Lq.ppq },
    { NOT => Lp.p(TRUE FALSE) },
    { IFTHENELSE => Lp.La.Lb.(pab) },

    { SUCC => Ln.Lf.Lx.(f((nf)x)) },
    { PLUS => Lm.Ln.(m SUCC n) },
    { SUB => Lm.Ln.(n PRED m) },
    { MULT => Lm.Ln.((m PLUS n) 0) },
    { POW => Lb.Ln.(nb) },

    { PAIR => Lx.Ly.Lf.(f(xy)) },
    { FIRST => Lp.(p(Lx.Ly.x)) },
    { SECOND => Lp.(p(Lx.Ly.y)) },

    { NIL => Lf.TRUE },
    { NULL => Lp.p(Lx.Ly.FALSE) },

    { ISZERO => Ln.(n (Lx.FALSE) TRUE) },
    { LEQ => Lm.Ln.(ISZERO(SUB mn)) },

    { PREDICATE => Ln.(n(Lg.Lk.ISZERO)(g1)k(PLUS((gk)1))(Lv.0)0) }
}
