use glm::{cross, look_at, modf, normalize, vec3, TMat4, TVec3};

use crate::to_radians;
use crate::MovementState;

pub struct Camera {
    position: TVec3<f32>,
    front: TVec3<f32>,
    right: TVec3<f32>,
    up: TVec3<f32>,
    pitch: f32,
    yaw: f32,
    speed: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        let right = normalize(&cross(&vec3(0.0, 0.0, -1.0), &vec3(0.0, 1.0, 0.0)));
        Self {
            position: vec3(0.0, 0.0, 3.0),
            front: vec3(0.0, 0.0, -1.0),
            right,
            up: normalize(&cross(&right, &vec3(0.0, 0.0, -1.0))),
            pitch: 0.0,
            yaw: -90.0,
            speed: 2.5,
            sensitivity: 0.5,
        }
    }

    fn set_yaw(&mut self, value: f32) {
        self.yaw = modf(self.yaw + value * self.sensitivity, 360.0);
    }

    fn set_pitch(&mut self, value: f32) {
        self.pitch -= value * self.sensitivity;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        } else if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
    }

    pub fn update_orientation(&mut self, dx: f32, dy: f32) {
        self.set_yaw(dx);
        self.set_pitch(dy);

        self.front.x = to_radians(self.yaw).cos() * to_radians(self.pitch).cos();
        self.front.y = to_radians(self.pitch).sin();
        self.front.z = to_radians(self.yaw).sin() * to_radians(self.pitch).cos();
        self.front = normalize(&self.front);

        self.right = normalize(&cross(&self.front, &vec3(0.0, 1.0, 0.0)));
        self.up = normalize(&cross(&self.right, &self.front));
    }

    fn front_movement(&mut self, dt: f32) {
        self.position += self.speed * dt * self.front;
    }

    fn right_movement(&mut self, dt: f32) {
        self.position += self.right * self.speed * dt;
    }

    pub fn update_movement(&mut self, movement_state: &MovementState, dt: f32) {
        if movement_state.forward.is_pressed() {
            self.front_movement(dt);
        } else if movement_state.backward.is_pressed() {
            self.front_movement(-dt);
        }
        if movement_state.right.is_pressed() {
            self.right_movement(dt);
        } else if movement_state.left.is_pressed() {
            self.right_movement(-dt);
        }
    }

    pub fn view_matrix(&self) -> TMat4<f32> {
        look_at(&self.position, &(self.position + self.front), &self.up)
    }
}
