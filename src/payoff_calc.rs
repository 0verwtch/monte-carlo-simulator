/**
* Description: This module provides pay off calculator for each path based on the option type
* 1. Calculates timestamps
* 2. Calculates the payoff per path
* 3. Returns a vector with the payoffs per path
*/

use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::options::{ExerciseType, Options};

pub fn pay_off_calc(mut path: Vec<f64>, strike_price: f64) -> f64 {
    // Payoff calculator helper function
    path.pop().unwrap().max(strike_price)
}

pub fn get_time_stamp(end: f32) -> u64 {
    let end_secs = end * 365.0f32 * 24.0 * 60.0 * 60.0;
    println!("{:#?}", end_secs);
    let calculated_duration = SystemTime::now().add(Duration::from_secs(end_secs as u64)).duration_since(UNIX_EPOCH).unwrap();
    calculated_duration.as_secs()
}

pub fn run(option: Options, paths: Vec<Vec<f64>>) -> Vec<f64> {

    // pay_off calculator function
    let mut results: Vec<f64> = vec![];
    for path in paths {
        results.push(pay_off_calc(path, option.strike_price));
    }
    results
}
