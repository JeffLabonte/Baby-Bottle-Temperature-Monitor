use std::fs;

use log::info;

const BASE_DIR_TEMPERATURE_SENSOR: &str = "/sys/bus/w1/devices/";

pub struct WaterTemperatureSensor {
    temperature_filepath: String,
    current_temperature: f32,
    temperature_threshold: f32,
    temperature_has_changed: bool,
    temperature_back_to_normal: bool,
}

impl WaterTemperatureSensor {
    pub fn new() -> Self {
        WaterTemperatureSensor {
            temperature_filepath: WaterTemperatureSensor::get_temperature_filepath().unwrap(),
            current_temperature: 0.0,
            temperature_threshold: 30.0,
            temperature_has_changed: false,
            temperature_back_to_normal: false,
        }
    }

    pub fn read(&mut self) -> f32 {
        info!("Reading temperature from {}\n", self.temperature_filepath);
        self.current_temperature = fs::read_to_string(&self.temperature_filepath)
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap()
            / 1000.0;

        self.set_temperature_has_changed();
        self.current_temperature
    }

    pub fn is_temperature_back_to_normal(&self) -> bool {
        self.temperature_back_to_normal
    }

    pub fn reset_temperature_back_to_normal(&mut self) {
        self.temperature_back_to_normal = false;
    }

    fn set_temperature_has_changed(&mut self) {
        let new_temperature_has_changed = self.current_temperature > self.temperature_threshold;
        if self.temperature_has_changed && !new_temperature_has_changed {
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
