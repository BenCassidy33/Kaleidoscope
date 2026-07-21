use std::collections::HashMap;

use crate::{
    Lambda, LambdaKind, UnzipExpressions,
    types::{Node, VariableNode},
};

macro_rules! stdlib_assignments {
    ($({$ident:expr => $($value:tt)+}),* $(,)?) => {
        pub fn stdlib_assignments() -> HashMap<VariableNode, Node> {
            let mut stdlib = HashMap::new();
            $(
                let LambdaKind::Assignment { mut ident, body } =
                Lambda::parse(format!("{} := {}", stringify!($ident), stringify!($($value)*)))
                    .unzip_expressions()
                    .unwrap().0[0]
                    .clone().kind
                else {
                    panic!("Invalid stdlib_assignment.");
                };

                ident.is_stdlib = true;
                let body = body.replace_assignments(&stdlib);

                if let Some(item) = stdlib.insert(ident, body) {
                    panic!("Stdlib cannot override previous assignments. Attempt to reassign item: {}", item);
                };
            )*

            return stdlib;
        }
    };
}

//TODO: Check accuracy of these
// https://en.wikipedia.org/wiki/Lambda_calculus#Logic_and_predicates
stdlib_assignments! {
    { I => Lx.x },
    { S => Lx.Ly.Lz.xyz },
    { K => Lx.Ly.x },
    { B => Lx.Ly.Lz.xyz },
    { C => Lx.Ly.Lz.xzy },
    { W => Lx.Ly.xyy },
    { U => (Lx.x)x },
    { O_{mega} => UU },
    { Y => BU(CBU) },
    { T_{RUE} => Lx.Ly.x },
    { F_{ALSE} => Lx.Ly.y },
    { A_{ND} => Lp.Lq.((pq)p) },
    { O_R => Lp.Lq.ppq },
    { N_{OT} => Lp.p T_{RUE} F_{ALSE} },
}
