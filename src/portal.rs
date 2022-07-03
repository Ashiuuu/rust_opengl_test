use gl33::*;

use crate::{
    active_texture, framebuffer::Framebuffer, plane::Plane, scene_object::SceneObject,
    shader_program::ShaderProgram, static_camera::StaticCamera,
};

pub struct Portal<'a> {
    pub surface: SceneObject<'a>,
    pub camera: StaticCamera,
    framebuffer: Framebuffer,
}

impl<'a> Portal<'a> {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        Self {
            surface: SceneObject::new(Plane::new(vec![])),
            framebuffer: Framebuffer::new(window_width, window_height),
            camera: StaticCamera::new(),
        }
    }

    pub fn bind_framebuffer(&self) {
        self.framebuffer.bind()
    }

    pub fn render(&self, shader: &ShaderProgram) {
        active_texture(GL_TEXTURE0);
        self.framebuffer.bind_texture();
        shader.set_int("texture_diffuse1", 0);
        self.surface.draw(&shader);
    }
}
