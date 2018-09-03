use glm;
use glm::ext;

mod camera;
use camera::Camera;

struct StaticCamera {
    view_matrix: glm::Mat4,
    projection_matrix: glm::Mat4
}

impl StaticCamera {
    fn new(glm::Vec3 position, glm::Vec3 target, f32 aspect_ratio, f32 fovy) -> StaticCamera {
        let view = glm::ext::look_at(position, target, glm::vec3(0.0, 1.0, 0.0));
        let proj = glm::ext::perspective_rh(fovy, aspect_ratio, 0.00001, 10000.0);

        StaticCamera {
            view_matrix: view,
            projection_matrix: proj
        }
    }
}

impl Camera for StaticCamera {
    pub fn get_view_matrix(&self) -> glm::Mat4 {
        self.view_matrix 
    }

    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        self.projection_matrix 
    }
}
