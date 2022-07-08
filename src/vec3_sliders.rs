use glm::TVec4;

use crate::{draw::Draw, shader_program::ShaderProgram, slider::Slider, transform::Vec3Mut};

pub struct Vec3Sliders<'a> {
    x_slider: Slider<'a>,
    y_slider: Slider<'a>,
    z_slider: Slider<'a>,
}

impl<'a> Vec3Sliders<'a> {
    pub fn new(
        //target: &'a mut TVec3<f32>,
        target: Vec3Mut<'a>,
        position: (f32, f32),
        width: f32,
        total_height: f32,
        outer_color: TVec4<f32>,
        inner_color: TVec4<f32>,
        window_width: f32,
        window_height: f32,
        margin: f32,
        spacing: f32,
    ) -> Self {
        let height = (total_height - 2.0 * spacing) / 3.0;
        Self {
            x_slider: Slider::new(
                //target.data.0.position,
                target.x,
                position,
                width,
                height,
                outer_color,
                inner_color,
                -90.0,
                90.0,
                window_width,
                window_height,
                margin,
            ),
            y_slider: Slider::new(
                //&mut target.y,
                target.y,
                (position.0, position.1 + height + spacing),
                width,
                height,
                outer_color,
                inner_color,
                -90.0,
                90.0,
                window_width,
                window_height,
                margin,
            ),
            z_slider: Slider::new(
                //&mut target.z,
                target.z,
                (position.0, position.1 + 2.0 * (height + spacing)),
                width,
                height,
                outer_color,
                inner_color,
                -90.0,
                90.0,
                window_width,
                window_height,
                margin,
            ),
        }
    }
}

impl<'a> Draw for Vec3Sliders<'a> {
    fn draw(&self, shader: &ShaderProgram) {
        self.x_slider.draw(shader);
        self.y_slider.draw(shader);
        self.z_slider.draw(shader);
    }
}
