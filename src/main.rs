use gl33::global_loader::*;
use gl33::*;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};

use core::mem::{size_of, size_of_val};
use std::fs;

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn from(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

struct VAO {
    vao: u32,
}

impl VAO {
    fn new() -> Option<Self> {
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

    fn bind(&self) {
        glBindVertexArray(self.vao);
    }

    fn clear_binding() {
        glBindVertexArray(0);
    }
}

enum BufferType {
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

struct VBO {
    id: u32,
    buffer_type: BufferType,
}

impl VBO {
    fn new(buffer_type: BufferType) -> Option<Self> {
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

    fn bind(&self) {
        unsafe {
            glBindBuffer((&self.buffer_type).into(), self.id);
        }
    }

    fn clear_binding(&self) {
        unsafe {
            glBindBuffer((&self.buffer_type).into(), 0);
        }
    }
}

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
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(self.id, 1024, &mut log_len, v.as_mut_ptr().cast());
            String::from_utf8_lossy(&v).to_string()
        }
    }
}

struct ShaderProgram {
    id: u32,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

impl ShaderProgram {
    fn from_files(vertex_filename: &str, fragment_filename: &str) -> Option<Self> {
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

    fn link(&self) {
        self.vertex_shader.compile();
        self.fragment_shader.compile();
        glLinkProgram(self.id);

        if self.check_linking_status().is_err() {
            panic!("Shader Program Linking Error: {}", self.error_log());
        }

        self.vertex_shader.delete();
        self.fragment_shader.delete();
    }

    fn use_program(&self) {
        glUseProgram(self.id);
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

type Vertex = [f32; 3];
const TRIANGLE_1: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.25, 0.0]];
const TRIANGLE_2: [Vertex; 9] = [
    // top right triangle
    [0.5, -0.5, 0.0],
    [0.0, 0.25, 0.0],
    [0.5, 0.5, 0.0],
    // top left triangle
    [-0.5, -0.5, 0.0],
    [0.0, 0.25, 0.0],
    [-0.5, 0.5, 0.0],
    // bottom triangle
    [-0.5, -0.5, 0.0],
    [0.5, -0.5, 0.0],
    [0.0, -0.75, 0.0],
];

fn clear_color(color: &Color) {
    unsafe {
        glClearColor(color.r, color.g, color.b, 1.0);
    }
}

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("OpenGL Test !");

    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    unsafe {
        load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            context.get_proc_address(r_str) as _
        })
    };

    clear_color(&Color::from(0.2, 0.3, 0.3));

    let vao_1 = VAO::new().unwrap();
    vao_1.bind();

    let vbo_1 = VBO::new(BufferType::Array).unwrap();
    vbo_1.bind();

    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&TRIANGLE_1) as isize,
            TRIANGLE_1.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    let vao_2 = VAO::new().unwrap();
    vao_2.bind();

    let vbo_2 = VBO::new(BufferType::Array).unwrap();
    vbo_2.bind();

    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&TRIANGLE_2) as isize,
            TRIANGLE_2.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    let shader_program_1 =
        ShaderProgram::from_files("vertex_shader.vs", "fragment_shader.fs").unwrap();
    shader_program_1.link();

    let shader_program_2 =
        ShaderProgram::from_files("vertex_shader.vs", "fragment_shader_2.fs").unwrap();
    shader_program_2.link();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => unsafe {
                glClear(GL_COLOR_BUFFER_BIT);

                shader_program_1.use_program();
                vao_1.bind();
                glDrawArrays(GL_TRIANGLES, 0, 3);

                shader_program_2.use_program();
                vao_2.bind();
                glDrawArrays(GL_TRIANGLES, 0, 9);

                VAO::clear_binding();

                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    })
}
