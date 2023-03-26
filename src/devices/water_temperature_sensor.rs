use gpio::{sysfs::SysFsGpioInput, GpioIn};

pub struct WaterTemperatureSensor {
    input_device: SysFsGpioInput,
}

impl WaterTemperatureSensor {
    pub fn new(pin: u16) -> Self {
        Self {
            input_device: SysFsGpioInput::open(pin).unwrap(),
        }
    }
}
