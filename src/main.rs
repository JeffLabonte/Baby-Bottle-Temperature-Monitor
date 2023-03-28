mod devices;

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_sns::Client::new(&config);

    let mut water_temperature_sensor = WaterTemperatureSensor::new();
    loop {
        println!("Temperature: {}", water_temperature_sensor.read());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
