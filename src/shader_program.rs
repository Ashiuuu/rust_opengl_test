use gl33::global_loader::*;
use gl33::*;

use glm::{Mat4, Vec3, Vec4};

use std::ffi::CString;
use std::fs;

enum ShaderType {
    VertexShader,
    FragmentShader,
}

impl From<&ShaderType> for gl33::ShaderType {
    fn from(t: &ShaderType) -> Self {
        match t {
            ShaderType::VertexShader => GL_VERTEX_SHADER,
            ShaderType::FragmentShader => GL_FRAGMENT_SHADER,
        }
    }
}

struct Shader {
    shader_type: ShaderType,
    id: u32,
}

impl Shader {
    fn from_source(source: &str, shader_type: ShaderType) -> Option<Self> {
        let id = glCreateShader((&shader_type).into());
        if id == 0 {
            None
        } else {
            unsafe {
                glShaderSource(
                    id,
                    1,
                    &(source.as_bytes().as_ptr().cast()),
                    &(source.len().try_into().unwrap()),
                );
            }
            Some(Self { shader_type, id })
        }
    }

    fn from_file(filename: &str, shader_type: ShaderType) -> Option<Self> {
        let file_content = fs::read_to_string(filename).unwrap();
        Shader::from_source(file_content.as_str(), shader_type)
    }

    fn compile(&self) {
        glCompileShader(self.id);

        if self.check_compilation_status().is_err() {
            panic!("Shader Compilation Error: {}", self.error_log());
        }
    }

    fn delete(&self) {
        glDeleteShader(self.id);
    }

    fn check_compilation_status(&self) -> Result<(), ()> {
        unsafe {
            let mut success = 0;
            glGetShaderiv(self.id, GL_COMPILE_STATUS, &mut success);
            if success == 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    fn error_log(&self) -> String {
        unsafe {
            let mut needed_len = 0;
            glGetShaderiv(self.id, GL_INFO_LOG_LENGTH, &mut needed_len);
            let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
            let mut written_len = 0_i32;
            glGetShaderInfoLog(self.id, needed_len, &mut written_len, v.as_mut_ptr().cast());
            v.set_len(written_len.try_into().unwrap());
            String::from_utf8_lossy(&v).to_string()
        }
    }
}

pub struct ShaderProgram {
    id: u32,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

impl ShaderProgram {
    pub fn from_files(vertex_filename: &str, fragment_filename: &str) -> Option<Self> {
        let id = glCreateProgram();
        let vertex_shader = Shader::from_file(vertex_filename, ShaderType::VertexShader)?;
        glAttachShader(id, vertex_shader.id);
        let fragment_shader = Shader::from_file(fragment_filename, ShaderType::FragmentShader)?;
        glAttachShader(id, fragment_shader.id);

        Some(Self {
            id,
            vertex_shader,
            fragment_shader,
        })
    }

    pub fn link(&self) {
        self.vertex_shader.compile();
        self.fragment_shader.compile();
        glLinkProgram(self.id);

        if self.check_linking_status().is_err() {
            panic!("Shader Program Linking Error: {}", self.error_log());
        }

        self.vertex_shader.delete();
        self.fragment_shader.delete();
    }

    pub fn use_program(&self) {
        glUseProgram(self.id);
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe { glGetUniformLocation(self.id, c_name.as_ptr().cast()) }
    }

    pub fn set_int(&self, name: &str, v0: i32) {
        unsafe {
            glUniform1i(self.get_uniform_location(name), v0);
        }
    }

    pub fn set_float(&self, name: &str, v0: f32) {
        unsafe {
            glUniform1f(self.get_uniform_location(name), v0);
        }
    }

    pub fn set_vec3(&self, name: &str, v0: Vec3) {
        unsafe {
            glUniform3fv(self.get_uniform_location(name), 1, v0.as_ptr().cast());
        }
    }

    pub fn set_vec4(&self, name: &str, v0: Vec4) {
        unsafe {
            glUniform4fv(self.get_uniform_location(name), 1, v0.as_ptr().cast());
        }
    }

    pub fn set_mat4(&self, name: &str, v0: &Mat4) {
        unsafe {
            glUniformMatrix4fv(self.get_uniform_location(name), 1, 0, v0.as_ptr().cast());
        }
    }

    fn check_linking_status(&self) -> Result<(), ()> {
        unsafe {
            let mut success = 0;
            glGetProgramiv(self.id, GL_LINK_STATUS, &mut success);
            if success == 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    fn error_log(&self) -> String {
        unsafe {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetProgramInfoLog(self.id, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            String::from_utf8_lossy(&v).to_string()
        }
    }
}
