/**
* Options structure
**/

#[derive(Debug, Clone)]
pub enum ExerciseType {
    European, // Exercised only at maturity
    American, // Exercised anytime before or at maturity
    Bermudan, // Exercised at a specific dates before expiry
}

#[derive(Debug, Clone)]
pub enum PayoffType {
    Buy,
    Sell,
}
#[derive(Debug, Clone)]
pub struct Options {
    pub exercise_type: ExerciseType,
    pub strike_price: f64,
    pub payoff_type: PayoffType,
    pub time_to_maturity: f64,
    pub volatility: f64,
    pub risk_free_rate: f64,
    pub asset_price: f64,
    pub time_steps: u16,
    pub number_of_sims: u16,
    pub exercise_time: f64,
}

impl Options {
    pub fn new(
        exercise_type: ExerciseType,
        strike_price: f64,
        payoff_type: PayoffType,
        time_to_maturity: f64,
        volatility: f64,
        risk_free_rate: f64,
        asset_price: f64,
        time_steps: u16,
        number_of_sims: u16,
        exercise_time: f64,
    ) -> Options {
        Self {
            exercise_type,
            strike_price,
            payoff_type,
            time_to_maturity,
            volatility,
            risk_free_rate,
            asset_price,
            time_steps,
            number_of_sims,
            exercise_time,
        }
    }
}
