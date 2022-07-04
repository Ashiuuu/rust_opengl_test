use texture::Texture2D;

mod camera;
mod draw;
mod framebuffer;
mod key_state;
mod lights;
mod macros;
mod mesh;
mod model;
mod plane;
mod portal;
mod scene_object;
mod shader_program;
mod static_camera;
mod texture;
mod transform;
mod utils;
mod vertex_objects;

extern crate nalgebra_glm as glm;

use {
    camera::Camera,
    framebuffer::Framebuffer,
    gl33::{global_loader::*, *},
    glutin::{
        dpi::{LogicalPosition, LogicalSize},
        event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::Window,
        window::WindowBuilder,
        Api, ContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent,
    },
    key_state::MovementState,
    model::Model,
    plane::Plane,
    portal::Portal,
    scene_object::SceneObject,
    shader_program::ShaderProgram,
    std::time::Instant,
    utils::*,
    vertex_objects::VAO,
};

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

    let window_width = context.window().inner_size().width;
    let window_height = context.window().inner_size().height;

    init_opengl(&context);

    // Actual program starts here
    //let shader = ShaderProgram::from_files("src\\model_loading.vs", "src\\model_loading.fs");
    let shader = ShaderProgram::from_files("src/model_loading.vs", "src/model_loading.fs");

    //let mut model = SceneObject::new(Model::new("backpack\\backpack.obj"));
    let mut model = SceneObject::new(Model::new("backpack/backpack.obj"));
    let mut normal_plane = SceneObject::new(Plane::new(vec![Texture2D::from_image(
        "container.jpg",
        ".",
        texture::TextureType::Diffuse,
    )]));
    let mut back_plane = SceneObject::new(Plane::new(vec![Texture2D::from_image(
        "ao.jpg",
        "backpack",
        texture::TextureType::Diffuse,
    )]));

    let mut portal1 = Portal::new(
        window_width.try_into().unwrap(),
        window_height.try_into().unwrap(),
    );
    let mut portal2 = Portal::new(
        window_width.try_into().unwrap(),
        window_height.try_into().unwrap(),
    );

    model.set_position(glm::vec3(0.0, -1.75, 0.0));
    model.set_angle(45.0);

    normal_plane.set_position(glm::vec3(0.0, 0.0, 5.0));
    normal_plane.set_rotation_axis(glm::vec3(1.0, 0.0, 0.0));
    normal_plane.set_angle(90.0);
    normal_plane.set_scale(glm::vec3(10.0, 10.0, 1.0));

    back_plane.set_position(glm::vec3(0.0, 0.0, 4.0));
    back_plane.set_scale(glm::vec3(5.0, 5.0, 1.0));

    portal1.surface.set_position(glm::vec3(1.0, 1.0, -8.0));
    portal1.surface.set_scale(glm::vec3(5.0, 5.0, 1.0));

    portal2.surface.set_position(glm::vec3(0.0, 0.0, -8.0));
    portal2.surface.set_rotation_axis(glm::vec3(0.0, 1.0, 0.0));
    portal2.surface.set_angle(90.0);
    portal2.surface.set_scale(glm::vec3(5.0, 5.0, 1.0));

    clear_color(0.25, 0.25, 0.25);

    let mut camera = Camera::new();
    let mut mouse_snapback = true;

    let mut movement_state = MovementState::new();

    let mut last_frame = Instant::now();
    let start = Instant::now();

    unsafe {
        glEnable(GL_DEPTH_TEST);
    }

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
                                (window_width as f32) / 2.0,
                                (window_height as f32) / 2.0,
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
            Event::MainEventsCleared => {
                gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

                let dt = last_frame.elapsed().as_secs_f32();
                last_frame = Instant::now();
                let elapsed = start.elapsed().as_secs_f32();

                // camera handling
                camera.update_movement(&movement_state, dt);

                portal1.camera.pos = portal2.surface.position() - camera.position;
                portal2.camera.pos = portal1.surface.position() - camera.position;

                shader.use_program();

                let projection_matrix = glm::perspective(
                    (window_width as f32) / (window_height as f32),
                    to_radians(90.0),
                    0.1,
                    100.0,
                );
                shader.set_mat4("projection", &projection_matrix);

                // first render for framebuffer 1
                portal1.bind_framebuffer();
                gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                shader.set_mat4("view", &portal1.camera.view_matrix());
                model.draw(&shader);
                normal_plane.draw(&shader);
                back_plane.draw(&shader);

                // then render for framebuffer 2
                portal2.bind_framebuffer();
                gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
                shader.set_mat4("view", &portal2.camera.view_matrix());
                model.draw(&shader);
                normal_plane.draw(&shader);
                back_plane.draw(&shader);

                // then render normal scene
                Framebuffer::clear_binding();
                gl_clear(GL_COLOR_BUFFER_BIT);
                shader.set_mat4("view", &camera.view_matrix());
                model.draw(&shader);
                normal_plane.draw(&shader);
                back_plane.draw(&shader);

                portal1.render(&shader);
                portal2.render(&shader);

                VAO::clear_binding();

                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    })
}
