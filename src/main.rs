#![allow(dead_code)]
extern crate rand;
mod markov;
mod stats;

use markov::LikelihoodRatio;
use markov::MarkovChain;
use markov::V;

use stats::Stats;

const X_0: V = V(100., 5., 0.);
const THETA: f64 = 0.05;

fn finite_difference(mc: &mut MarkovChain, mc_h: &mut MarkovChain, h: f64) -> f64 {
    // We take n ~ h^{-2} to compensate for variance introduced
    // let n = h.powi(-2) as usize;

    // let mut res = Vec::with_capacity(n);
    // for _ in 0..n {
    //     let y   = mc.simulate();
    //     let y_h = mc_h.simulate();

    //     res.push((y_h - y) / h);
    // }

    // res.average()

    let y   = mc.simulate();
    let y_h = mc_h.simulate();

    (y_h - y) / h
}

fn main() {
    // Finite Difference Method
    // We use various h.

    let mut mc = MarkovChain::new(X_0, THETA);

    let h        = [ 0.01, 0.005, 0.001, 0.0005 ];
    let mut fd   = [ vec![], vec![], vec![], vec![] ];
    let mut mc_h = [
        MarkovChain::new(X_0, THETA + h[0]),
        MarkovChain::new(X_0, THETA + h[1]),
        MarkovChain::new(X_0, THETA + h[2]),
        MarkovChain::new(X_0, THETA + h[3]),
    ];

    for _ in 0..100000 {
        for idx in 0 .. h.len() {
            fd[idx].push(finite_difference(&mut mc, &mut mc_h[idx], h[idx]));
        }
    }

    println!("Finite Difference Method");
    for idx in 0 .. h.len() {
        print!("h {:<8}", h[idx]);
        fd[idx].pretty_print();
    }

    // Likelihood Ratio Method
    // No Control Variance
    let mut lh = LikelihoodRatio::new(X_0, THETA);
    let mut lh_res = Vec::new();
    let mut lh_cv  = Vec::new(); // Control variate will be saved for later

    for _ in 0..10000 {
        let (v, cv) = lh.simulate();
        lh_res.push(v);
        lh_cv.push(cv);
    }

    println!("");
    println!("Likelihood Ratio Method, No Control Variance");
    lh_res.pretty_print();

    // Likelihood Ratio Method
    // With control variance
    let alpha = control_variate(&lh_res, &lh_cv);
    let res   =
        lh_res.iter().zip(lh_cv.iter())
            .map(|(&v, &cv)| v - alpha * cv)
            .collect::<Vec<_>>();

    println!("");
    println!("Liklihood Ratio Method, w/ CV: Alpha = {}", alpha);
    res.pretty_print();
}

fn control_variate(x: &[f64], y: &[f64]) -> f64 {
    let alpha =
        x.iter().zip(y.iter())
            .map(|(&a, &b)| a*b)
            .sum::<f64>() /
        y.iter().map(|&a| a.powi(2))
            .sum::<f64>();

    alpha
}