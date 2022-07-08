use glm::TVec4;

use crate::{draw::Draw, quad::Quad, utils::*};

pub struct Slider<'a> {
    target: &'a mut f32,
    position: (f32, f32),
    inner_quad: Quad,
    outer_quad: Quad,
    margin: f32,
    width: f32,
    height: f32,
    window_width: f32,
    window_height: f32,
    inner_color: TVec4<f32>,
    min: f32,
    max: f32,
}

impl<'a> Slider<'a> {
    pub fn new(
        target: &'a mut f32,
        position: (f32, f32),
        width: f32,
        height: f32,
        outer_color: TVec4<f32>,
        inner_color: TVec4<f32>,
        min: f32,
        max: f32,
        window_width: f32,
        window_height: f32,
        margin: f32,
    ) -> Self {
        let value = *target;
        Self {
            target,
            position,
            inner_quad: Slider::generate_inner_quad(
                position,
                margin,
                value,
                max,
                min,
                width,
                height,
                window_width,
                window_height,
                inner_color,
            ),
            outer_quad: Quad::new(
                vec2_from_tuple(pixels_to_coords(
                    position.0,
                    position.1,
                    window_width as f32,
                    window_height as f32,
                )),
                width / window_width,
                height / window_height,
                outer_color,
            ),
            margin,
            width,
            height,
            window_width,
            window_height,
            inner_color,
            min,
            max,
        }
    }

    fn generate_inner_quad(
        position: (f32, f32),
        margin: f32,
        value: f32,
        max: f32,
        min: f32,
        width: f32,
        height: f32,
        window_width: f32,
        window_height: f32,
        color: TVec4<f32>,
    ) -> Quad {
        let actual_width = width * (value / max);

        Quad::new(
            vec2_from_tuple(pixels_to_coords(
                position.0 + margin,
                position.1 + margin,
                window_width,
                window_height,
            )),
            (actual_width - (2.0 * margin)) / window_width,
            (height - (2.0 * margin)) / window_height,
            color,
        )
    }

    pub fn set_value(&mut self, value: f32) {
        if value < self.min {
            *self.target = self.min;
        } else if value > self.max {
            *self.target = self.max;
        } else {
            *self.target = value;
        }

        self.inner_quad = Slider::generate_inner_quad(
            self.position,
            self.margin,
            *self.target,
            self.max,
            self.min,
            self.width,
            self.height,
            self.window_width,
            self.window_height,
            self.inner_color,
        )
    }

    pub fn step_value(&mut self, increment: f32) {
        self.set_value(*self.target + increment)
    }
}

impl<'a> Draw for Slider<'a> {
    fn draw(&self, shader: &crate::shader_program::ShaderProgram) {
        self.inner_quad.draw(shader);
        self.outer_quad.draw(shader);
    }
}
