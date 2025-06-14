use crate::influxdb::InfluxDB;
/**
* This module defines the Geometric brownian motion for generating different paths
*/
use crate::options::Options;
use futures::SinkExt;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;
use std::sync::mpsc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};

pub async fn run(option: Options) -> Arc<Mutex<Vec<Vec<f64>>>> {
    let influx = Arc::new(Mutex::new(InfluxDB::new(
        "default_org".to_string(),
        "default_bucket".to_string(),
        "default_token".to_string(),
    )));
    let dt = option.time_to_maturity / option.time_steps as f64; // Calculate the time increment
    let prices: Arc<Mutex<Vec<Vec<f64>>>> = Arc::new(Mutex::new(vec![vec![option.asset_price]]));
    let mut handles = vec![];

    // Create a channel to send prices to the InfluxDB writer
    let (tx, rx) = unbounded_channel::<(usize, usize, f64)>();

    // Spawn a task to handle writing to InfluxDB
    register_step_price(influx.clone(), rx).await;
    for path in 0..option.number_of_sims as usize {
        let prices = Arc::clone(&prices);
        let tx_clone = tx.clone();

        handles.push(tokio::spawn(async move {
            let mut rng = ChaCha20Rng::seed_from_u64(path as u64 * 42); // Seed the RNG with the path number
            let mut prices_at_step = vec![option.asset_price];
            for step in 0..option.time_steps as usize {
                let random_num = rng.random_range(0.0..1.0);
                let price = prices
                    .lock()
                    .await
                    .get(path)
                    .unwrap_or(&vec![option.asset_price])
                    .get(step)
                    .unwrap_or(&option.asset_price)
                    * ((option.risk_free_rate - (0.5 * option.volatility.powi(2))) * dt
                        + (option.volatility * dt.sqrt() * random_num))
                        .exp(); // Geometric brownian motion formula
                prices_at_step.push(price);

                // register_step_price(path, step, price, influx.clone()).await;
                tx_clone
                    .send((path, step, price))
                    .expect("Failed to send price");
            }
            prices.lock().await.push(prices_at_step);
        }));
    }
    for handle in handles {
        handle.await.expect("Failed to join task");
    }

    prices
}

async fn register_step_price(influx: Arc<Mutex<InfluxDB>>, mut rx: Receiver<(usize, usize, f64)>) {
    // This function will handle writing prices to InfluxDB in batches

    tokio::spawn(async move {
        
        let interval = tokio::time::interval(std::time::Duration::from_millis(100));
        while let Some((path, step, price)) = rx.recv().await {
            // Write to influxdb
            let point = {
                let influx = influx.clone();
                let influx = influx.lock().await;
                influx
                    .create(
                        price,
                        "path".to_string(),
                        format!("{}-{}", path, step),
                        "asset_price".to_string(),
                        "path_price".to_string(),
                    )
                    .expect("Failed to write to InfluxDB")
            };
            {
                let mut influx = influx.lock().await;
                influx.batch.push(point);
            }

            if influx.lock().await.batch.len() >= 1000 {
                let mut influx = influx.lock().await;
                influx.write().await.expect("Failed to write to InfluxDB");
                influx.batch.clear();
            }
        }
        // Flush any remaining points after the loop ends
        if !influx.lock().await.batch.is_empty() {
            let mut influx = influx.lock().await;
            influx.write().await.expect("Failed to write to InfluxDB");
        }
    });
}
