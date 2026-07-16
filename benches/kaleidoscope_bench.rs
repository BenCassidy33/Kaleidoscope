use criterion::{Criterion, black_box, criterion_group, criterion_main};

macro_rules! benches {
    ($group:ident, $( $name:ident => $s:literal ),+ $(,)?) => {
        $(
            pub fn $name(c: &mut Criterion) {
                assert!(kaleidoscope::Lambda::parse($s).collect::<Result<Vec<_>, _>>().is_ok());
                c.bench_function(&format!("bench '{}'", $s), |b| {
                    b.iter(|| kaleidoscope::Lambda::parse(black_box($s)))
                });
            }
        )+

        criterion_group!($group, $( $name ),+);
    };
}

benches! {
    parsing_benchmarks,
    parse_single_var => "x",
    parse_single_var_simple_subscript => "x_1",
    parse_single_var_complex_subscript => "x_{bravery_sacrfice_death}",
    parse_identity_abstraction => "Lx.x",
    parse_identity_abstraction_with_complex_variables => "Lx_{bravery_sacrfice_death}.x_{bravery_sacrfice_death}",
    parse_simple_var_application => "xy",
    parse_complex_var_application => "x_1y_1",
    parse_extra_complex_var_application => "x_{honnor}y_{bravery}"
}

criterion_main!(parsing_benchmarks);
