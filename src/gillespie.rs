use stats::V;
use stats::Noise;

const XI_1: V = V(-1.0, -1.0,  1.0);
const XI_2: V = V( 1.0,  1.0, -1.0);

fn lambda_1(v: V) -> f64 {
    0.1 * v.0 * v.1
}

fn lambda_2(v: V) -> f64 {
    v.2
}

pub struct Gillespie {
    v:     V,
    t:     f64,
    pub hist_x: Vec<f64>,
    pub hist_y: Vec<f64>,
    pub hist_z: Vec<f64>,
    pub hist_t: Vec<f64>,
    init:  V,
    unif:  Noise,
}

impl Gillespie {
    pub fn new(v: V) -> Gillespie {
        Gillespie {
            v:      v,
            t:      0.0,
            hist_x: vec![v.0],
            hist_y: vec![v.1],
            hist_z: vec![v.2],
            hist_t: vec![0.0],
            init:   v,
            unif:   Noise::new(),
        }
    }

    pub fn simulate(&mut self, t: f64) -> V {
        self.reset();

        while self.t < t {
            self.sample()
        }

        self.v
    }

    pub fn sample(&mut self) {
        let lambda1 = lambda_1(self.v);
        let lambda2 = lambda_2(self.v);

        let lambda = lambda1 + lambda2;

        let u1 = self.unif.ind_sample();
        let u2 = self.unif.ind_sample();

        self.t += - u1.ln() / lambda;

        self.v = self.v + 
            if u2 < lambda1 / lambda {
                XI_1
            } else {
                XI_2
            };
        
        self.hist_x.push(self.v.0);
        self.hist_y.push(self.v.1);
        self.hist_z.push(self.v.2);
        self.hist_t.push(self.t);
    }

    pub fn reset(&mut self) {
        self.v = self.init;
        self.t = 0.0;
        self.hist_x = vec![self.init.0];
        self.hist_y = vec![self.init.1];
        self.hist_z = vec![self.init.2];
        self.hist_t = vec![0.0];
    }
}