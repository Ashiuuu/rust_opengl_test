use {
    gl33::{global_loader::*, *},
    glm::{TVec2, TVec3},
    std::mem::{size_of, MaybeUninit},
    std::ptr,
};

use glm::{vec2, vec3};

use crate::{
    macros::*,
    shader_program::ShaderProgram,
    texture::{Texture2D, TextureType},
    utils::usize_to_glenum,
    vertex_objects::{BufferType, VAOCreationFail, VBOCreationFail, VAO, VBO},
};

#[repr(C)]
pub struct Vertex {
    pub position: TVec3<f32>,
    pub normal: TVec3<f32>,
    pub tex_coords: TVec2<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            tex_coords: vec2(0.0, 0.0),
        }
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture2D>,
    vao: VAO,
    vbo: VBO,
    ebo: VBO,
}

#[derive(Debug)]
pub enum MeshCreationFail {
    VAOError,
    VBOError,
    EBOError,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture2D>) -> Self {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao: VAO::new().unwrap(),
            vbo: VBO::new(BufferType::Array).unwrap(),
            ebo: VBO::new(BufferType::ElementArray).unwrap(),
        };

        mesh.setup_mesh();
        mesh
    }

    fn setup_mesh(&mut self) {
        self.vao.bind();

        self.vbo.bind();
        unsafe {
            glBufferData(
                GL_ARRAY_BUFFER,
                (self.vertices.len() * size_of::<Vertex>())
                    .try_into()
                    .unwrap(),
                self.vertices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }

        self.ebo.bind();
        unsafe {
            glBufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * size_of::<u32>()).try_into().unwrap(),
                self.indices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }

        unsafe {
            let stride: i32 = size_of::<Vertex>().try_into().unwrap();

            glEnableVertexAttribArray(0);
            glVertexAttribPointer(0, 3, GL_FLOAT, 0, stride, 0 as *const _);

            glEnableVertexAttribArray(1);
            glVertexAttribPointer(
                1,
                3,
                GL_FLOAT,
                0,
                stride,
                offset_of!(Vertex, normal) as *const _,
            );

            glEnableVertexAttribArray(2);
            glVertexAttribPointer(
                2,
                2,
                GL_FLOAT,
                0,
                stride,
                offset_of!(Vertex, tex_coords) as *const _,
            );
        }

        VAO::clear_binding();
    }

    pub fn draw(&self, shader: &ShaderProgram) {
        let mut diffuse_n = 0;
        let mut specular_n = 0;

        for (i, texture) in self.textures.iter().enumerate() {
            unsafe {
                glActiveTexture(usize_to_glenum(0x84c0 + i));
            }

            let name = &texture.ty;
            let number = match name {
                TextureType::Diffuse => {
                    diffuse_n += 1;
                    diffuse_n
                }
                TextureType::Specular => {
                    specular_n += 1;
                    specular_n
                }
                _ => panic!("Unknown texture type"),
            };

            shader.set_int(&format!("{}{}", name, number), i as i32);
            texture.bind();
        }

        self.vao.bind();

        unsafe {
            glDrawElements(
                GL_TRIANGLES,
                self.indices.len().try_into().unwrap(),
                GL_UNSIGNED_INT,
                0 as *const _,
            );
            glActiveTexture(GL_TEXTURE0);
        }

        VAO::clear_binding();
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
