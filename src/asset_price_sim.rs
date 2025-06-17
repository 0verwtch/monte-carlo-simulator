/**
* This module defines the Geometric brownian motion for generating different paths
*/
use crate::options::Options;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn run(option: Options) -> Arc<Mutex<Vec<Vec<f64>>>> {
    let dt = option.time_to_maturity / option.time_steps as f64; // Calculate the time increment
    let prices: Arc<Mutex<Vec<Vec<f64>>>> = Arc::new(Mutex::new(vec![vec![option.asset_price]]));

    for path in 0..option.number_of_sims as usize {
        let prices = Arc::clone(&prices);

        // handles.push(tokio::spawn(async move {
        let mut rng = ChaCha20Rng::seed_from_u64(path as u64 * 42); // Seed the RNG with the path number
        let mut prices_at_step = vec![option.asset_price];
        for step in 0..option.time_steps as usize {
            let random_num = rng.random_range(0.0..1.0);
            let price = prices
                .lock()
                .await
                .get(path)
                .unwrap_or(&vec![option.asset_price])
                .get(step)
                .unwrap_or(&option.asset_price)
                * ((option.risk_free_rate - (0.5 * option.volatility.powi(2))) * dt
                    + (option.volatility * dt.sqrt() * random_num))
                    .exp(); // Geometric brownian motion formula
            prices_at_step.push(price);

        }
        prices.lock().await.push(prices_at_step);
    }

    prices
}
