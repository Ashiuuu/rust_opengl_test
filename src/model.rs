use obj::Obj;

use crate::{mesh::Mesh, shader_program::ShaderProgram};

pub struct Model {
    meshes: Vec<Mesh>,
    directory: String,
}

impl Model {
    pub fn new(directory: String) -> Self {
        let mut ret = Self {
            meshes: vec![],
            directory,
        };

        ret.load();

        ret
    }

    fn load(&mut self) {
        let mut obj = Obj::load(&self.directory).unwrap();
        obj.load_mtls().unwrap();
        let obj_data = obj.data;

        println!(
            "{} | {} | {}",
            obj_data.position.len(),
            obj_data.texture.len(),
            obj_data.normal.len()
        );
    }

    pub fn draw(&self, shader: &ShaderProgram) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}
