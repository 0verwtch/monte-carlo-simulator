mod options;
mod asset_price_sim;
mod payoff_calc;

use std::time::Duration;
use options::ExerciseType::European;
use options::PayoffType::Buy;
use crate::payoff_calc::pay_off_calc;

fn main() {
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

    let paths = asset_price_sim::run(option.clone());
    
    // Pay off calculator
    let pay_offs = payoff_calc::run(option.clone(), paths);
    println!("{:#?}", payoff_calc::get_time_stamp(0.5,));
    for pay_off in pay_offs {
        println!("{:?}\n", pay_off);
    }
}
