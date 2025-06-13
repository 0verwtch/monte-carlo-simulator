/**
* Description: This module provides an adjusted price for an option given the performance payoff
* 1. Calculates the average payoff
* 2. Calculates the discounted price
* 3. Calculates the variance of the payoffs to emulate how it varies across paths
* 4. Calculates the standard error 
* 5. Returns a Pricing instance
*/

#[derive(Debug)]
pub struct Pricing {
    pub discounted_price: f64,
    pub variance: f64,
    pub standard_error: f64,
    pub avg_payoff: f64,
}


impl Pricing {
    pub fn new(discounted_price: f64, variance: f64, standard_error: f64, avg_payoff: f64) -> Pricing {
        Self {
            discounted_price,
            variance,
            standard_error,
            avg_payoff,
        }
    }
}
pub fn run(payoffs: Vec<f64>, r:f64, t:f64) -> Pricing {
    let avg_payoff = payoffs.iter().sum::<f64>() / payoffs.len() as f64;
    println!("avg_payoff: {}", avg_payoff);
    let discounted_price = (-r * t).exp() * avg_payoff;
    let variance = {
        let mean = avg_payoff;
        let variance = payoffs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (payoffs.len() as f64);
        variance.sqrt()
    };
    let standard_err = variance / (payoffs.len() as f64).sqrt();
    Pricing::new(discounted_price, variance, standard_err, avg_payoff)
}
