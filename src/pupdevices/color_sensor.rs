use crate::parameters::{Ev3Result, SensorPort};

pub struct ColorSensor {
    sensor: ev3dev_lang_rust::sensors::ColorSensor,
}

impl ColorSensor {
    pub fn new(port: SensorPort) -> Ev3Result<Self> {
        Ok(Self {
            sensor: ev3dev_lang_rust::sensors::ColorSensor::get(port)?,
        })
    }

    pub fn get_color(&self) -> Ev3Result<i32> {
        self.sensor.get_color()
    }
}
