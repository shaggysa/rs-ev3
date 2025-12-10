use scopeguard::defer;
use std::time::Duration;

use crate::parameters::{Direction, Ev3Result, MotorPort};
use ev3dev_lang_rust::motors::TachoMotor;

pub struct Motor {
    motor: TachoMotor,
}

impl Motor {
    pub fn new(port: MotorPort, direction: Direction) -> Ev3Result<Self> {
        let motor = TachoMotor::get(port)?;
        if direction == Direction::CounterClockWise {
            motor.set_polarity("inversed")?;
        }
        Ok(Motor { motor })
    }

    async fn wait_for_stop(&self) -> Ev3Result<()> {
        defer! {
            _ = self.motor.stop();
        }

        while self.motor.is_running().is_ok_and(|f| f) {
            smol::Timer::after(Duration::from_millis(5)).await;
        }

        Ok(())
    }

    pub async fn run_angle(&self, angle: i32) -> Ev3Result<()> {
        self.motor.run_to_rel_pos(Some(angle))?;

        self.wait_for_stop().await
    }

    pub async fn run_target(&self, target: i32) -> Ev3Result<()> {
        self.motor.run_to_abs_pos(Some(target))?;

        self.wait_for_stop().await
    }

    pub async fn run_time(&self, time: Duration) -> Ev3Result<()> {
        self.motor.run_timed(Some(time))?;

        self.wait_for_stop().await
    }

    pub async fn run_until_stalled(&self) -> Ev3Result<()> {
        defer! {
            _ = self.motor.stop()
        }

        self.motor.run_forever()?;

        while self.motor.is_stalled().is_ok_and(|f| !f) {
            smol::Timer::after(Duration::from_millis(5)).await;
        }

        Ok(())
    }

    pub async fn run_while<F>(&self, condition: F) -> Ev3Result<()>
    where
        F: Fn() -> bool,
    {
        defer! {
            _ = self.motor.stop();
        }

        self.motor.run_forever()?;

        while condition() {
            smol::Timer::after(Duration::from_millis(5)).await;
        }

        Ok(())
    }
}
