
/**
* This module provides pay off calculator for each path based on the option type
*/
use std::time::{SystemTime, UNIX_EPOCH};
use crate::options::{ExerciseType, Options};

const SIX_MONTH_SEC: u64 = 15778800;
pub fn pay_off_calc(mut path: Vec<f64>, strike_price: f64) -> f64 {
    // Payoff calculator helper function
    path.pop().unwrap().max(strike_price)
}

pub fn get_time_stamp(start: SystemTime, end: SystemTime) -> SystemTime {
    
}
pub fn run(option: Options, paths: Vec<Vec<f64>>) -> Vec<f64> {
    // pay_off calculator function
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let mut results: Vec<f64> = vec![];
    match option.exercise_type {
        ExerciseType::European => {
            // Exercised at maturity
            if current_time == option.time_to_maturity {
                for path in paths {
                    results.push(pay_off_calc(path, option.strike_price));
                }
            }
            results
        }
        ExerciseType::American => {
            // Exercised anytime before or at maturity
            if current_time >= option.time_to_maturity {
                for path in paths {
                    results.push(pay_off_calc(path, option.strike_price));
                }
            }
            results
        }

        ExerciseType::Bermudan => {
            // Exercised at a specific date before expiry
            if current_time >= option.exercise_time {
                for path in paths {
                    results.push(pay_off_calc(path, option.strike_price));
                }
            }
            results
        }
        
    }
}
