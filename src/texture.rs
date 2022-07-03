use image::io::Reader as ImageReader;

use derive_more::Display;
use gl33::{global_loader::*, *};

use super::glenum_to_i32;

#[derive(Display, Clone)]
pub enum TextureType {
    Diffuse,
    Specular,
    Emission,
    Ambient,
    Normal,
}

#[derive(Clone)]
pub struct Texture2D {
    id: u32,
    pub ty: TextureType,
    pub path: String,
}

impl Texture2D {
    pub fn from_image(path: &str, directory: &str, ty: TextureType) -> Self {
        //let filename = format!("{}\\{}", directory, path);
        let filename = format!("{}/{}", directory, path);

        let image = ImageReader::open(filename).unwrap().decode().unwrap();
        //.flipv();
        let format = match image {
            image::DynamicImage::ImageLuma8(_) => GL_RED,
            image::DynamicImage::ImageLumaA8(_) => GL_RG,
            image::DynamicImage::ImageRgb8(_) => GL_RGB,
            image::DynamicImage::ImageRgba8(_) => GL_RGBA,
            _ => panic!("Unsupported image format"),
        };

        let mut id = 0;
        unsafe {
            glGenTextures(1, &mut id);
            assert_ne!(id, 0);

            glBindTexture(GL_TEXTURE_2D, id);
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                glenum_to_i32(format),
                image.width().try_into().unwrap(),
                image.height().try_into().unwrap(),
                0,
                format,
                GL_UNSIGNED_BYTE,
                image.as_bytes().as_ptr().cast(),
            );
            glGenerateMipmap(GL_TEXTURE_2D);

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, glenum_to_i32(GL_REPEAT));
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, glenum_to_i32(GL_REPEAT));
            glTexParameteri(
                GL_TEXTURE_2D,
                GL_TEXTURE_MIN_FILTER,
                glenum_to_i32(GL_LINEAR_MIPMAP_LINEAR),
            );
            glTexParameteri(
                GL_TEXTURE_2D,
                GL_TEXTURE_MAG_FILTER,
                glenum_to_i32(GL_LINEAR),
            );
        }

        Self {
            id,
            ty,
            path: path.to_owned(),
        }
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
