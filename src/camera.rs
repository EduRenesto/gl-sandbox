use glm;

trait Camera {
    pub fn get_view_matrix(&self) -> glm::Mat4;
    pub fn get_projection_matrix(&self) -> glm::Mat4;
}
