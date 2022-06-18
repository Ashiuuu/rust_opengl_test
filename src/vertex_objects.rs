use gl33::global_loader::*;
use gl33::*;

pub struct VAO {
    vao: u32,
}

#[derive(Debug)]
pub struct VAOCreationFail {}

impl VAO {
    pub fn new() -> Result<Self, VAOCreationFail> {
        let mut vao = 0;
        unsafe {
            glGenVertexArrays(1, &mut vao);
            if vao != 0 {
                Ok(Self { vao })
            } else {
                Err(VAOCreationFail {})
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct VBOCreationFail(pub BufferType);

pub struct VBO {
    id: u32,
    buffer_type: BufferType,
}

impl VBO {
    pub fn new(buffer_type: BufferType) -> Result<Self, VBOCreationFail> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Ok(Self {
                id: vbo,
                buffer_type,
            })
        } else {
            Err(VBOCreationFail(buffer_type))
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindBuffer((&self.buffer_type).into(), self.id);
        }
    }

    pub fn clear_binding() {
        unsafe {
            glBindBuffer((&BufferType::Array).into(), 0);
        }
    }
}
