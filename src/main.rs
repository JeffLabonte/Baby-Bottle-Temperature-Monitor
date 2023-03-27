mod devices;

use crate::devices::water_temperature_sensor::WaterTemperatureSensor;

fn main() {
    let mut water_temperature_sensor = WaterTemperatureSensor::new();
    loop {
        println!("Temperature: {}", water_temperature_sensor.read());
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
