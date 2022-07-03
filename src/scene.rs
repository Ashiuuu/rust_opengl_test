use std::collections::HashMap;

use crate::{scene_object::SceneObject, shader_program::ShaderProgram};

pub struct Scene<'a> {
    objects: HashMap<&'static str, SceneObject<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    //pub fn insert(&mut self, name: &str, obj: SceneObject<'a>) -> Option<SceneObject<'a>> {
    //match self.objects.get(name) {
    //None => self.objects.insert(name, obj),
    //Some(_) => Some(obj),
    //}
    //}

    pub fn draw(&self, shader: &ShaderProgram) {
        for (name, o) in self.objects.iter() {
            o.draw(shader);
        }
    }
}
