use glm::{look_at, vec3, Mat4, TVec3};

pub struct StaticCamera {
    pub pos: TVec3<f32>,
    pub look_at: TVec3<f32>,
}

impl StaticCamera {
    pub fn new() -> Self {
        Self {
            pos: vec3(0.0, 0.0, 0.0),
            look_at: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        look_at(&self.pos, &self.look_at, &vec3(0.0, 1.0, 0.0))
    }
}
