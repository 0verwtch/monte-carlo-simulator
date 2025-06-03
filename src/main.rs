mod options;
mod asset_price_sim;

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
    );
    
    let paths = asset_price_sim::run(option);
    for path in paths {
        println!("{:?}\n", path);
    }
}
