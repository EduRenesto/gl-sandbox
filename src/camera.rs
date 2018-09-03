use glm;

pub trait Camera {
    fn get_view_matrix(&self) -> glm::Mat4;
    fn get_projection_matrix(&self) -> glm::Mat4;
}
