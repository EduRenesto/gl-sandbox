extern crate glm;

pub struct Mesh<'a> {
    positions: &'a [glm::Vec3],
    normals: &'a [glm::Vec3],
    uvs: &'a [glm::Vec2],
    indices: &'a [u8]
}
