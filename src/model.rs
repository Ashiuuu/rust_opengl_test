use {
    glm::{vec2, vec3},
    std::path::Path,
};

use crate::{
    draw::Draw,
    mesh::Mesh,
    mesh::Vertex,
    shader_program::ShaderProgram,
    texture::{Texture2D, TextureType},
};

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture2D>,
    directory: String,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let (prefix, _) = path.split_once(".").unwrap();
        let file = Path::new("ressources")
            .join("models")
            .join(prefix)
            .join(path);

        let mut model = Model::default();

        model.load(file.as_os_str().to_str().unwrap());

        model
    }

    fn load(&mut self, path: &str) {
        let path = Path::new(path);

        self.directory = path
            .parent()
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap()
            .into();
        let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);

        let (models, materials) =
            obj.expect(format!("filename: \"{}\"", path.to_str().unwrap()).as_str());
        let materials = materials.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let p = &mesh.positions;
            let n = &mesh.normals;
            let t = &mesh.texcoords;

            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
                    tex_coords: vec2(t[i * 2], t[i * 2 + 1]),
                });
                //println!("tex_coords: ({}, {})", t[i * 2], t[i * 2 + 1]);
            }

            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                if !material.diffuse_texture.is_empty() {
                    let texture =
                        self.load_material(&material.diffuse_texture, TextureType::Diffuse);
                    textures.push(texture);
                }
                if !material.specular_texture.is_empty() {
                    let texture =
                        self.load_material(&material.specular_texture, TextureType::Specular);
                    textures.push(texture);
                }
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    fn load_material(&mut self, path: &str, ty: TextureType) -> Texture2D {
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let texture = Texture2D::from_image(path, &self.directory, ty);
        self.textures_loaded.push(texture.clone());
        texture
    }
}

impl Draw for Model {
    fn draw(&self, shader: &ShaderProgram) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}
