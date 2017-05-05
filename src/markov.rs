use rand;
use rand::distributions::IndependentSample;

use stats::V;

const XI_1: V = V(-1., 1., 0.);
const XI_2: V = V(0., -1., 1.);

pub struct FiniteDifference {
    pub v1:    V,
    pub v2:    V,
    pub init:  V,
    pub theta: f64,
    pub h:     f64,
    rng:       rand::XorShiftRng,
}

impl FiniteDifference {
    pub fn new(v: V, theta: f64, h: f64) -> FiniteDifference {
        FiniteDifference {
            v1:    v,
            v2:    v,
            init:  v,
            theta: theta,
            h:     h,
            rng:   rand::weak_rng(),
        }
    }

    pub fn simulate(&mut self) -> f64 {
        use rand::distributions::Range;

        // Generate a common list of uniforms
        let range = Range::new(0.0, 1.0);

        let t1 = self.theta + self.h / 2.0;
        let t2 = self.theta - self.h / 2.0;

        for _ in 0..100 {
            let u   = range.ind_sample(&mut self.rng);
            self.v1 = sample(u, t1, self.v1);
            self.v2 = sample(u, t2, self.v2);
        }

        let res = (self.v1.0 - self.v2.0) / self.h;

        // Reset the buffer
        self.v1 = self.init;
        self.v2 = self.init;

        res
    }
}

fn sample(u: f64, t: f64, v: V) -> V {
    if u < p2(t, v) {
        v + XI_2
    } else {
        v + XI_1
    }
}

fn p2(t: f64, v: V) -> f64 {
    1. / (t * v.0 + 1.)
}