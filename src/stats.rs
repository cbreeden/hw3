use rand;
use std::ops;
use rand::distributions::IndependentSample;

pub struct Noise {
    rng:   rand::XorShiftRng,
    range: rand::distributions::Range<f64>,
}

impl Noise {
    pub fn new() -> Noise {
        use rand::distributions::Range;

        Noise {
            rng:   rand::weak_rng(),
            range: Range::new(0.0, 1.0),
        }
    }

    pub fn ind_sample(&mut self) -> f64 {
        self.range.ind_sample(&mut self.rng)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V(pub f64, pub f64, pub f64);

impl ops::Add for V {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        V(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

pub trait Stats {
    fn average(&self) -> f64;
    fn variance(&self) -> f64;
    fn std_dev(&self) -> f64;
    fn confidence_interval(&self) -> f64;
    fn pretty_print(&self);
}

impl Stats for [f64] {
    fn average(&self) -> f64 {
        self.iter().sum::<f64>() / (self.len() as f64)
    }

    fn variance(&self) -> f64 {
        let avg = self.average();

        let sq_avg = self.iter()
            .map(|&n| n*n)
            .sum::<f64>() / (self.len() as f64);

        sq_avg - avg * avg
    }

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn confidence_interval(&self) -> f64 {
        (1.96 * self.std_dev())
            / (self.len() as f64).sqrt()
    }

    fn pretty_print(&self) {
        println!("Average = {:<10.2} Variance = {:<10.2} StdDev = {:<10.2} Confidence = {:<10.2}",
            self.average(),
            self.variance(),
            self.std_dev(),
            self.confidence_interval());
    }
}

#[cfg(test)]
mod test {
    use super::Stats;

    #[test]
    fn average() {
        let dice = vec![1.,2.,3.,4.,5.,6.];
        assert!(
            (dice.average() - 3.5).abs() < 0.01
        );
    }

    #[test]
    fn variance() {
        let dice = vec![1.,2.,3.,4.,5.,6.];
        assert!(
            (dice.variance() - 2.92).abs() < 0.01
        );
    }
}