use draw::Draw;
use texture::Texture2D;

use crate::{transform::AsVec3Mut, vec3_sliders::Vec3Sliders};

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
mod quad;
mod scene_object;
mod shader_program;
mod slider;
mod static_camera;
mod texture;
mod transform;
mod utils;
mod vec3_sliders;
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

    println!("Window created: ({}, {})", window_width, window_height);

    init_opengl(&context);

    // Actual program starts here
    let shader = ShaderProgram::from_files("model_loading.vs", "model_loading.fs");
    let gui_shader = ShaderProgram::from_files("gui.vs", "gui.fs");

    let mut model = SceneObject::model("backpack.obj");
    let mut normal_plane = SceneObject::plane(Some(Texture2D::from_texture(
        "container.jpg",
        texture::TextureType::Diffuse,
    )));
    let mut back_plane = SceneObject::plane(Some(Texture2D::from_image(
        "ao.jpg",
        "ressources/models/backpack",
        texture::TextureType::Diffuse,
    )));

    let mut portal1 = Portal::new(
        window_width.try_into().unwrap(),
        window_height.try_into().unwrap(),
    );
    let mut portal2 = Portal::new(
        window_width.try_into().unwrap(),
        window_height.try_into().unwrap(),
    );

    model.set_position(glm::vec3(0.0, -1.75, 0.0));
    model.set_roll(45.0);

    normal_plane.set_position(glm::vec3(0.0, 0.0, 5.0));
    normal_plane.set_scale(glm::vec3(10.0, 10.0, 1.0));
    //normal_plane.set_yaw(90.0);

    back_plane.set_position(glm::vec3(0.0, 0.0, 4.0));
    back_plane.set_scale(glm::vec3(5.0, 5.0, 1.0));

    portal1.surface.set_position(glm::vec3(1.0, 1.0, -8.0));
    portal1.surface.set_scale(glm::vec3(5.0, 5.0, 1.0));

    portal2.surface.set_position(glm::vec3(0.0, 0.0, -8.0));
    portal2.surface.set_pitch(90.0);
    portal2.surface.set_scale(glm::vec3(5.0, 5.0, 1.0));

    // GUI setup
    let sliders = Vec3Sliders::new(
        //&mut normal_plane.transform.angles,
        normal_plane.transform.mut_angles(),
        (50.0, 50.0),
        200.0,
        200.0,
        glm::vec4(0.5, 0.5, 0.5, 0.5),
        glm::vec4(0.5, 0.0, 0.5, 1.0),
        window_width as f32,
        window_height as f32,
        50.0,
        20.0,
    );

    clear_color(0.25, 0.25, 0.25);

    let mut camera = Camera::new();
    let mut mouse_snapback = true;

    let mut movement_state = MovementState::new();

    let mut last_frame = Instant::now();
    let start = Instant::now();

    unsafe {
        glEnable(GL_DEPTH_TEST);
        //glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
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
                            //VirtualKeyCode::Right => slider.step_value(1.0),
                            //VirtualKeyCode::Right => {
                            //normal_plane.set_roll(normal_plane.roll() + 10.0)
                            //}
                            //VirtualKeyCode::Left => {
                            //normal_plane.set_roll(normal_plane.roll() - 10.0)
                            //}
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

                // Render GUI on top of everything
                unsafe {
                    glEnable(GL_BLEND);
                    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
                }
                gui_shader.use_program();
                sliders.draw(&gui_shader);

                VAO::clear_binding();

                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    })
}
