use gl33::global_loader::*;
use gl33::*;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};

use core::mem::{size_of, size_of_val};

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERTEX_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 aPos;
    void main()
    {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
    "#;

const FRAGMENT_SHADER: &str = r#"#version 330 core
    out vec4 final_color;
    
    void main()
    {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);
    }
    "#;

fn shader_error_log(shader: u32) {
    unsafe {
        let mut success = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }
}

fn program_link_error_log(program: u32) {
    unsafe {
        let mut success = 0;
        glGetProgramiv(program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetProgramInfoLog(program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
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

    unsafe {
        glClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        glBindVertexArray(vao);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
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

        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        glShaderSource(
            vertex_shader,
            1,
            &(VERTEX_SHADER.as_bytes().as_ptr().cast()),
            &(VERTEX_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(vertex_shader);
        shader_error_log(vertex_shader);

        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        glShaderSource(
            fragment_shader,
            1,
            &(FRAGMENT_SHADER.as_bytes().as_ptr().cast()),
            &(FRAGMENT_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(fragment_shader);
        shader_error_log(fragment_shader);

        let shader_program = glCreateProgram();
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);
        program_link_error_log(shader_program);
        glUseProgram(shader_program);

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);
    }

    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
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
                glDrawArrays(GL_TRIANGLES, 0, 3);
                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    })
}
