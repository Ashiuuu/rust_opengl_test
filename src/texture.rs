use std::fmt::Display;

use image::{io::Reader as ImageReader, ColorType};

use gl33::{global_loader::*, *};

use super::glenum_to_i32;

pub enum TextureType {
    Diffuse,
    Specular,
    Emission,
}

impl Display for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Diffuse => "diffuse",
                Self::Specular => "specular",
                Self::Emission => "emission",
            }
        )
    }
}

pub struct Texture2D {
    id: u32,
    pub ty: TextureType,
}

impl Texture2D {
    pub fn from_image(filename: &str, ty: TextureType) -> Self {
        let image = ImageReader::open(filename)
            .unwrap()
            .decode()
            .unwrap()
            .flipv();

        let mut id = 0;
        unsafe {
            glGenTextures(1, &mut id);
            assert_ne!(id, 0);

            glBindTexture(GL_TEXTURE_2D, id);
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                glenum_to_i32(GL_RGB),
                image.width().try_into().unwrap(),
                image.height().try_into().unwrap(),
                0,
                match image.color() {
                    ColorType::Rgb8 => GL_RGB,
                    ColorType::Rgba8 => GL_RGBA,
                    _ => panic!("Can't convert color type {:?}", image.color()),
                },
                GL_UNSIGNED_BYTE,
                image.as_bytes().as_ptr().cast(),
            );
            glGenerateMipmap(GL_TEXTURE_2D);
        }

        Self { id, ty }
    }

    pub fn bind(&self) {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.id);
        }
    }

    pub fn clear_binding() {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, 0);
        }
    }
}
