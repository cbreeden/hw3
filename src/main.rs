#![allow(dead_code)]
extern crate rand;
extern crate gnuplot;

mod markov;
mod stats;
mod gillespie;
mod reaction;

use stats::V;
use stats::Stats;


fn finite_difference() {
    use markov::FiniteDifference;

    const X_0: V     = V(100., 5., 0.);
    const THETA: f64 = 0.05;
    const N: usize   = 10_000;

    // Finite Difference Method
    let h = [ 0.01, 0.005, 0.001, 0.0005 ];

    let mut mc = [
        FiniteDifference::new(X_0, THETA, h[0]),
        FiniteDifference::new(X_0, THETA, h[1]),
        FiniteDifference::new(X_0, THETA, h[2]),
        FiniteDifference::new(X_0, THETA, h[3]),
    ];

    let mut results = [
        Vec::with_capacity(N),
        Vec::with_capacity(N),
        Vec::with_capacity(N),
        Vec::with_capacity(N),
     ];

    for _ in 0..N {
        for idx in 0 .. h.len() {
            results[idx].push(mc[idx].simulate());
        }
    }

    println!("Finite Difference Method");
    for idx in 0 .. h.len() {
        print!("h {:<8}", h[idx]);
        results[idx].pretty_print();
    }
}

fn gillespie() {
    use gnuplot::{Figure, AutoOption, AxesCommon, Caption, Color};
    use gillespie::Gillespie;

    const X0: V    = V(15.0, 20.0, 0.0);
    const T: f64   = 2.0;
    const N: usize = 1_000;

    let mut gil = Gillespie::new(X0);
    let mut results = Vec::with_capacity(N);

    for _ in 0..N {
        results.push(gil.simulate(T).2);
    }

    let mut fig = Figure::new();

    fig
        .set_terminal("pngcairo", "gillespie.png")
        .axes2d()
        .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(2.0))
        .set_title("Gillespie Method Trajectory", &[])
        .set_x_label("Time", &[])
        .set_y_label("Count", &[])
        .lines(&gil.hist_t, &gil.hist_x, &[Caption("S_1"), Color("blue")])
        .lines(&gil.hist_t, &gil.hist_y, &[Caption("S_2"), Color("red")])
        .lines(&gil.hist_t, &gil.hist_z, &[Caption("S_3"), Color("green")]);

    fig.show();

    println!("Samples: {:?}", results.len());
    results.pretty_print();
}

fn reaction() {
    use reaction::Reaction;
    use gnuplot::{Figure, AutoOption, AxesCommon, Caption, Color};

    const X0: V    = V(1.0, 10.0, 50.0);
    const T: f64   = 8.0;
    const N: usize = 1_000;

    let mut rx = Reaction::new(X0);
    let mut results = Vec::with_capacity(N);

    for _ in 0..N {
        results.push(rx.simulate(T).2);
    }

    let mut fig = Figure::new();
    
    fig
        .set_terminal("pngcairo", "reaction.png")
        .axes2d()
        .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(8.0))
        .set_title("Reaction Method Trajectory", &[])
        .set_x_label("Time", &[])
        .set_y_label("Count", &[])
        .lines(&rx.hist_time, &rx.hist_mrna, &[Caption("mRNA"), Color("blue")])
        .lines(&rx.hist_time, &rx.hist_prot, &[Caption("Protein"), Color("red")]);

    fig.show();

    println!("Samples: {:?}", results.len());
    results.pretty_print();
}

fn main() {
    reaction();
    gillespie();
}