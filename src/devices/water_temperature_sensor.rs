use chrono::{DateTime, Utc};

pub struct WaterTemperatureSensor {
    pub current_temperature: f32,
    temperature_filepath: String,
    last_temperature: f32,
    temperature_threshold: u8,
    temperature_has_changed: bool,
    temperature_back_to_normal: bool,
    temperatures_collected_for_rate: Vec<(DateTime<Utc>, f32)>,
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        #[cfg(test)]
        use mockall::automock;

        #[automock]
        impl WaterTemperatureSensor{
            pub fn new() -> Self {
                WaterTemperatureSensor{
                    temperature_filepath: String::from(""),
                    current_temperature: 0.0,
                    last_temperature: 0.0,
                    temperature_threshold: 30,
                    temperature_has_changed: false,
                    temperature_back_to_normal: false,
                    temperatures_collected_for_rate: Vec::new(),
                }
            }

            pub fn read(&mut self) -> f32 {
                self.current_temperature
            }

            pub fn should_collect_data(&self) -> bool {
                self.current_temperature != self.last_temperature
            }

            pub fn is_temperature_back_to_normal(&self) -> bool {
                self.temperature_back_to_normal
            }

            pub fn get_temperature_has_changed(&self) -> bool {
                self.temperature_has_changed
            }

            pub fn reset_temperature_back_to_normal(&mut self) {
                self.temperature_back_to_normal = false;
            }

            pub fn is_sampling_ready(&mut self) -> bool {
                true
            }

            pub fn flush(&mut self) {
                self.temperatures_collected_for_rate.clear()
            }

            pub fn get_cooling_rate_per_sec(&mut self) -> f32{
                0.0
            }
        }
    } else {
        use std::fs;

        use log::{debug, info};

        const BASE_DIR_TEMPERATURE_SENSOR: &str = "/sys/bus/w1/devices/";
        const SAMPLING_SIZE: usize = 300;
        const QUERY_DELAY_TIME_IN_SECONDS: u64 = 1;

        impl WaterTemperatureSensor {
            pub fn new() -> Self {
                WaterTemperatureSensor {
                    temperature_filepath: WaterTemperatureSensor::get_temperature_filepath().unwrap(),
                    current_temperature: 0.0,
                    last_temperature: 0.0,
                    temperature_threshold: 30,
                    temperature_has_changed: false,
                    temperature_back_to_normal: false,
                    temperatures_collected_for_rate: Vec::new(),
                }
            }

            pub fn read(&mut self) {
                debug!("Reading temperature from {}\n", self.temperature_filepath);
                self.last_temperature = self.current_temperature;
                self.current_temperature = fs::read_to_string(&self.temperature_filepath)
                    .unwrap()
                    .trim()
                    .parse::<f32>()
                    .unwrap()
                    / 1000.0;

                self.set_temperature_has_changed();
                if self.should_collect_for_sampling()
                 && self.temperatures_collected_for_rate.len() < SAMPLING_SIZE {
                    info!("Collecting temperature for sampling");
                    self.temperatures_collected_for_rate.push((Utc::now(), self.current_temperature));
                }
                info!("Current Temperature {}", self.current_temperature);
                std::thread::sleep(std::time::Duration::from_secs(QUERY_DELAY_TIME_IN_SECONDS));
            }

            pub fn is_temperature_back_to_normal(&self) -> bool {
                self.temperature_back_to_normal
            }

            pub fn get_temperature_has_changed(&self) -> bool {
                self.temperature_has_changed
            }

            pub fn reset_temperature_back_to_normal(&mut self) {
                self.temperature_back_to_normal = false;
            }

            pub fn should_collect_data(&self) -> bool {
                self.current_temperature != self.last_temperature
            }

            pub fn is_sampling_ready(&mut self) -> bool {
                let mut is_sampling_ready = false;
                if self.should_collect_for_sampling()
                 && self.temperatures_collected_for_rate.len() > SAMPLING_SIZE {
                    is_sampling_ready = true;
                }
                info!("Is sampling ready: {}", is_sampling_ready);
                is_sampling_ready
            }

            pub fn get_cooling_rate_per_sec(&mut self) -> f32{
                let first_datetime_temperature = self.temperatures_collected_for_rate.first().unwrap();
                let last_datetime_temperature = self.temperatures_collected_for_rate.last().unwrap();

                let time_difference = last_datetime_temperature.0.signed_duration_since(first_datetime_temperature.0).num_seconds() as f32;
                let temperature_difference = last_datetime_temperature.1 - first_datetime_temperature.1;

                return temperature_difference / time_difference;
            }

            pub fn flush(&mut self) {
                self.temperatures_collected_for_rate.clear()
            }

            fn should_collect_for_sampling(&self) -> bool {
                self.current_temperature > self.temperature_threshold as f32
                 && self.current_temperature < self.last_temperature
            }

            fn set_temperature_has_changed(&mut self) {
                let new_temperature_has_changed =
                    self.current_temperature as u8 > self.temperature_threshold;
                debug!(
                    "Temperature has changed: {} -> {}",
                    self.temperature_has_changed, new_temperature_has_changed
                );
                if self.temperature_has_changed && !new_temperature_has_changed {
                    info!("Temperature back to normal");
                    self.temperature_back_to_normal = true;
                }
                self.temperature_has_changed = new_temperature_has_changed;
            }

            fn get_temperature_filepath() -> Result<String, String> {
                let directories = fs::read_dir(BASE_DIR_TEMPERATURE_SENSOR).unwrap();
                let mut base_directory_copy = String::from(BASE_DIR_TEMPERATURE_SENSOR);

                for directory in directories {
                    let directory = directory.unwrap();
                    let directory_name = directory.file_name().into_string().unwrap();

                    if directory_name.starts_with("28-") {
                        base_directory_copy.push_str(&directory_name);
                        base_directory_copy.push_str("/temperature");
                        info!("Found temperature sensor at {}\n", base_directory_copy);
                        return Ok(base_directory_copy);
                    }
                }
                Err(String::from("Unable to find temperature sensor"))
            }

        }
    }
}
