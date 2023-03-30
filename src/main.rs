mod devices;

use std::env;

use twilio::OutboundMessage;

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

async fn publish_message_to_sms(
    client: &twilio::Client,
    temperature: f32,
    to_phone_number: &String,
    from_phone_number: &String,
) -> () {
    client
        .send_message(OutboundMessage::new(
            to_phone_number.as_str(),
            from_phone_number.as_str(),
            format!("The water bottle temperature is {}", temperature).as_str(),
        ))
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client = twilio::Client::new(
        env::var("TWILIO_ACCOUNT_ID")
            .expect("TWILIO_ACCOUNT_ID must be set")
            .as_str(),
        env::var("TWILIO_AUTH_TOKEN")
            .expect("TWILIO_AUTH_TOKEN must be set")
            .as_str(),
    );
    let to_phone_number = env::var("TO_PHONE_NUMBER").expect("PHONE_NUMBER must be set");
    let from_phone_number = env::var("FROM_PHONE_NUMBER").expect("PHONE_NUMBER must be set");
    let mut phone_notified = false;

    let mut water_temperature_sensor = WaterTemperatureSensor::new();
    loop {
        let temperature = water_temperature_sensor.read();
        println!("Temperature: {}", temperature);
        std::thread::sleep(std::time::Duration::from_secs(1));

        if phone_notified && water_temperature_sensor.is_temperature_back_to_normal() {
            println!("Resetting the flags ...");
            phone_notified = false;
            water_temperature_sensor.reset_temperature_back_to_normal()
        } else if water_temperature_sensor.is_temperature_back_to_normal() && !phone_notified {
            println!("Notifying user ...");
            publish_message_to_sms(&client, temperature, &to_phone_number, &from_phone_number)
                .await;
            phone_notified = true;
        }
    }
}
