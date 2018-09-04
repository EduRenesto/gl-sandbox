use glm;

use camera::Camera;

pub struct StaticCamera {
    view_matrix: glm::Mat4,
    projection_matrix: glm::Mat4
}

impl StaticCamera {
    pub fn new(position: glm::Vec3, target: glm::Vec3, aspect_ratio: f32, fovy: f32) -> StaticCamera {
        let view = glm::ext::look_at(position, target, glm::vec3(0.0, 1.0, 0.0));
        let proj = glm::ext::perspective_rh(fovy, aspect_ratio, 0.00001, 10000.0);

        StaticCamera {
            view_matrix: view,
            projection_matrix: proj
        }
    }
}

impl Camera for StaticCamera {
    fn get_view_matrix(&self) -> glm::Mat4 {
        self.view_matrix 
    }

    fn get_projection_matrix(&self) -> glm::Mat4 {
        self.projection_matrix 
    }
}
