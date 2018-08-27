extern crate glm;

pub struct Mesh<'a> {
    positions: &'a [glm::Vec3],
    normals: &'a [glm::Vec3],
    uvs: &'a [glm::Vec2],
    indices: &'a [u8],

    use_positions: bool,
    use_normals: bool,
    use_uvs: bool,
    indexed: bool
}
