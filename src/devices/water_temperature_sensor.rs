use std::fs;

const BASE_DIR_TEMPERATURE_SENSOR: &str = "/sys/bus/w1/devices/";

pub struct WaterTemperatureSensor {
    temperature_filepath: String,
}

impl WaterTemperatureSensor {
    pub fn new() -> Self {
        WaterTemperatureSensor {
            temperature_filepath: WaterTemperatureSensor::get_temperature_filepath().unwrap(),
        }
    }

    pub fn read(&mut self) -> f32 {
        fs::read_to_string(&self.temperature_filepath)
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap()
            / 1000.0
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
                println!("Found temperature sensor at {}", base_directory_copy);
                return Ok(base_directory_copy);
            }
        }
        Err(String::from("Unable to find temperature sensor"))
    }
}
