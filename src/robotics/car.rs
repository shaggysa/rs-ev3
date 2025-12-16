use ev3dev_lang_rust::Ev3Result;

use crate::pupdevices::Motor;

pub struct Car {
    steering_motor: Motor,
    drive_motor: Motor,
    left_max: i32,
    right_max: i32,
}

impl Car {
    pub async fn new(steering_motor: Motor, drive_motor: Motor) -> Ev3Result<Self> {
        steering_motor.run_until_stalled(-250).await?;

        let left_max = steering_motor.get_angle()?;

        steering_motor.run_until_stalled(250).await?;

        let right_max = steering_motor.get_angle()?;

        Ok(Self {
            steering_motor,
            drive_motor,
            left_max,
            right_max,
        })
    }

    pub async fn steer(&self, percentage: u8) -> Ev3Result<()> {
        self.steering_motor.run_target((self.left_max + self.right_max)/2*, 250)
    }
}
