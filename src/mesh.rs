extern crate glm;

use util;

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

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        for line in (&contents).lines() {
            if line.starts_with("v ") {

            } else if line.starts_with("vn ") {

            } else if line.starts_with("vt ") {

            } else if line.starts_with("f ") {

            } else {

            }
        }

        Ok(Mesh {
            positions: positions,
            normals: normals,
            uvs: uvs,
            indices: Vec::new(),
            use_positions: true,
            use_normals: true,
            use_uvs: true,
            indexed: false
        })
    }
}
