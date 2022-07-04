use crate::{
    draw::Draw,
    shader_program::ShaderProgram,
    transform::Transform,
    utils::{to_radians, IDENTITY_MAT4},
};

use glm::{vec3, Mat4, TVec3};

pub struct SceneObject<'a> {
    object: Box<dyn Draw + 'a>,
    transform: Transform,
}

impl<'a> SceneObject<'a> {
    pub fn new(object: impl Draw + 'a) -> Self {
        Self {
            object: Box::new(object),
            transform: Transform::new(),
        }
    }

    pub fn position(&self) -> TVec3<f32> {
        self.transform.position
    }

    pub fn set_position(&mut self, pos: TVec3<f32>) {
        self.transform.position = pos
    }

    pub fn set_angle(&mut self, angles: TVec3<f32>) {
        self.transform.angles = angles
    }

    pub fn set_rotation_axis(&mut self, axis: TVec3<f32>) {
        self.transform.rotation_axis = axis
    }

    pub fn set_scale(&mut self, scale: TVec3<f32>) {
        self.transform.scale = scale
    }

    pub fn model_matrix(&self) -> Mat4 {
        glm::scale(
            &glm::translate(
                &glm::rotate(&IDENTITY_MAT4, to_radians(self.angle), &self.rot_axis),
                &self.pos,
            ),
            &self.scale,
        )
    }

    pub fn set_model_matrix(&self, shader: &ShaderProgram) {
        shader.set_mat4("model", &self.model_matrix())
    }

    pub fn draw(&self, shader: &ShaderProgram) {
        self.set_model_matrix(shader);
        self.object.draw(shader)
    }
}
