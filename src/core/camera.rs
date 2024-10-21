use crate::core::common::Ray;
use crate::input::{MoveDirection, SceneAction};
use cgmath::{Deg, InnerSpace, Quaternion, Rad, Rotation, Rotation3, Vector2, Vector3, Zero};
use derive_builder::Builder;
use std::f32::consts::PI;
use std::time::Duration;
use crate::core::movement::MovementComponent;

#[derive(Builder)]
pub(crate) struct Camera {
    movement: MovementComponent,
    pitch: Rad<f32>,
    yaw: Rad<f32>,
    fov: Rad<f32>,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
}

impl Camera {
    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {

        self.yaw += Rad(-delta_x);
        self.pitch += Rad(-delta_y);

        // Clamp the pitch angle to prevent flipping
        const RIGHT_ANGLE: f32 = 90.;
        let pitch_limit = Rad((RIGHT_ANGLE - 1.).to_radians());
        if self.pitch > pitch_limit {
            self.pitch = pitch_limit;
        }
        if self.pitch < -pitch_limit {
            self.pitch = -pitch_limit;
        }
    }

    pub fn get_ray(&self, pixel_in_screen_space: Vector2<f32>) -> Ray {
        let scale = self.scale();
        let x_scaled = pixel_in_screen_space.x * self.aspect_ratio * scale;
        let y_scaled = pixel_in_screen_space.y * scale;

        let dir_camera_space = Vector3::new(x_scaled, y_scaled, -1.0).normalize();
        let dir_world_space = self.rotation().rotate_vector(dir_camera_space);

        Ray::new(self.movement.position, dir_world_space)
    }

    pub fn scale(&self) -> f32 {
        (self.fov.0 / 2.0).tan()
    }
    fn direction_to_movement(&self, direction: MoveDirection) -> Vector3<f32> {
        let rotation = self.rotation();

        let vec = match direction {
            MoveDirection::Forward => Vector3 { x: 0.0, y: 0.0, z: -1.0 },
            MoveDirection::Left => Vector3 { x: -1.0, y: 0.0, z: 0.0 },
            MoveDirection::Backward => Vector3 { x: 0.0, y: 0.0, z: 1.0 },
            MoveDirection::Right => Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            MoveDirection::Up => Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            MoveDirection::Down => Vector3 { x: 0.0, y: -1.0, z: 0.0 },
        };

        rotation.rotate_vector(vec)
    }
    pub fn tick(&mut self, actions: impl Iterator<Item=SceneAction>, dt: &Duration) {
        let mut vel = Vector3::zero();
        for action in actions {
            match action {
                SceneAction::RotateCamera { delta } => {
                    let delta_in_screen_space = delta.map(|x| x as f32) / 1000.;
                    let sensitivity = 2.;
                    let yaw_change = delta_in_screen_space.y * sensitivity * self.scale();
                    let pitch_change = -delta_in_screen_space.x * sensitivity;
                    self.rotate(yaw_change, pitch_change);
                }
                SceneAction::Move(dir) => vel += self.direction_to_movement(dir)
            };
        }

        self.movement.set_velocity(vel * self.movement.max_speed);
        self.movement.tick(&dt)
    }

    pub fn from_position_and_target(position: Vector3<f32>, look_at: Vector3<f32>) -> Self {
        // Default parameters
        let fov = Deg(45.0).into();
        let aspect_ratio = 16.0 / 9.0;
        let near_plane = 0.1;
        let far_plane = 100.0;

        let forward = (look_at - position).normalize();

        let yaw = Rad(forward.x.atan2(forward.z) + PI);
        let pitch = Rad(-forward.y.asin());

        Camera {
            movement: MovementComponent::new(position),
            yaw,
            pitch,
            fov,
            aspect_ratio,
            near_plane,
            far_plane,
        }
    }

    fn rotation(&self) -> Quaternion<f32> {
        let yaw_quat = Quaternion::from_angle_y(self.yaw);
        let pitch_quat = Quaternion::from_angle_x(self.pitch);
        yaw_quat * pitch_quat
    }
}