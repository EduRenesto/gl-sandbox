use camera::Camera;

pub trait Actor {
    fn initialize(&self);
    fn render(&self, camera: Camera);
}
