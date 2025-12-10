use crate::parameters::{Ev3Result, SensorPort};

pub struct UltrasonicSensor {
    sensor: ev3dev_lang_rust::sensors::UltrasonicSensor,
}

impl UltrasonicSensor {
    pub fn new(port: SensorPort) -> Ev3Result<Self> {
        Ok(Self {
            sensor: ev3dev_lang_rust::sensors::UltrasonicSensor::get(port)?,
        })
    }

    pub fn distance(&self) -> Ev3Result<i32> {
        self.sensor.get_distance()
    }

    pub fn distance_cm(&self) -> Ev3Result<f32> {
        self.sensor.get_distance_centimeters()
    }

    pub fn distance_in(&self) -> Ev3Result<f32> {
        self.sensor.get_distance_inches()
    }
}
