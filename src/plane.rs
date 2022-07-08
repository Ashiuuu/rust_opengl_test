use crate::{
    draw::Draw,
    macros::offset_of,
    mesh::Vertex,
    shader_program::ShaderProgram,
    texture::Texture2D,
    utils::active_texture,
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
    pub texture: Option<Texture2D>,
    vao: VAO,
    vbo: VBO,
}

impl Plane {
    pub fn new(texture: Option<Texture2D>) -> Self {
        let plane = Self {
            texture,
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
        active_texture(GL_TEXTURE0);
        shader.set_int("texture_diffuse1", 0);
        self.texture.as_ref().map(|t| t.bind());

        self.vao.bind();

        unsafe {
            glDrawArrays(GL_TRIANGLES, 0, 6);
        }

        VAO::clear_binding();
    }
}
