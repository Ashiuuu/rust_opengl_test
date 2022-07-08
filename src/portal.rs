use gl33::*;

use crate::{
    active_texture, framebuffer::Framebuffer, scene_object::SceneObject,
    shader_program::ShaderProgram, static_camera::StaticCamera,
};

pub struct Portal {
    pub surface: SceneObject,
    pub camera: StaticCamera,
    framebuffer: Framebuffer,
}

impl Portal {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        Self {
            surface: SceneObject::plane(None),
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
