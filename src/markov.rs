use rand;
use rand::distributions::{IndependentSample, Range};
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V(pub f64, pub f64, pub f64);

impl ops::Add for V {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        V(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

const X_0:  V = V(100., 5., 0.);
const XI_1: V = V(-1., 1., 0.);
const XI_2: V = V(0., -1., 1.);

pub struct MarkovChain {
    pub v: V,
    pub theta: f64,
    rng:   rand::ThreadRng,
    range: Range<f64>,
}

impl MarkovChain {
    pub fn new(v: V, theta: f64) -> MarkovChain {
        MarkovChain {
            v: v,
            theta: theta,
            rng: rand::thread_rng(),
            range: Range::new(0.0, 1.0),
        }
    }


    // f(X^theta)
    pub fn simulate(&mut self) -> f64 {
        for _ in 0..100 {
            self.sample();
        }

        let res = self.v.0; // (X^\theta_0)_100
        self.reset();
        res
    }

    pub fn sample(&mut self) -> Event {
        let cutoff = self.p2();
        let n = self.range.ind_sample(&mut self.rng);

        if n < cutoff {
            self.v = self.v + XI_2;
            Event::P2
        } else {
            self.v = self.v + XI_1;
            Event::P1
        }
    }

    fn reset(&mut self) {
        self.v = X_0;
    }

    fn p1(&self) -> f64 {
        (self.theta * self.v.0)
            / (self.theta * self.v.0 + 1.0)
    }

    fn p2(&self) -> f64 {
        1. / (self.theta * self.v.0 + 1.)
    }
}

//
// Likelihood Ratio method related types.
//

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    P1,
    P2,
}

impl Event {
    fn ln_dtheta(&self, v: V, t: f64) -> f64 {
        match *self {
            // P1'/P1 = 1/ [ 0(0x + 1) ]
            Event::P1 => {
                let x = v.0;
                let y = v.1;

                if y <= 0. { return 0. }
                1. / (t*(t*x + 1.))
            },

            // P2'/P2 = -x / (0x + 1)
            Event::P2 => {
                let x = v.0;
                let y = v.1;

                if y <= 0. { return 0. }
                - x / (t*x + 1.)
            }
        }
    }
}

pub struct LikelihoodRatio {
    mc:      MarkovChain,
    density: f64
}

impl LikelihoodRatio {
    pub fn new(v: V, theta: f64) -> LikelihoodRatio {
        LikelihoodRatio {
            mc: MarkovChain {
                v: v,
                theta: theta,
                rng: rand::thread_rng(),
                range: Range::new(0.0, 1.0),
            },
            density: 0.,
        }
    }

    pub fn simulate(&mut self) -> (f64, f64) {
        for _ in 0..100 {
            self.sample();
        }

        let state   = self.mc.v;
        let density = self.density;
        let result  = state.0 * density;

        self.reset();
        (result, density)
    }

    fn sample(&mut self) {
        let theta = self.mc.theta;
        let state = self.mc.v;
        let event = self.mc.sample();

        let dln = event.ln_dtheta(state, theta);

        assert!(!dln.is_nan());

        self.density += dln;
    }

    fn reset(&mut self) {
        self.mc.reset();
        self.density = 0.0;
    }
}
