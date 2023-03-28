mod devices;

use std::env;

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_sns::Client::new(&config);

    let phone_number = env::var("PHONE_NUMBER").expect("PHONE_NUMBER must be set");
    let topic_arn = env::var("TOPIC_ARN").expect("TOPIC_ARN must be set");

    let mut water_temperature_sensor = WaterTemperatureSensor::new();
    loop {
        let temperature = water_temperature_sensor.read();
        println!("Temperature: {}", temperature);
        std::thread::sleep(std::time::Duration::from_secs(1));
        client
            .subscribe()
            .topic_arn(&topic_arn)
            .protocol("sms")
            .endpoint(&phone_number)
            .send()
            .await
            .unwrap();

        client
            .publish()
            .topic_arn(&topic_arn)
            .message(&format!("The water temperature is {}", temperature))
            .send()
            .await
            .unwrap();
    }
}
