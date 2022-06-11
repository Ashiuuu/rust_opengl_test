#[allow(dead_code)]
mod shader_program;
mod vertex_objects;

use gl33::global_loader::*;
use gl33::*;

use glutin::{
    event::{Event, KeyboardInput, ScanCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};

use core::cmp::{max, min};
use core::mem::{size_of, size_of_val};
use image::{io::Reader as ImageReader, ColorType};
use std::time::Instant;

use shader_program::ShaderProgram;
use vertex_objects::{BufferType, VAO, VBO};

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

struct Texture2D {
    id: u32,
}

impl Texture2D {
    pub fn from_image(filename: &str) -> Self {
        let image = ImageReader::open(filename)
            .unwrap()
            .decode()
            .unwrap()
            .flipv();

        let mut id = 0;
        unsafe {
            glGenTextures(1, &mut id);
            assert_ne!(id, 0);

            glBindTexture(GL_TEXTURE_2D, id);
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                glenum_to_i32(GL_RGB),
                image.width().try_into().unwrap(),
                image.height().try_into().unwrap(),
                0,
                match image.color() {
                    ColorType::Rgb8 => GL_RGB,
                    ColorType::Rgba8 => GL_RGBA,
                    _ => panic!("Can't convert color type {:?}", image.color()),
                },
                GL_UNSIGNED_BYTE,
                image.as_bytes().as_ptr().cast(),
            );
            glGenerateMipmap(GL_TEXTURE_2D);
        }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.id);
        }
    }

    pub fn clear_binding() {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, 0);
        }
    }
}

type Vertex = [f32; 3];
// Pos[f32;3] Color[f32;3] TexturePos[f32;2]
const RECTANGLE_VERTICES: [f32; 32] = [
    0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -0.5, -0.5,
    0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
];
const RECTANGLE_INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

fn clear_color(color: &Color) {
    unsafe {
        glClearColor(color.r, color.g, color.b, 1.0);
    }
}

fn glenum_to_i32(e: GLenum) -> i32 {
    match e {
        GL_NEAREST => 0x2600,
        GL_LINEAR => 0x2601,
        GL_LINEAR_MIPMAP_LINEAR => 0x2703,
        GL_RGB => 0x1907,
        GL_CLAMP_TO_EDGE => 0x812F,
        _ => panic!("Don't call into for GLenum variant {:?}", e),
    }
}

fn main() {
    // Window and OpenGL initialization
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("OpenGL Test !");

    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    // Load OpenGL symbols
    unsafe {
        load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            context.get_proc_address(r_str) as _
        })
    };

    unsafe {
        glTexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MIN_FILTER,
            glenum_to_i32(GL_LINEAR_MIPMAP_LINEAR),
        );
        glTexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MAG_FILTER,
            glenum_to_i32(GL_LINEAR),
        );
    }

    let mut transparancy = 0.2;
    let step = 0.01;

    clear_color(&Color::from(0.2, 0.3, 0.3));

    let vao = VAO::new().unwrap();
    vao.bind();

    let vbo = VBO::new(BufferType::Array).unwrap();
    let ebo = VBO::new(BufferType::ElementArray).unwrap();

    let texture_1 = Texture2D::from_image("container.jpg");
    let texture_2 = Texture2D::from_image("awesomeface.png");

    unsafe {
        vbo.bind();
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&RECTANGLE_VERTICES) as isize,
            RECTANGLE_VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        ebo.bind();
        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            size_of_val(&RECTANGLE_INDICES) as isize,
            RECTANGLE_INDICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        let float_size: i32 = size_of::<f32>().try_into().unwrap();
        let stride = 8 * float_size;

        // position attribute
        glVertexAttribPointer(0, 3, GL_FLOAT, 0, stride, 0 as *const _);
        glVertexAttribPointer(1, 3, GL_FLOAT, 0, stride, size_of::<Vertex>() as *const _);
        glVertexAttribPointer(
            2,
            2,
            GL_FLOAT,
            0,
            stride,
            (2_i32 * (size_of::<Vertex>() as i32)) as *const _,
        );

        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);
        glEnableVertexAttribArray(2);

        texture_1.bind();
        glTexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_WRAP_S,
            glenum_to_i32(GL_CLAMP_TO_EDGE),
        );
        glTexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_WRAP_T,
            glenum_to_i32(GL_CLAMP_TO_EDGE),
        );

        Texture2D::clear_binding();
    }

    let shader_program =
        ShaderProgram::from_files("vertex_shader.vs", "fragment_shader.fs").unwrap();
    shader_program.link();

    let start_time = Instant::now();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input: kb_input,
                    is_synthetic: false,
                } => match kb_input.scancode {
                    57416 => {
                        transparancy = if transparancy - step < 0.0 {
                            0.0
                        } else {
                            transparancy - step
                        }
                    }
                    57424 => {
                        transparancy = if transparancy + step > 1.0 {
                            1.0
                        } else {
                            transparancy + step
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => context.window().request_redraw(),
            Event::RedrawRequested(_) => unsafe {
                glClear(GL_COLOR_BUFFER_BIT);

                vao.bind();
                shader_program.use_program();
                glActiveTexture(GL_TEXTURE0);
                texture_1.bind();
                glActiveTexture(GL_TEXTURE1);
                texture_2.bind();

                shader_program.set_int("texture1", 0);
                shader_program.set_int("texture2", 1);
                shader_program.set_float("transparancy", transparancy);
                glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);

                VAO::clear_binding();

                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    })
}
