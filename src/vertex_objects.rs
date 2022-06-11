use gl33::global_loader::*;
use gl33::*;

pub struct VAO {
    vao: u32,
}

impl VAO {
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            glGenVertexArrays(1, &mut vao);
            if vao != 0 {
                Some(Self { vao })
            } else {
                None
            }
        }
    }

    pub fn bind(&self) {
        glBindVertexArray(self.vao);
    }

    pub fn clear_binding() {
        glBindVertexArray(0);
    }
}

pub enum BufferType {
    Array,
    ElementArray,
}

impl From<&BufferType> for gl33::GLenum {
    fn from(t: &BufferType) -> Self {
        match t {
            BufferType::Array => GL_ARRAY_BUFFER,
            BufferType::ElementArray => GL_ELEMENT_ARRAY_BUFFER,
        }
    }
}

pub struct VBO {
    id: u32,
    buffer_type: BufferType,
}

impl VBO {
    pub fn new(buffer_type: BufferType) -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self {
                id: vbo,
                buffer_type,
            })
        } else {
            None
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindBuffer((&self.buffer_type).into(), self.id);
        }
    }

    pub fn clear_binding(&self) {
        unsafe {
            glBindBuffer((&self.buffer_type).into(), 0);
        }
    }
}
