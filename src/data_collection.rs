use log::error;
use std::{collections::HashMap, env};

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

pub async fn collect_data(water_temperature_sensor: &WaterTemperatureSensor) -> () {
    let collection_enabled =
        env::var("DATA_COLLECTION_ENABLED").expect("DATA_COLLECTION_ENABLED must be set");

    if collection_enabled.trim().parse().unwrap() && water_temperature_sensor.should_collect_data()
    {
        let mut json_body = HashMap::new();
        json_body.insert(
            "temperature_in_celcius",
            water_temperature_sensor.current_temperature,
        );

        let url = env::var("DATA_COLLECTION_URL").expect("DATA_COLLECTION_URL must be set");
        let data_collection_auth =
            env::var("DATA_COLLECTION_SECRET").expect("DATA_COLLECTION_SECRET must be set");
        let response = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .header("X-Require-Whisk-Auth", data_collection_auth)
            .json(&json_body)
            .send()
            .await
            .unwrap();

        error!("{}", response.status());
    }
}
