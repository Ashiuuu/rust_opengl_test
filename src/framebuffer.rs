use gl33::{global_loader::*, *};

use crate::{glenum_to_i32, Texture2D};

pub struct Framebuffer {
    fbo: u32,
    texture: u32,
    rbo: u32,
}

impl Framebuffer {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        let mut ret = Self {
            fbo: 0,
            texture: 0,
            rbo: 0,
        };

        unsafe {
            glGenFramebuffers(1, &mut ret.fbo);
            glBindFramebuffer(GL_FRAMEBUFFER, ret.fbo);
            glGenTextures(1, &mut ret.texture);
            glBindTexture(GL_TEXTURE_2D, ret.texture);

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                glenum_to_i32(GL_RGB),
                window_width,
                window_height,
                0,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                0 as *const _,
            );

            glTexParameteri(
                GL_TEXTURE_2D,
                GL_TEXTURE_MIN_FILTER,
                glenum_to_i32(GL_LINEAR),
            );
            glTexParameteri(
                GL_TEXTURE_2D,
                GL_TEXTURE_MAG_FILTER,
                glenum_to_i32(GL_LINEAR),
            );

            glFramebufferTexture2D(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                GL_TEXTURE_2D,
                ret.texture,
                0,
            );
            Texture2D::clear_binding();

            glGenRenderbuffers(1, &mut ret.rbo);
            glBindRenderbuffer(GL_RENDERBUFFER, ret.rbo);
            glRenderbufferStorage(
                GL_RENDERBUFFER,
                GL_DEPTH24_STENCIL8,
                window_width.try_into().unwrap(),
                window_height.try_into().unwrap(),
            );
            glBindRenderbuffer(GL_RENDERBUFFER, 0);

            glFramebufferRenderbuffer(
                GL_FRAMEBUFFER,
                GL_DEPTH_STENCIL_ATTACHMENT,
                GL_RENDERBUFFER,
                ret.rbo,
            );

            match glCheckFramebufferStatus(GL_FRAMEBUFFER) {
                GL_FRAMEBUFFER_COMPLETE => (),
                _ => println!("Framebuffer is not complete"),
            };
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }

        ret
    }

    pub fn bind(&self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.fbo);
        }
    }

    pub fn bind_texture(&self) {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.texture);
        }
    }

    pub fn clear_binding() {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }
    }
}
