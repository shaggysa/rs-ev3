use std::f32::consts::PI;

use ev3dev_lang_rust::Ev3Result;

use crate::pupdevices::{GyroSensor, Motor};
use pid::{ControlOutput, Pid};

pub struct DriveBase {
    left_motor: Motor,
    right_motor: Motor,
    wheel_diameter: f32,
    axle_track: f32,
    distance_pid: Pid<f32>,
    heading_pid: Pid<f32>,
    distance_target: i32,
    heading_target: f32,
    using_gyros: bool,
    gyros: Vec<GyroSensor>,
}

impl DriveBase {
    pub fn new(
        left_motor: Motor,
        right_motor: Motor,
        wheel_diameter: u32,
        axle_track: u32,
    ) -> Self {
        Self {
            left_motor,
            right_motor,
            wheel_diameter,
            axle_track,
            distance_pid: *Pid::new(0.0, 40.0).p(15.0, 40.0).i(0.5, 40.0).d(3.0, 40.0),
            heading_pid: *Pid::new(0.0, 40.0).p(3.0, 40.0).i(0.4, 40.0).d(1.5, 40.0),
            distance_target: 0,
            heading_target: 0.0,
            using_gyros: false,
            gyros: Vec::new(),
        }
    }

    pub fn add_gyro(&mut self, gyro: GyroSensor) {
        self.gyros.push(gyro);
    }

    pub fn use_gyro(&mut self, use_gyro: bool) {
        self.using_gyros = use_gyro;
    }

    pub fn straight(&mut self, distance: i32) -> Ev3Result<()> {
        let left_counts_per_rot = self.left_motor.motor.get_count_per_rot()?;
        let right_counts_per_rot = self.right_motor.motor.get_count_per_rot()?;

        let left_mm_per_count = left_counts_per_rot as f32 * 2.0 * PI * self.wheel_diameter;
        let right_mm_per_count = right_counts_per_rot as f32 * 2.0 * PI * self.wheel_diameter;

        let left_starting_counts = self.left_motor.motor.get_full_travel_count();
        let right_starting_counts = self.right_motor.motor.get_full_travel_count();



        loop {
            let
        }

        Ok(())
    }
}
