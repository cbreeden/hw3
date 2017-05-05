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
        // Sum X_i^2 / N - Avg*Avg
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