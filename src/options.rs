/**
* Options structure
**/

pub enum ExerciseType {
    European, // Exercised only at maturity
    American, // Exercised anytime before or at maturity
    Bermudan // Exercised at a specific dates before expiry
}

pub enum PayoffType {
    Buy,
    Sell,
}
pub struct Options {
    pub exercise_type: ExerciseType,
    pub strike_price: f64,
    pub payoff_type: PayoffType,
    pub time_to_maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub asset_price: f64,
    pub time_steps: u8,
    pub number_of_sims: u16
}