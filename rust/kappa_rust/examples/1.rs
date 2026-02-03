use kappa_rust::*;

fn main() {
    println!("0");

    let context = Context::default();
    let mixture = context
        .build_mixture()
        .add_molecula("N2", false, false)
        .add_atom("N")
        .build()
        .unwrap();

    let t = [2500.0, 5000.0, 7500.0, 10000.0];
    let p = [100000.0, 100000.0, 100000.0, 100000.0];
    let n = [
        [0.5, 0.5].as_slice(),
        [0.5, 0.5].as_slice(),
        [0.5, 0.5].as_slice(),
        [0.5, 0.5].as_slice(),
    ];

    let res = mixture
        .bozman_distribution(&t, &p, n.as_slice())
        .unwrap()
        .compute_transport_coefficient(None, None)
        .unwrap();

    let a = res.as_slice()[0];
    println!(
        "res {{{} {} {}}}",
        a.thermal_conductivity, a.shear_viscosity, a.bulk_viscosity
    );

    println!("res {:?}", res.as_slice());

    let res = mixture
        .bozman_distribution_with_closure_callback(1, || CalculateParams {
            #[warn(non_snake_case)]
            T: 10000.0,
            p: 100000.0,
            n: [0.5, 0.5].as_slice().as_ptr(),
        })
        .unwrap()
        .compute_transport_coefficient(None, None)
        .unwrap();
    let a = res.as_slice()[0];
    println!(
        "res {{{} {} {}}}",
        a.thermal_conductivity, a.shear_viscosity, a.bulk_viscosity
    );

    let mut i = 0;
    let i_max = 200;
    let res = mixture
        .bozman_distribution_with_closure_callback(i_max, || {
            let res = CalculateParams {
                T: 10_000.0 + 50.0 * (i as f64 - i_max as f64 / 2.0),
                p: 100_000.0 + 500.0 * (i as f64 - i_max as f64 / 2.0),
                n: [0.5, 0.5].as_slice().as_ptr(),
            };
            i += 1;
            res
        })
        .unwrap()
        .compute_transport_coefficient(None, None)
        .unwrap();
    println!("res {:?}", res.as_slice());
}
