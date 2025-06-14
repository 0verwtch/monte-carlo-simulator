use chrono::Utc;
use dotenv;
use futures::stream;
/**
* Description: This module provides methods to read and write to an influxdb 2 instance
* T
*/
use influxdb2::{Client, models::DataPoint, api::write::TimestampPrecision};
use std::env;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct InfluxDB {
    pub org: String,
    pub bucket: String,
    pub token: String,
    pub client: Client,
    pub batch: Vec<DataPoint>,
}

impl InfluxDB {
    pub fn new(org: String, bucket: String, token: String) -> InfluxDB {
        dotenv::dotenv().ok();

        // Load from env, fallback to args
        let org_env = env::var("INFLUXDB_ORG").unwrap_or(org);
        let bucket_env = env::var("INFLUXDB_BUCKET").unwrap_or(bucket);
        let token_env = env::var("INFLUXDB_TOKEN").unwrap_or(token);
        let url = env::var("INFLUX_URL").expect("INFLUX_URL not set");

        let client = Client::new(&url, &org_env, &token_env);

        Self {
            org: org_env,
            bucket: bucket_env,
            token: token_env,
            client,
            batch: Vec::with_capacity(1000),
            
        }
    }
    pub fn create(
        &self,
        value: f64,
        tag: String,
        tag_value: String,
        measurement: String,
        field_name: String,
    ) -> Result<DataPoint, Box<dyn std::error::Error>> {
        // Create a data point with the given value, tag, and measurement
        let point = DataPoint::builder(measurement.clone())
            .tag(tag.clone(), tag_value.clone())
            .field(field_name, value)
            .timestamp((Utc::now().timestamp_millis()))
            .build()
            .expect("Failed to build data point");

        Ok(point)
    }
    pub async fn write(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Write batch data point to InfluxDB
        let points: Vec<_> = self.batch.drain(..).collect();
        self.client
            .write_with_precision(
                self.bucket.as_str(),
                stream::iter(points.clone()),
                TimestampPrecision::Milliseconds,
            )
            .await
            .expect("Failed to write to InfluxDB");
        println!("Wrote {} points to InfluxDB", points.clone().len());
        Ok(())
    }
}
