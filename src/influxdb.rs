
/**
* Description: This module provides methods to read and write to an influxdb 2 instance
* T
*/
use influxdb2::{models::DataPoint, Client};
use std::env;
use chrono::Utc;
use dotenv;
use futures::stream;

struct InfluxDB {
    org: String,
    bucket: String,
    token: String,
    client: Client,
}

impl InfluxDB {
    fn new(org: String, bucket: String, token: String, client: Client) -> InfluxDB {
        Self {
            org,
            bucket,
            token,
            client,
        }
    }

    pub async fn create(
        value: f64,
        tag: String,
        tag_value: String,
        measurement: String,
        field_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let org = env::var("INFLUXDB_ORG");
        let bucket = env::var("INFLUXDB_BUCKET");
        let token = env::var("INFLUXDB_TOKEN");
        let client = Client::new(env::var("INFLUX_URL").unwrap(), org.unwrap(), token.unwrap());

        let point = DataPoint::builder(measurement)
            .tag(tag, tag_value)
            .field(field_name, value.to_string())
            .timestamp(Utc::now().timestamp_millis())
            .build()
            .expect("Failed to build data point");
        client.write(bucket.unwrap().as_str(), stream::once(async { point }));
        Ok(())
    }
}
