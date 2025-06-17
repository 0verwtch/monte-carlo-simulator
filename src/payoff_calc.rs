/**
* Description: This module provides pay off calculator for each path based on the option type
* 1. Calculates timestamps
* 2. Calculates the payoff per path
* 3. Returns a vector with the payoffs per path
*/
use std::ops::Add;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::helpers::influxdb::{InfluxDB, register_batch_points};
use crate::helpers::logger::{write_log, create_log_file};
use crate::options::{ExerciseType, Options};



pub fn pay_off_calc(mut path: Vec<f64>, strike_price: f64) -> f64 {
    // Payoff calculator helper function
    path.pop().unwrap().max(strike_price)
}

pub fn get_time_stamp(end: f32) -> u64 {
    let end_secs = end * 365.0f32 * 24.0 * 60.0 * 60.0;
    println!("{:#?}", end_secs);
    let calculated_duration = SystemTime::now()
        .add(Duration::from_secs(end_secs as u64))
        .duration_since(UNIX_EPOCH)
        .unwrap();
    calculated_duration.as_secs()
}

pub async fn run(option: Options, paths: Vec<Vec<f64>>) -> Vec<f64> {
    let log_file = create_log_file("payoff_calc");
    // pay_off calculator function
    let mut results: Vec<f64> = vec![];
    for path in paths {
        results.push(pay_off_calc(path, option.strike_price));
    }
    // Register the payoffs in InfluxDB
    let influx = InfluxDB::new(
        "default_org".to_string(),
        "default_bucket".to_string(),
        "default_token".to_string(),
    );
    let tag = "payoff".to_string();
    let measurement = "payoff_measurement".to_string();
    let field_name = "payoff_value".to_string();
    let influx = Arc::new(Mutex::new(influx));
    let influx_clone = Arc::clone(&influx);
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<(usize, f64)>();

    // Spawn a task to handle reading from the channel and writing to InfluxDB
    register_batch_points(influx_clone.clone(), rx, tag.clone(), measurement.clone(), field_name.clone());
    // Spawn a task to handle writing to InfluxDB
    for (i, result) in results.iter().enumerate() {
        // Send the payoff to the channel
        tx.send((i, *result)).expect("Failed to send payoff");
        write_log(format!("Payoff calculated for path {}: {}", i, *result).as_str(), &log_file).unwrap();

    }
    println!("{:?}", results.len());
    drop(tx); // Close the channel to signal that no more messages will be sent
    results
}
 