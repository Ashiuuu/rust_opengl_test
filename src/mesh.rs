use {
    gl33::{global_loader::*, *},
    glm::{TVec2, TVec3},
    std::mem::{size_of, size_of_val, MaybeUninit},
    std::ptr,
};

use crate::{
    macros::*,
    shader_program::ShaderProgram,
    texture::{Texture2D, TextureType},
    usize_to_glenum,
    vertex_objects::{BufferType, VAOCreationFail, VBOCreationFail, VAO, VBO},
};

pub struct Vertex {
    position: TVec3<f32>,
    normal: TVec3<f32>,
    tex_coords: TVec2<f32>,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture2D>,
    vao: VAO,
    vbo: VBO,
    ebo: VBO,
}

pub enum MeshCreationFail {
    VAOError,
    VBOError,
    EBOError,
    ConversionFail,
}

pub enum DrawFail {
    TextureOutOfBounds,
    unimplementedTexture,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        textures: Vec<Texture2D>,
    ) -> Result<Self, MeshCreationFail> {
        let vao = VAO::new()?;
        let vbo = VBO::new(BufferType::Array)?;
        let ebo = VBO::new(BufferType::ElementArray)?;

        vao.bind();
        vbo.bind();

        unsafe {
            glBufferData(
                GL_ARRAY_BUFFER,
                size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }

        ebo.bind();

        unsafe {
            glBufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                size_of_val(&indices) as isize,
                indices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }

        unsafe {
            let stride: i32 = size_of::<Vertex>().try_into().unwrap(); // shouldn't fail unless the structure is very very big
            let normal_offset = offset_of!(Vertex, normal);
            let tex_coords_offset = offset_of!(Vertex, tex_coords);

            glEnableVertexAttribArray(0);
            glVertexAttribPointer(0, 3, GL_FLOAT, 0, stride, 0 as *const _);

            glEnableVertexAttribArray(1);
            glVertexAttribPointer(1, 3, GL_FLOAT, 0, stride, normal_offset as *const _);

            glEnableVertexAttribArray(2);
            glVertexAttribPointer(2, 2, GL_FLOAT, 0, stride, tex_coords_offset as *const _);
        }

        VBO::clear_binding();
        VAO::clear_binding();

        Ok(Self {
            vertices,
            indices,
            textures,
            vao,
            vbo,
            ebo,
        })
    }

    pub fn draw(&self, shader: &ShaderProgram) -> Result<(), DrawFail> {
        let mut diffuse_n = 1;
        let mut specular_n = 1;

        for i in 0..self.textures.len() {
            unsafe {
                glActiveTexture(usize_to_glenum(0x84c0 + i));
            }

            let name = match self.textures.get(i) {
                None => return Err(DrawFail::TextureOutOfBounds),
                Some(tex) => &tex.ty,
            };
            let number = match name {
                TextureType::Diffuse => {
                    let temp = diffuse_n;
                    diffuse_n += 1;
                    temp
                }
                TextureType::Specular => {
                    let temp = specular_n;
                    specular_n += 1;
                    temp
                }
                _ => return Err(DrawFail::unimplementedTexture),
            };

            shader.set_int(
                format!("{}{}{}", "material.", name, number).as_str(),
                i.try_into().unwrap(), // shouldn't fail until we have a high amount of texture
            );

            match self.textures.get(i) {
                None => return Err(DrawFail::TextureOutOfBounds),
                Some(tex) => tex.bind(),
            }
        }

        unsafe {
            glActiveTexture(GL_TEXTURE0);
            glDrawElements(
                GL_TRIANGLES,
                self.indices.len().try_into().unwrap(),
                GL_UNSIGNED_INT,
                0 as *const _,
            );
        }

        VAO::clear_binding();
        Ok(())
    }
}

impl From<VAOCreationFail> for MeshCreationFail {
    fn from(_: VAOCreationFail) -> Self {
        Self::VAOError
    }
}

impl From<VBOCreationFail> for MeshCreationFail {
    fn from(f: VBOCreationFail) -> Self {
        match f.0 {
            BufferType::Array => Self::VBOError,
            BufferType::ElementArray => Self::EBOError,
        }
    }
}
