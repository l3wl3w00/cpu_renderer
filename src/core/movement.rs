use cgmath::{InnerSpace, Vector3, Zero};
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct MovementComponent {
    pub position: Vector3<f32>,
    velocity: Vector3<f32>,
    pub max_speed: f32,
}

impl MovementComponent {
    pub fn new(initial_position: Vector3<f32>) -> MovementComponent {
        MovementComponent {
            position: initial_position, velocity: Vector3::zero(), max_speed: 5.0,
        }
    }

    pub fn set_velocity(&mut self, delta_velocity: Vector3<f32>) {
        self.velocity = delta_velocity;
    }

    pub fn tick(&mut self, dt: &Duration) {
        if self.velocity.magnitude() > self.max_speed + f32::EPSILON {
            self.velocity = self.velocity.normalize() * self.max_speed;
        }

        self.position += self.velocity * dt.as_secs_f32();
    }
}