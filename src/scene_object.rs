use crate::{
    draw::Draw,
    shader_program::ShaderProgram,
    utils::{to_radians, IDENTITY_MAT4},
};

use glm::{vec3, Mat4, TVec3};

pub struct SceneObject<'a> {
    object: Box<dyn Draw + 'a>,
    pos: TVec3<f32>,
    angle: f32,
    rot_axis: TVec3<f32>,
    scale: TVec3<f32>,
}

impl<'a> SceneObject<'a> {
    pub fn new(object: impl Draw + 'a) -> Self {
        Self {
            object: Box::new(object),
            pos: vec3(0.0, 0.0, 0.0),
            angle: 0.0,
            rot_axis: vec3(0.662, 0.2, 0.722),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn position(&self) -> TVec3<f32> {
        self.pos
    }

    pub fn set_position(&mut self, pos: TVec3<f32>) {
        self.pos = pos
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle
    }

    pub fn set_rotation_axis(&mut self, axis: TVec3<f32>) {
        self.rot_axis = axis
    }

    pub fn set_scale(&mut self, scale: TVec3<f32>) {
        self.scale = scale
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

//impl<'a> Deref for SceneObject<'a> {
//type Target = dyn Draw + 'a;

//fn deref(&self) -> &Self::Target {
//&(*self.object)
//}
//}
