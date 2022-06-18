use glm::TVec3;

use crate::shader_program::ShaderProgram;

pub struct DirectionalLight {
    pub direction: TVec3<f32>,
    pub ambient: TVec3<f32>,
    pub diffuse: TVec3<f32>,
    pub specular: TVec3<f32>,
}

impl DirectionalLight {
    pub fn new(
        direction: TVec3<f32>,
        ambient: TVec3<f32>,
        diffuse: TVec3<f32>,
        specular: TVec3<f32>,
    ) -> Self {
        Self {
            direction,
            ambient,
            diffuse,
            specular,
        }
    }

    pub fn set_into_shader(&self, shader: &ShaderProgram, name: &str) {
        shader.set_vec3(format!("{}{}", name, ".direction").as_str(), self.direction);
        shader.set_vec3(format!("{}{}", name, ".ambient").as_str(), self.ambient);
        shader.set_vec3(format!("{}{}", name, ".diffuse").as_str(), self.diffuse);
        shader.set_vec3(format!("{}{}", name, ".specular").as_str(), self.specular);
    }
}

pub struct PointLight {
    pub position: TVec3<f32>,
    constant: f32,
    linear: f32,
    quadratic: f32,
    pub ambient: TVec3<f32>,
    diffuse: TVec3<f32>,
    specular: TVec3<f32>,
}

impl PointLight {
    pub fn new(
        position: TVec3<f32>,
        ambient: TVec3<f32>,
        diffuse: TVec3<f32>,
        specular: TVec3<f32>,
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            constant,
            linear,
            quadratic,
        }
    }

    pub fn set_into_shader(&self, shader: &ShaderProgram, name: &str) {
        shader.set_vec3(format!("{}{}", name, ".position").as_str(), self.position);
        shader.set_float(format!("{}{}", name, ".constant").as_str(), self.constant);
        shader.set_float(format!("{}{}", name, ".linear").as_str(), self.linear);
        shader.set_float(format!("{}{}", name, ".quadratic").as_str(), self.quadratic);
        shader.set_vec3(format!("{}{}", name, ".ambient").as_str(), self.ambient);
        shader.set_vec3(format!("{}{}", name, ".diffuse").as_str(), self.diffuse);
        shader.set_vec3(format!("{}{}", name, ".specular").as_str(), self.specular);
    }
}
