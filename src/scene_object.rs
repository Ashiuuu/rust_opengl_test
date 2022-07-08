use crate::{
    draw::Draw, model::Model, plane::Plane, shader_program::ShaderProgram, texture::Texture2D,
    transform::Transform,
};

use glm::{Mat4, TVec3};

pub struct SceneObject {
    object: Box<dyn Draw>,
    pub transform: Transform,
}

impl SceneObject {
    pub fn model(path: &str) -> Self {
        Self {
            object: Box::new(Model::new(path)),
            transform: Transform::new(),
        }
    }

    pub fn plane(texture: Option<Texture2D>) -> Self {
        Self {
            object: Box::new(Plane::new(texture)),
            transform: Transform::new(),
        }
    }

    pub fn position(&self) -> TVec3<f32> {
        self.transform.position
    }

    pub fn set_position(&mut self, pos: TVec3<f32>) {
        self.transform.position = pos
    }

    pub fn roll(&self) -> f32 {
        self.transform.angles.x
    }

    pub fn set_roll(&mut self, value: f32) {
        self.transform.angles.x = value
    }

    pub fn pitch(&self) -> f32 {
        self.transform.angles.y
    }

    pub fn set_pitch(&mut self, value: f32) {
        self.transform.angles.y = value
    }

    pub fn yaw(&self) -> f32 {
        self.transform.angles.z
    }

    pub fn set_yaw(&mut self, value: f32) {
        self.transform.angles.z = value
    }

    pub fn set_scale(&mut self, scale: TVec3<f32>) {
        self.transform.scale = scale
    }

    pub fn model_matrix(&self) -> Mat4 {
        self.transform.matrix()
    }

    pub fn draw(&self, shader: &ShaderProgram) {
        shader.set_mat4("model", &self.model_matrix());
        self.object.draw(shader)
    }
}
