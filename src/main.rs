mod camera;
mod lights;
mod macros;
mod mesh;
mod model;
mod shader_program;
mod texture;
mod vertex_objects;

extern crate nalgebra_glm as glm;

use gl33::global_loader::*;
use gl33::*;

use lazy_static::lazy_static;

use glm::Mat4;
use glutin::{
    dpi::{LogicalPosition, LogicalSize},
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};
use lights::{DirectionalLight, PointLight};

use core::mem::{size_of, size_of_val};
use std::{f32::consts::PI, time::Instant};

use camera::Camera;
use model::Model;
use shader_program::ShaderProgram;
use texture::{Texture2D, TextureType};
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

pub enum KeyState {
    Pressed,
    Released,
}

impl KeyState {
    fn is_pressed(&self) -> bool {
        match self {
            Self::Pressed => true,
            _ => false,
        }
    }

    fn is_released(&self) -> bool {
        match self {
            Self::Released => true,
            _ => false,
        }
    }
}

impl From<ElementState> for KeyState {
    fn from(e: ElementState) -> Self {
        match e {
            ElementState::Pressed => Self::Pressed,
            ElementState::Released => Self::Released,
        }
    }
}

pub struct MovementState {
    pub forward: KeyState,
    pub backward: KeyState,
    pub right: KeyState,
    pub left: KeyState,
}

impl MovementState {
    fn new() -> Self {
        Self {
            forward: KeyState::Released,
            backward: KeyState::Released,
            right: KeyState::Released,
            left: KeyState::Released,
        }
    }
}

lazy_static! {
    static ref IDENTITY_MAT4: Mat4 =
        glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,);
}

fn to_radians(e: f32) -> f32 {
    e * PI / 180.0
}

type Vertex = [f32; 8];

const CUBE: [f32; 288] = [
    -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0, 0.5,
    0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, -0.5, 0.5, -0.5,
    0.0, 0.0, -1.0, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0,
    0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0,
    1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, -0.5,
    -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, -0.5, 0.5, -0.5,
    -1.0, 0.0, 0.0, 1.0, 1.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, -0.5, -0.5, -0.5, -1.0,
    0.0, 0.0, 0.0, 1.0, -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0,
    1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5,
    -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5,
    1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0,
    0.0, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0,
    0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0, -0.5,
    -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5, 0.5, -0.5,
    0.0, 1.0, 0.0, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0,
    1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
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

fn usize_to_glenum(e: usize) -> GLenum {
    match e {
        0x84c0 => GL_TEXTURE0,
        0x84c1 => GL_TEXTURE1,
        0x84c2 => GL_TEXTURE2,
        0x84c3 => GL_TEXTURE3,
        0x84c4 => GL_TEXTURE4,
        0x84c5 => GL_TEXTURE5,
        0x84c6 => GL_TEXTURE6,
        0x84c7 => GL_TEXTURE7,
        0x84c8 => GL_TEXTURE8,
        0x84c9 => GL_TEXTURE0,
        _ => unimplemented!(),
    }
}

fn main() {
    // Window and OpenGL initialization
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("OpenGL Test !")
        .with_inner_size(LogicalSize::new(1600, 1080));

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

    let model = Model::new("backpack".to_string());

    clear_color(&Color::from(0.25, 0.25, 0.25));

    let mut camera = Camera::new();
    let mut mouse_snapback = true;

    let mut movement_state = MovementState::new();

    let cube_positions = [
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

    let light_vao = VAO::new().unwrap();
    let container_vao = VAO::new().unwrap();

    let vbo = VBO::new(BufferType::Array).unwrap();
    //let ebo = VBO::new(BufferType::ElementArray).unwrap();

    unsafe {
        vbo.bind();
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&CUBE) as isize,
            CUBE.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        let stride = size_of::<Vertex>() as i32;
        let float_size: i32 = size_of::<f32>().try_into().unwrap();

        container_vao.bind();

        // position attribute
        glVertexAttribPointer(0, 3, GL_FLOAT, 0, stride, 0 as *const _);
        glVertexAttribPointer(1, 3, GL_FLOAT, 0, stride, (float_size * 3) as *const _);
        glVertexAttribPointer(2, 2, GL_FLOAT, 0, stride, (float_size * 6) as *const _);

        glEnableVertexAttribArray(0);
        glEnableVertexAttribArray(1);
        glEnableVertexAttribArray(2);

        light_vao.bind();
        vbo.bind();
        glVertexAttribPointer(0, 3, GL_FLOAT, 0, stride, 0 as *const _);
        glEnableVertexAttribArray(0);
    }

    let directional_light = DirectionalLight::new(
        glm::vec3(-0.2, -1.0, -0.3),
        glm::vec3(0.2, 0.2, 0.2),
        glm::vec3(0.5, 0.5, 0.5),
        glm::vec3(1.0, 1.0, 1.0),
    );

    let point_light_positions = vec![
        glm::vec3(0.7, 0.2, 2.0),
        glm::vec3(2.3, -3.3, -4.0),
        glm::vec3(-4.0, 2.0, -12.0),
        glm::vec3(0.0, 0.0, -3.0),
    ];

    let point_light_colors = vec![
        glm::vec3(1.0, 0.6, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(1.0, 1.0, 0.0),
        glm::vec3(0.2, 0.2, 1.0),
    ];

    let point_lights = vec![
        PointLight::new(
            point_light_positions[0],
            point_light_colors[0] * 0.1,
            point_light_colors[0],
            point_light_colors[0],
            1.0,
            0.09,
            0.032,
        ),
        PointLight::new(
            point_light_positions[1],
            point_light_colors[1] * 0.1,
            point_light_colors[1],
            point_light_colors[1],
            1.0,
            0.09,
            0.032,
        ),
        PointLight::new(
            point_light_positions[2],
            point_light_colors[2] * 0.1,
            point_light_colors[2],
            point_light_colors[2],
            1.0,
            0.09,
            0.032,
        ),
        PointLight::new(
            point_light_positions[3],
            point_light_colors[3] * 0.1,
            point_light_colors[3],
            point_light_colors[3],
            1.0,
            0.09,
            0.032,
        ),
    ];

    let diffuse_map = Texture2D::from_image("container2.png", TextureType::Diffuse);
    let specular_map = Texture2D::from_image("container2_specular.png", TextureType::Specular);
    let emission_map = Texture2D::from_image("matrix.jpg", TextureType::Emission);

    let shader_program =
        ShaderProgram::from_files("vertex_shader.vs", "fragment_shader.fs").unwrap();
    let light_shader =
        ShaderProgram::from_files("vertex_shader.vs", "fragment_shader_2.fs").unwrap();
    shader_program.link();
    light_shader.link();

    shader_program.use_program();
    shader_program.set_int("material.diffuse", 0);
    shader_program.set_int("material.specular", 1);
    shader_program.set_int("material.emission", 2);

    let mut last_frame = Instant::now();
    let start = Instant::now();

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
                    camera.update_orientation(dx as f32, dy as f32);
                    if mouse_snapback {
                        context
                            .window()
                            .set_cursor_position(LogicalPosition::new(
                                window_width / 2.0,
                                window_height / 2.0,
                            ))
                            .unwrap();
                    }
                }
                DeviceEvent::Key(key) => {
                    if let Some(key_code) = key.virtual_keycode {
                        match key_code {
                            VirtualKeyCode::W => movement_state.forward = key.state.into(),
                            VirtualKeyCode::S => movement_state.backward = key.state.into(),
                            VirtualKeyCode::D => movement_state.right = key.state.into(),
                            VirtualKeyCode::A => movement_state.left = key.state.into(),
                            VirtualKeyCode::C => mouse_snapback = true,
                            VirtualKeyCode::V => mouse_snapback = false,
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
                let elapsed = start.elapsed().as_secs_f32();

                // camera handling
                camera.update_movement(&movement_state, dt);

                container_vao.bind();
                shader_program.use_program();

                glActiveTexture(GL_TEXTURE0);
                diffuse_map.bind();
                glActiveTexture(GL_TEXTURE1);
                specular_map.bind();
                glActiveTexture(GL_TEXTURE2);
                emission_map.bind();

                let projection_matrix =
                    glm::perspective(window_width / window_height, to_radians(90.0), 0.1, 100.0);

                let camera_view = camera.view_matrix();

                shader_program.set_mat4("projection", &projection_matrix);
                shader_program.set_mat4("view", &camera_view);

                shader_program.set_vec3("viewPos", camera.position);

                shader_program.set_vec3("material.specular", glm::vec3(0.5, 0.5, 0.5));
                shader_program.set_float("material.shininess", 32.0);

                shader_program.set_float("time", elapsed);

                directional_light.set_into_shader(&shader_program, "dirLight");
                for (i, light) in point_lights.iter().enumerate() {
                    light.set_into_shader(&shader_program, format!("pointLights[{}]", i).as_str());
                }

                let mut angle = 0.0;
                for (i, pos) in cube_positions.into_iter().enumerate() {
                    let model = glm::translate(&IDENTITY_MAT4, &pos);

                    angle += 10.0;
                    let model = if i % 3 == 0 {
                        glm::rotate(
                            &model,
                            to_radians(angle) + elapsed,
                            &glm::vec3(i as f32, 0.3, 0.5),
                        )
                    } else {
                        glm::rotate(&model, to_radians(angle), &glm::vec3(1.0, 0.3, 0.5))
                    };
                    shader_program.set_mat4("model", &model);

                    glDrawArrays(GL_TRIANGLES, 0, 180);
                }

                light_shader.use_program();

                light_shader.set_mat4("projection", &projection_matrix);
                light_shader.set_mat4("view", &camera_view);

                light_vao.bind();
                for light in &point_lights {
                    let model = glm::translate(&IDENTITY_MAT4, &light.position);
                    let model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
                    light_shader.set_mat4("model", &model);
                    light_shader.set_vec3("lightColor", light.ambient);
                    glDrawArrays(GL_TRIANGLES, 0, 180);
                }

                VAO::clear_binding();

                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    })
}
