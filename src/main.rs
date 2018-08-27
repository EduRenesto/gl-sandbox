extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use gl::types::*;

mod mesh;

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

fn init_gl() {
    unsafe {
        gl::ClearColor(0.0, 0.0, 1.0, 1.0);
    }
}

fn render() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "GL Sandbox", glfw::WindowMode::Windowed)
                                .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    init_gl();

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

        render();

        window.render_context().swap_buffers();
    }
}
