pub use ev3dev_lang_rust::motors::MotorPort;
pub use ev3dev_lang_rust::sensors::SensorPort;
pub use ev3dev_lang_rust::{Ev3Error, Ev3Result};
#[derive(PartialEq)]
pub enum Direction {
    ClockWise,
    CounterClockWise,
}
