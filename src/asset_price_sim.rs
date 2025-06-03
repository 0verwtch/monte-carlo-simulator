use crate::options::Options;
/**
* This module defines the Geometric brownian motion for generating different paths
*/
use rand::Rng;

pub fn run(option: Options) -> Vec<Vec<f64>> {
    // this function returns the price paths of a given options
    let dt = option.time_to_maturity / option.time_steps as f64; // Calculate the time increment
    let mut rng = rand::rng();
    let mut prices: Vec<Vec<f64>> = vec![vec![option.asset_price]];
    for path in 0..option.number_of_sims as usize {
        let mut prices_at_step = vec![option.asset_price];
        for step in 0..option.time_steps as usize {
            let random_num = rng.random_range(0.0..1.0);
            let price = prices.get(path).unwrap_or(&vec![option.asset_price]).get(step).unwrap_or(&option.asset_price)
                * ((option.risk_free_rate - (0.5 * option.volatility.powi(2))) * dt
                    + (option.volatility * dt.sqrt() * random_num))
                    .exp(); // Geometric brownian motion formula
            prices_at_step.push(price);
        }
        prices.push(prices_at_step);
    }
    
    prices
}
