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
    { O_{mega} => UU },
    { Y => BU(CBU) },
    { T_{RUE} => Lx.Ly.x },
    { F_{ALSE} => Lx.Ly.y },

    { A_{ND} => Lp.Lq.((pq)p) },
    { O_R => Lp.Lq.ppq },
    { N_{OT} => Lp.p T_{RUE} F_{ALSE} },
    { I_{FTHENELSE} => Lp.La.Lb.(pab) },

    { S_{UCC} => Ln.Lf.Lx.(f((nf)x)) },
    { P_{LUS} => Lm.Ln.(mS_{UCC}n) },
    { S_{UB} => Lm.Ln.(n P_{RED} m) },
    { M_{ULT} => Lm.Ln.(m(P_{LUS}n) 0) },
    { P_{OW} => Lb.Ln.(nb) },

    { P_{AIR} => Lx.Ly.Lf.(f(xy)) },
    { F_{IRST} => Lp.(p(Lx.Ly.x)) },
    { S_{ECOND} => Lp.(p(Lx.Ly.y)) },

    { N_{IL} => Lf.T_{RUE} },
    { N_{ULL} => Lp.p(Lx.Ly.F_{ALSE}) },

    { I_{SZERO} => Ln.(n (Lx.F_{ALSE}) T_{RUE}) },
    { L_{EQ} => Lm.Ln.(I_{SZERO}(S_{UB}mn)) },

    { P_{REDICATE} => Ln.(n(Lg.Lk.I_{SZERO})(g1)k(P_{LUS}((gk)1))(Lv.0)0) }
}
