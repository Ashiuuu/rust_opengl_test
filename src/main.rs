use glutin::{window::Window, ContextWrapper, NotCurrent, PossiblyCurrent};

mod camera;
mod key_state;
mod lights;
mod macros;
mod mesh;
mod model;
mod shader_program;
mod texture;
mod utils;
mod vertex_objects;

extern crate nalgebra_glm as glm;

use {
    camera::Camera,
    gl33::{global_loader::*, *},
    glm::Mat4,
    glutin::{
        dpi::{LogicalPosition, LogicalSize},
        event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        Api, ContextBuilder, GlRequest,
    },
    key_state::MovementState,
    lazy_static::lazy_static,
    model::Model,
    shader_program::ShaderProgram,
    std::time::Instant,
    utils::{clear_color, glenum_to_i32, to_radians},
    vertex_objects::VAO,
};

lazy_static! {
    static ref IDENTITY_MAT4: Mat4 =
        glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,);
}

fn init_window(
    width: i32,
    height: i32,
) -> (EventLoop<()>, ContextWrapper<PossiblyCurrent, Window>) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("OpenGL Test !")
        .with_inner_size(LogicalSize::new(width, height));

    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    (el, context)
}

fn init_opengl(context: &ContextWrapper<PossiblyCurrent, Window>) {
    unsafe {
        load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            context.get_proc_address(r_str) as _
        });

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
}

fn main() {
    // Window and OpenGL initialization
    let (el, context) = init_window(1600, 1080);

    let window_width = context.window().inner_size().width as f32;
    let window_height = context.window().inner_size().height as f32;

    init_opengl(&context);

    // Actual program starts here
    let shader =
        ShaderProgram::from_files("src\\model_loading.vs", "src\\model_loading.fs").unwrap();
    let model = Model::new("backpack\\backpack.obj");

    clear_color(0.25, 0.25, 0.25);

    let mut camera = Camera::new();
    let mut mouse_snapback = true;

    let mut movement_state = MovementState::new();

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

                shader.use_program();

                let projection_matrix =
                    glm::perspective(window_width / window_height, to_radians(90.0), 0.1, 100.0);

                let camera_view = camera.view_matrix();

                let model_matrix = glm::translate(&IDENTITY_MAT4, &glm::vec3(0.0, -1.75, 0.0));

                shader.set_mat4("projection", &projection_matrix);
                shader.set_mat4("view", &camera_view);
                shader.set_mat4("model", &model_matrix);
                model.draw(&shader);

                VAO::clear_binding();

                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    })
}
