use glm::{quat_to_mat4, scale, translate, vec3, Mat4};
use nalgebra::UnitQuaternion;
use nalgebra_glm::TVec3;

use crate::utils::{to_radians, IDENTITY_MAT4};

pub struct Transform {
    pub position: TVec3<f32>,
    pub angles: TVec3<f32>,
    pub scale: TVec3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 0.0, 0.0),
            angles: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }

    fn rotation(&self) -> Mat4 {
        let quat = UnitQuaternion::from_euler_angles(
            to_radians(self.angles.x),
            to_radians(self.angles.y),
            to_radians(self.angles.z),
        );
        quat_to_mat4(&quat)
    }

    pub fn matrix(&self) -> Mat4 {
        let scale_matrix = scale(&IDENTITY_MAT4, &self.scale);
        let translation_matrix = translate(&scale_matrix, &self.position);
        self.rotation() * translation_matrix
    }

    pub fn mut_angles(&mut self) -> Vec3Mut {
        self.angles.as_vec3_mut()
    }
}

pub struct Vec3Mut<'a> {
    pub x: &'a mut f32,
    pub y: &'a mut f32,
    pub z: &'a mut f32,
}

pub trait AsVec3Mut {
    fn as_vec3_mut(&mut self) -> Vec3Mut;
}

impl AsVec3Mut for TVec3<f32> {
    fn as_vec3_mut(&mut self) -> Vec3Mut {
        Vec3Mut {
            x: &mut self.x,
            y: &mut self.y,
            z: &mut self.z,
        }
    }
}
