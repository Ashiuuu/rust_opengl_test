use crate::{
    draw::Draw,
    macros::offset_of,
    mesh::Vertex,
    shader_program::ShaderProgram,
    texture::{Texture2D, TextureType},
    utils::usize_to_glenum,
    vertex_objects::{BufferType, VAO, VBO},
};

use {
    core::mem::size_of,
    gl33::{global_loader::*, *},
    lazy_static::lazy_static,
};

lazy_static! {
    static ref UNIT_PLANE: [Vertex; 6] = [
        Vertex {
            position: glm::vec3(1.0, 1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
        Vertex {
            position: glm::vec3(-1.0, -1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vertex {
            position: glm::vec3(1.0, -1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(1.0, 0.0),
        },
        Vertex {
            position: glm::vec3(-1.0, 1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(0.0, 1.0),
        },
        Vertex {
            position: glm::vec3(-1.0, -1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(0.0, 0.0),
        },
        Vertex {
            position: glm::vec3(1.0, 1.0, 0.0),
            normal: glm::vec3(0.0, 0.0, -1.0),
            tex_coords: glm::vec2(1.0, 1.0),
        },
    ];
}

pub struct Plane {
    pub textures_loaded: Vec<Texture2D>,
    vao: VAO,
    vbo: VBO,
}

impl Plane {
    pub fn new(textures: Vec<Texture2D>) -> Self {
        let plane = Self {
            textures_loaded: textures,
            vao: VAO::new(),
            vbo: VBO::new(BufferType::Array),
        };

        plane.setup_plane();
        plane
    }

    fn setup_plane(&self) {
        self.vao.bind();
        self.vbo.bind();

        unsafe {
            glBufferData(
                GL_ARRAY_BUFFER,
                (UNIT_PLANE.len() * size_of::<Vertex>()).try_into().unwrap(),
                UNIT_PLANE.as_ptr().cast(),
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
    }
}

impl Draw for Plane {
    fn draw(&self, shader: &ShaderProgram) {
        let mut diffuse_n = 0;
        let mut specular_n = 0;

        for (i, texture) in self.textures_loaded.iter().enumerate() {
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
            glDrawArrays(GL_TRIANGLES, 0, 6);
            glActiveTexture(GL_TEXTURE0);
        }

        VAO::clear_binding();
    }
}
