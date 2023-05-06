mod data_collection;
mod devices;
mod helpers;
mod loggings;

use std::env;

use data_collection::collect_data;
use log::{debug, info};
use loggings::init_logs;
use twilio::OutboundMessage;

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

fn get_phone_numbers() -> Vec<String> {
    env::var("TO_PHONE_NUMBERS")
        .expect("TO_PHONE_NUMBERS must be set")
        .split(",")
        .map(|x| x.trim().to_string())
        .collect()
}

async fn publish_message_to_sms(temperature: f32) -> () {
    let twilio_account_id = env::var("TWILIO_ACCOUNT_ID").expect("TWILIO_ACCOUNT_ID must be set");
    let twilio_auth_token = env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN must be set");

    let client = twilio::Client::new(twilio_account_id.as_str(), twilio_auth_token.as_str());

    let to_phone_numbers = get_phone_numbers();
    let from_phone_number = env::var("FROM_PHONE_NUMBER").expect("PHONE_NUMBER must be set");

    for to_phone_number in to_phone_numbers {
        let response = client
            .send_message(OutboundMessage::new(
                from_phone_number.as_str(),
                to_phone_number.as_str(),
                format!("The temperature is {}", temperature).as_str(),
            ))
            .await;
        debug!("Response: {:?}", response);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_logs().unwrap_or_else(|_| panic!("Unable to initialize logs"));

    let mut phone_notified = false;

    let mut water_temperature_sensor = WaterTemperatureSensor::new();
    loop {
        let temperature = water_temperature_sensor.read();
        if water_temperature_sensor.get_temperature_has_changed() {
            info!("Temperature: {}", temperature);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        collect_data(&water_temperature_sensor).await;

        if phone_notified && water_temperature_sensor.is_temperature_back_to_normal() {
            debug!("Resetting the flags ...");
            phone_notified = false;
            water_temperature_sensor.reset_temperature_back_to_normal()
        } else if water_temperature_sensor.is_temperature_back_to_normal() && !phone_notified {
            debug!("Notifying user ...");
            publish_message_to_sms(temperature).await;
            phone_notified = true;
        }
    }
}
