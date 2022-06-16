#[allow(dead_code)]
mod shader_program;
mod vertex_objects;

extern crate nalgebra_glm as glm;

use gl33::global_loader::*;
use gl33::*;

use lazy_static::lazy_static;

use glm::Mat4;
use glutin::{
    dpi::LogicalPosition,
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};

use core::mem::{size_of, size_of_val};
use image::{io::Reader as ImageReader, ColorType};
use std::{f32::consts::PI, time::Instant};

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

//fn identity_mat4() -> Mat4 {
//glm::mat4(
//1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
//)
//}

lazy_static! {
    static ref IDENTITY_MAT4: Mat4 =
        glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,);
}

fn to_radians(e: f32) -> f32 {
    e * PI / 180.0
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

const CUBE: [f32; 180] = [
    -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
    -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0,
    0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5,
    0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5, 1.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5,
    0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0,
    0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5,
    0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5,
    -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5,
    1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 0.0, -0.5,
    0.5, -0.5, 0.0, 1.0,
];

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

    let window_width = context.window().inner_size().width as f32;
    let window_height = context.window().inner_size().height as f32;

    // Load OpenGL symbols
    unsafe {
        load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            context.get_proc_address(r_str) as _
        })
    };

    unsafe {
        glEnable(GL_DEPTH_TEST);
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

    clear_color(&Color::from(0.2, 0.3, 0.3));

    let mut camera_pos = glm::vec3(0.0, 0.0, 3.0);
    let mut camera_front = glm::vec3(0.0, 0.0, -1.0);
    let mut camera_up = glm::vec3(0.0, 1.0, 0.0);

    let mut pitch = 0.0;
    let mut yaw = -90.0;
    let sensitivity = 0.5;

    let mut forward = false;
    let mut backward = false;
    let mut right = false;
    let mut left = false;

    let cube_positions = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let vao = VAO::new().unwrap();
    vao.bind();

    let vbo = VBO::new(BufferType::Array).unwrap();
    //let ebo = VBO::new(BufferType::ElementArray).unwrap();

    let texture_1 = Texture2D::from_image("container.jpg");
    let texture_2 = Texture2D::from_image("awesomeface.png");

    unsafe {
        vbo.bind();
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&CUBE) as isize,
            CUBE.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        //ebo.bind();
        //glBufferData(
        //GL_ELEMENT_ARRAY_BUFFER,
        //size_of_val(&CUBE_INDICES) as isize,
        //CUBE_INDICES.as_ptr().cast(),
        //GL_STATIC_DRAW,
        //);

        let float_size: i32 = size_of::<f32>().try_into().unwrap();
        let stride = 5 * float_size;

        // position attribute
        glVertexAttribPointer(0, 3, GL_FLOAT, 0, stride, 0 as *const _);
        glVertexAttribPointer(
            1,
            2,
            GL_FLOAT,
            0,
            stride,
            (size_of::<Vertex>() as i32) as *const _,
        );

        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);

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

    let mut last_frame = Instant::now();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                    yaw += (dx as f32) * sensitivity;
                    pitch -= (dy as f32) * sensitivity;
                    context
                        .window()
                        .set_cursor_position(LogicalPosition::new(
                            window_width / 2.0,
                            window_height / 2.0,
                        ))
                        .unwrap();
                }
                DeviceEvent::Key(key) => {
                    if let Some(key_code) = key.virtual_keycode {
                        match key_code {
                            VirtualKeyCode::W => match key.state {
                                ElementState::Pressed => forward = true,
                                ElementState::Released => forward = false,
                            },
                            VirtualKeyCode::S => match key.state {
                                ElementState::Pressed => backward = true,
                                ElementState::Released => backward = false,
                            },
                            VirtualKeyCode::D => match key.state {
                                ElementState::Pressed => right = true,
                                ElementState::Released => right = false,
                            },
                            VirtualKeyCode::A => match key.state {
                                ElementState::Pressed => left = true,
                                ElementState::Released => left = false,
                            },
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => unsafe {
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

                let dt = last_frame.elapsed().as_secs_f32();
                last_frame = Instant::now();

                let camera_speed = 2.5 * dt;

                // camera handling
                if forward {
                    camera_pos += camera_speed * camera_front;
                } else if backward {
                    camera_pos -= camera_speed * camera_front;
                }
                if right {
                    camera_pos +=
                        glm::normalize(&glm::cross(&camera_front, &camera_up)) * camera_speed
                } else if left {
                    camera_pos -=
                        glm::normalize(&glm::cross(&camera_front, &camera_up)) * camera_speed
                }

                if pitch > 89.0 {
                    pitch = 89.0;
                } else if pitch < -89.0 {
                    pitch = -89.0;
                }
                let camera_direction = glm::vec3(
                    to_radians(yaw).cos() * to_radians(pitch).cos(),
                    to_radians(pitch).sin(),
                    to_radians(yaw).sin() * to_radians(pitch).cos(),
                );
                camera_front = glm::normalize(&camera_direction);

                vao.bind();
                shader_program.use_program();
                glActiveTexture(GL_TEXTURE0);
                texture_1.bind();
                glActiveTexture(GL_TEXTURE1);
                texture_2.bind();

                shader_program.set_int("texture1", 0);
                shader_program.set_int("texture2", 1);

                let projection_matrix =
                    glm::perspective(to_radians(45.0), window_width / window_height, 0.1, 100.0);

                let camera_view =
                    glm::look_at(&camera_pos, &(camera_pos + camera_front), &camera_up);

                shader_program.set_mat4("projection", &projection_matrix);
                shader_program.set_mat4("view", &camera_view);

                let mut angle = 0.0;
                for (i, pos) in cube_positions.into_iter().enumerate() {
                    let model = glm::translate(&IDENTITY_MAT4, &pos);

                    angle += 10.0;
                    let model = if i % 3 == 0 {
                        glm::rotate(
                            &model,
                            to_radians(angle) + dt,
                            &glm::vec3(i as f32, 0.3, 0.5),
                        )
                    } else {
                        glm::rotate(&model, to_radians(angle), &glm::vec3(1.0, 0.3, 0.5))
                    };
                    shader_program.set_mat4("model", &model);

                    glDrawArrays(GL_TRIANGLES, 0, 180);
                }

                //glDrawElements(GL_TRIANGLES, 36, GL_UNSIGNED_INT, 0 as *const _);

                VAO::clear_binding();

                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    })
}
