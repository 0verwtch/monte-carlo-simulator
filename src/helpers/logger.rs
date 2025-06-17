/**
* Description: This module provides logging functionality for the application
* 1. Initializes the fs:File
* 2. Creates a new file in the logs directory
* 3. Provides logging functions
*/

use std::fs::{self, File};
use std::io::{self, Write};
use chrono::Utc;


pub fn create_log_file(mod_path: &str) -> String {
    format!("logs/{}_{}.log", mod_path, Utc::now().format("%Y-%m-%d_%H-%M-%S"))
}
pub fn write_log(message: &str, log_file: &str) -> Result<(), io::Error> {
    // Ensure the logs directory exists and accessible
    fs::create_dir_all("logs").expect("Failed to create logs directory");
    // create or open the log file
    let mut file = File::options().append(true).create(true).open(log_file)
        .expect("Failed to open log file");
    // write the message to the log file
    writeln!(file, "{}", message).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to write to log file: {}", e))
    })?;
    Ok(())
}