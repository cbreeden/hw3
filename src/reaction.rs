use stats::V;
use stats::Noise;

const Z_1: V = V(0.0,  1.0,  0.0);
const Z_2: V = V(0.0,  0.0,  1.0);
const Z_3: V = V(0.0, -1.0,  0.0);
const Z_4: V = V(0.0,  0.0, -1.0);
static Zs: [V; 4] = [Z_1, Z_2, Z_3, Z_4];

fn update_lambda(v: V) -> [f64; 4] {
    [200.0, 10.0 * v.1, 25.0 * v.1, v.2]
}

fn update_ts(ts: [f64; 4], ls: [f64; 4], dt: f64) -> [f64; 4] {
    [ts[0] + ls[0] * dt, 
     ts[1] + ls[1] * dt, 
     ts[2] + ls[2] * dt, 
     ts[3] + ls[3] * dt]
}

fn calc_dt(ps: [f64; 4], ts: [f64; 4], ls: [f64; 4]) -> (usize, f64) {
    let mut min = ::std::f64::INFINITY;
    let mut arg = 0usize;

    for idx in 0..4 {
        let r = (ps[idx] - ts[idx]) / ls[idx];
        if  r < min {
            min = r;
            arg = idx;
        }
    }

    (arg, min)
}

pub struct Reaction {
    v:     V,
    init:  V,
    t:     f64,
    ts:    [f64; 4],
    ps:    [f64; 4],
    ls:    [f64; 4],
    pub hist_mrna: Vec<f64>,
    pub hist_prot: Vec<f64>,
    pub hist_time: Vec<f64>,
    unif:  Noise,
}

impl Reaction {
    pub fn new(v: V) -> Reaction {
        Reaction {
            v:     v,
            init:  v,
            t:     0.0,
            ts:    [0.0; 4],
            ls:    [0.0; 4],
            ps:    [0.0; 4],
            hist_mrna: Vec::new(),
            hist_prot: Vec::new(),
            hist_time: Vec::new(),
            unif:  Noise::new(),
        }
    }

    pub fn simulate(&mut self, t: f64) -> V {
        self.reset();

        while self.t < t {
            self.sample();
            self.hist_mrna.push(self.v.1);
            self.hist_prot.push(self.v.2);
            self.hist_time.push(self.t);
        }

        self.v
    }

    pub fn sample(&mut self) {
        self.ls = update_lambda(self.v);
        let (idx, dt) = calc_dt(self.ps, self.ts, self.ls);

        self.t += dt;
        self.v = self.v + Zs[idx];
        self.ts = update_ts(self.ts, self.ls, dt);

        let u = self.unif.ind_sample();
        self.ps[idx] += (1.0 / u).ln();
    }

    pub fn reset(&mut self) {
        self.v = self.init;
        self.t = 0.0;
        self.ts = [0.0; 4];
        self.ls = [0.0; 4];
        self.ps = [
            (1.0 / self.unif.ind_sample()).ln(),
            (1.0 / self.unif.ind_sample()).ln(),
            (1.0 / self.unif.ind_sample()).ln(),
            (1.0 / self.unif.ind_sample()).ln(),
        ];

        self.hist_mrna = Vec::new();
        self.hist_prot = Vec::new();
        self.hist_time = Vec::new();
    }
}