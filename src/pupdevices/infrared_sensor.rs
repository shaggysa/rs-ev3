use crate::parameters::{Ev3Result, SensorPort};

pub struct InfraredSensor {
    sensor: ev3dev_lang_rust::sensors::InfraredSensor,
}

impl InfraredSensor {
    pub fn new(port: SensorPort) -> Ev3Result<Self> {
        Ok(Self {
            sensor: ev3dev_lang_rust::sensors::InfraredSensor::get(port)?,
        })
    }

    pub fn distance(&self) -> Ev3Result<i32> {
        self.sensor.get_distance()
    }
}
