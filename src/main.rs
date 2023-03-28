mod devices;

use std::env;

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

async fn publish_message_to_sms(
    client: &aws_sdk_sns::Client,
    temperature: f32,
    phone_number: &String,
    topic_arn: &String,
) -> () {
    client
        .subscribe()
        .topic_arn(topic_arn)
        .protocol("sms")
        .endpoint(phone_number)
        .send()
        .await
        .unwrap();

    client
        .publish()
        .topic_arn(topic_arn)
        .message(&format!("The water temperature is {} Celcius", temperature))
        .send()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_sns::Client::new(&config);
    let phone_number = env::var("PHONE_NUMBER").expect("PHONE_NUMBER must be set");
    let topic_arn = env::var("TOPIC_ARN").expect("TOPIC_ARN must be set");
    let mut phone_notified = false;

    let mut water_temperature_sensor = WaterTemperatureSensor::new();
    loop {
        let temperature = water_temperature_sensor.read();
        println!("Temperature: {}", temperature);
        std::thread::sleep(std::time::Duration::from_secs(1));

        if water_temperature_sensor.get_has_temperature_changed() && !phone_notified {
            publish_message_to_sms(&client, temperature, &phone_number, &topic_arn).await;
        } else if !water_temperature_sensor.get_has_temperature_changed() && phone_notified {
            phone_notified = false;
        }
    }
}
