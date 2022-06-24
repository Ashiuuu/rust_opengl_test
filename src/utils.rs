use {
    gl33::{global_loader::*, *},
    std::f32::consts::PI,
};

pub fn to_radians(e: f32) -> f32 {
    e * PI / 180.0
}

pub fn clear_color(r: f32, g: f32, b: f32) {
    unsafe {
        glClearColor(r, g, b, 1.0);
    }
}

pub fn glenum_to_i32(e: GLenum) -> i32 {
    match e {
        GL_NEAREST => 0x2600,
        GL_LINEAR => 0x2601,
        GL_LINEAR_MIPMAP_LINEAR => 0x2703,
        GL_RGB => 0x1907,
        GL_CLAMP_TO_EDGE => 0x812F,
        GL_REPEAT => 0x2901,
        _ => panic!("Don't call into for GLenum variant {:?}", e),
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
        0x84c9 => GL_TEXTURE0,
        _ => unimplemented!(),
    }
}
