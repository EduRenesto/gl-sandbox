extern crate glm;

use util;

// TODO usar Option<T>
pub struct Mesh {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub uvs: Vec<glm::Vec2>,
    pub indices: Vec<u16>,

    pub use_positions: bool,
    pub use_normals: bool,
    pub use_uvs: bool,
    pub indexed: bool
}

impl Mesh {
    pub fn from_obj(filename: &str) -> Result<Mesh, String> {
        let contents = util::read_file(filename)?;

        let mut positions: Vec<glm::Vec3> = Vec::new();
        let mut normals: Vec<glm::Vec3> = Vec::new();
        let mut uvs: Vec<glm::Vec2> = Vec::new();

        let mut indexed_positions: Vec<glm::Vec3> = Vec::new();
        let mut indexed_normals: Vec<glm::Vec3> = Vec::new();
        let mut indexed_uvs: Vec<glm::Vec2> = Vec::new();

        for line in (&contents).lines() {
            // TODO refactor these
            if line.starts_with("v ") {
                let tokens: Vec<&str> = line.split(" ").collect();
    
                let x: f32 = tokens[1].parse::<f32>().expect("malformed obj file!");
                let y: f32 = tokens[2].parse::<f32>().expect("malformed obj file!");
                let z: f32 = tokens[3].parse::<f32>().expect("malformed obj file!");

                positions.push(glm::vec3(x, y, z));
            } else if line.starts_with("vn ") {
                let tokens: Vec<&str> = line.split(" ").collect();
    
                let x: f32 = tokens[1].parse::<f32>().expect("malformed obj file!");
                let y: f32 = tokens[2].parse::<f32>().expect("malformed obj file!");
                let z: f32 = tokens[3].parse::<f32>().expect("malformed obj file!");

                normals.push(glm::vec3(x, y, z));
            } else if line.starts_with("vt ") {
                let tokens: Vec<&str> = line.split(" ").collect();
    
                let u: f32 = tokens[1].parse::<f32>().expect("malformed obj file!");
                let v: f32 = tokens[2].parse::<f32>().expect("malformed obj file!");

                uvs.push(glm::vec2(u, v));
            } else if line.starts_with("f ") {
                let tokens: Vec<&str> = line.split(" ").collect();

                let t0: Vec<&str> = tokens[1].split("/").collect();
                let t1: Vec<&str> = tokens[2].split("/").collect();
                let t2: Vec<&str> = tokens[3].split("/").collect();
                
                indexed_positions.push(positions[t0[0].parse::<usize>().expect("malformed obj file!") - 1]);
                indexed_positions.push(positions[t1[0].parse::<usize>().expect("malformed obj file!") - 1]);
                indexed_positions.push(positions[t2[0].parse::<usize>().expect("malformed obj file!") - 1]);

                indexed_uvs.push(uvs[t0[1].parse::<usize>().expect("malformed obj file!") - 1]);
                indexed_uvs.push(uvs[t1[1].parse::<usize>().expect("malformed obj file!") - 1]);
                indexed_uvs.push(uvs[t2[1].parse::<usize>().expect("malformed obj file!") - 1]);

                indexed_normals.push(normals[t0[2].parse::<usize>().expect("malformed obj file!") - 1]);
                indexed_normals.push(normals[t1[2].parse::<usize>().expect("malformed obj file!") - 1]);
                indexed_normals.push(normals[t2[2].parse::<usize>().expect("malformed obj file!") - 1]);
            } 
        }

        Ok(Mesh {
            positions: indexed_positions,
            normals: indexed_normals,
            uvs: indexed_uvs,
            indices: Vec::new(),
            use_positions: true,
            use_normals: true,
            use_uvs: true,
            indexed: false
        })
    }
}
