use glm::vec3;
use nalgebra_glm::TVec3;

pub struct Transform {
    pub position: TVec3<f32>,
    pub rotation_axis: TVec3<f32>,
    pub angles: TVec3<f32>,
    pub scale: TVec3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 0.0, 0.0),
            rotation_axis: vec3(0.0, 1.0, 0.0),
            angles: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}
