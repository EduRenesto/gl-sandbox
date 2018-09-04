extern crate glfw;
extern crate gl;
extern crate glm;

use glfw::{Action, Context, Key};

mod util;
mod mesh;
mod camera;
mod static_camera;
mod shader;
mod vertexbuffer;
mod actor;

use shader::Shader;
use vertexbuffer::VertexBuffer;
use mesh::Mesh;
use actor::Actor;
use camera::Camera;
use static_camera::StaticCamera;

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

fn init_gl() {
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "GL Sandbox", glfw::WindowMode::Windowed)
                                .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    init_gl();

    let s = Shader::new(&[(gl::VERTEX_SHADER, "res/shaders/test.vs"),
                            (gl::FRAGMENT_SHADER, "res/shaders/test.fs")]).expect("kek");

    let camera = StaticCamera::new(glm::vec3(3.0, 3.0, 3.0), 
                                    glm::vec3(0.0, 0.0, 0.0),
                                    16.0/9.0, 0.7853);

    let mut initialized = false;
    let mut test_quad = VertexBuffer::default();

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if !initialized {
            let zero3 = glm::vec3(0.0, 0.0, 0.0);
            let zero2 = glm::vec2(0.0, 0.0);

            //test_quad = VertexBuffer::from_mesh(Mesh{
            //    positions: vec![glm::vec3(-1.0, 1.0, 0.0), glm::vec3(0.0, -1.0, 0.0), glm::vec3(1.0, 1.0, 0.0)],
            //    normals: vec![zero3, zero3, zero3],
            //    uvs: vec![zero2, zero2],
            //    indices: vec![],

            //    use_positions: true,
            //    use_normals: false,
            //    use_uvs: false,
            //    indexed: false
            //}).unwrap();

            test_quad = VertexBuffer::from_mesh(Mesh::from_obj("res/models/suzanne.obj").expect("failed to load obj file"))
                .expect("failed to create vertex buffer");

            initialized = true;
        } else {
            s.bind();
            s.uniform_matrix4("viewMatrix", camera.get_view_matrix());
            s.uniform_matrix4("projMatrix", camera.get_projection_matrix());
            test_quad.draw();
        }

        window.render_context().swap_buffers();
    }
}
