/**
* Description: Main entrance of the program
* 1. Runs the paths generator
* 2. Calculates the payoffs per path
* 3. Generates the final price adjusted using the pricing engine
*/

mod options;
mod asset_price_sim;
mod payoff_calc;
mod influxdb;
mod pricing_engine;

use options::ExerciseType::European;
use options::PayoffType::Buy;
use tokio;

#[tokio::main]
async fn main() {
    let option: options::Options = options::Options::new(
        European,
        100.0,
        Buy,
    0.5,        // in years (6 months)
         0.20,             // 20% annualized
         0.035,        // 3.5% annual risk-free rate
         102.0,           // current price of underlying asset
         100,              // Monte Carlo time granularity
         10000,
         payoff_calc::get_time_stamp(0.5) as f64
    );

    // Path generation
    let paths = asset_price_sim::run(option.clone());
    
    // Pay off calculator
    let pay_offs = payoff_calc::run(option.clone(), paths.await.lock().await.clone());
    
    // Pricing estimation
    let pricing = pricing_engine::run(pay_offs.clone(), option.risk_free_rate, option.time_to_maturity);
    println!("{:?}", pricing);
}
