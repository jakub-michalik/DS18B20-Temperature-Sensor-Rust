#[cfg(feature = "host")]
use rand::Rng;

pub struct DS18B20Sensor {
    temperature: f32,
}

impl DS18B20Sensor {
    pub fn new() -> DS18B20Sensor {
        DS18B20Sensor {
            temperature: 0.0,
        }
    }

    pub fn read(&mut self) {
        #[cfg(feature = "host")]
        {
            let mut rng = rand::thread_rng();
            self.temperature = rng.gen_range(-55.0..125.0);
        }
        #[cfg(feature = "cortex")]
        {
            // read the sensor using the hardware peripherals on the Cortex-M
        }
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}
