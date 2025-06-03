mod options;
mod asset_price_sim;
mod payoff_calc;

use std::time::Duration;
use options::ExerciseType::European;
use options::PayoffType::Buy;


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
         Duration::from_millis(5000).as_secs_f64()
    );

    let paths = asset_price_sim::run(option.clone());
    // for path in paths.clone() {
    //     println!("{:?}\n", path);
    // }
    
    // Pay off calculator
    let pay_offs = payoff_calc::run(option.clone(), paths);
    println!("{:#?}", pay_offs);
    for pay_off in pay_offs {
        println!("=== {:?}\n ===", pay_off);
    }
}
