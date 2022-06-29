use crate::shader_program::ShaderProgram;

pub trait Draw {
    fn draw(&self, shader: &ShaderProgram);
}
