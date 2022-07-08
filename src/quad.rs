use {
    core::mem::size_of,
    gl33::{global_loader::*, *},
    glm::TVec4,
};

use glm::{vec2, TVec2};

use crate::{
    draw::Draw,
    shader_program::ShaderProgram,
    vertex_objects::{BufferType, VAO, VBO},
};

#[repr(C)]
struct Vertex2D {
    position: TVec2<f32>,
}

impl Default for Vertex2D {
    fn default() -> Self {
        Self {
            position: vec2(0.0, 0.0),
        }
    }
}

pub struct Quad {
    position: TVec2<f32>,
    width: f32,
    height: f32,
    color: TVec4<f32>,
    vao: VAO,
    vbo: VBO,
    ebo: VBO,
}

impl Quad {
    pub fn new(position: TVec2<f32>, width: f32, height: f32, color: TVec4<f32>) -> Self {
        let width = width * 2.0;
        let height = height * 2.0;
        let mut quad = Self {
            position,
            width,
            height,
            color,
            vao: VAO::new(),
            vbo: VBO::new(BufferType::Array),
            ebo: VBO::new(BufferType::ElementArray),
        };

        quad.setup_quad();
        quad
    }

    fn setup_quad(&mut self) {
        self.vao.bind();

        let vertices = [
            Vertex2D {
                position: vec2(self.position.x, self.position.y),
            },
            Vertex2D {
                position: vec2(self.position.x + self.width, self.position.y),
            },
            Vertex2D {
                position: vec2(self.position.x, self.position.y - self.height),
            },
            Vertex2D {
                position: vec2(self.position.x + self.width, self.position.y - self.height),
            },
        ];

        let indices: [u32; 6] = [0, 2, 1, 2, 3, 1];

        self.vbo.bind();
        unsafe {
            glBufferData(
                GL_ARRAY_BUFFER,
                (vertices.len() * size_of::<Vertex2D>()).try_into().unwrap(),
                vertices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }

        self.ebo.bind();
        unsafe {
            glBufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<Vertex2D>()).try_into().unwrap(),
                indices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }

        unsafe {
            let stride: i32 = size_of::<Vertex2D>().try_into().unwrap();

            glEnableVertexAttribArray(0);
            glVertexAttribPointer(0, 2, GL_FLOAT, 0, stride, 0 as *const _);
        }
    }
}

impl Draw for Quad {
    fn draw(&self, shader: &ShaderProgram) {
        self.vao.bind();

        shader.set_vec4("color", self.color);

        unsafe {
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
        }

        VAO::clear_binding()
    }
}
