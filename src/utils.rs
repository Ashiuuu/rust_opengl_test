use {
    gl33::{global_loader::*, *},
    glm::Mat4,
    lazy_static::lazy_static,
    std::f32::consts::PI,
};

lazy_static! {
    pub static ref IDENTITY_MAT4: Mat4 =
        glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,);
}

pub fn to_radians(e: f32) -> f32 {
    e * PI / 180.0
}

pub fn clear_color(r: f32, g: f32, b: f32) {
    unsafe {
        glClearColor(r, g, b, 1.0);
    }
}

pub fn gl_clear(bits: GLbitfield) {
    unsafe {
        glClear(bits);
    }
}

pub fn active_texture(tex: GLenum) {
    unsafe {
        glActiveTexture(tex);
    }
}

pub fn glenum_to_i32(e: GLenum) -> i32 {
    match e {
        GL_RED => 0x1903,
        GL_RGB => 0x1907,
        GL_NEAREST => 0x2600,
        GL_LINEAR => 0x2601,
        GL_LINEAR_MIPMAP_LINEAR => 0x2703,
        GL_REPEAT => 0x2901,
        GL_CLAMP_TO_EDGE => 0x812F,
        _ => panic!("Don't call for GLenum variant {:?}", e),
    }
}

pub fn usize_to_glenum(e: usize) -> GLenum {
    match e {
        0x84c0 => GL_TEXTURE0,
        0x84c1 => GL_TEXTURE1,
        0x84c2 => GL_TEXTURE2,
        0x84c3 => GL_TEXTURE3,
        0x84c4 => GL_TEXTURE4,
        0x84c5 => GL_TEXTURE5,
        0x84c6 => GL_TEXTURE6,
        0x84c7 => GL_TEXTURE7,
        0x84c8 => GL_TEXTURE8,
        0x84c9 => GL_TEXTURE9,
        _ => unimplemented!(),
    }
}
