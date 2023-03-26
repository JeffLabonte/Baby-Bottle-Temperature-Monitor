use gpio::{sysfs::SysFsGpioInput, GpioIn, GpioValue};

pub struct WaterTemperatureSensor {
    input_device: SysFsGpioInput,
}

impl WaterTemperatureSensor {
    pub fn new(pin: u16) -> Self {
        Self {
            input_device: SysFsGpioInput::open(pin).unwrap(),
        }
    }

    pub fn read(&mut self) -> GpioValue {
        self.input_device.read_value().unwrap()
    }
}
