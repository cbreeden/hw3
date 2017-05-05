#![allow(dead_code)]
extern crate rand;
mod markov;
mod stats;

use markov::FiniteDifference;
use markov::V;

use stats::Stats;

const X_0: V     = V(100., 5., 0.);
const THETA: f64 = 0.05;
const N: usize   = 10_000;

fn main() {
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