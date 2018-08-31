use std::mem;
use std::ptr;

use gl;
use gl::types::*;

use glm;

use mesh::Mesh;

/*
 * This in the future will be way more versatile.
 * Since I just want to test stuff for now,
 * let's assume that every vertex buffer 
 * will have 3 buffers:
 *  - positions;
 *  - normals;
 *  - texture coordinates
 */
struct VertexBuffer {
    position_buffer: GLuint,
    normal_buffer: GLuint,
    uv_buffer: GLuint,

    handle: GLuint,

    vertex_count: u16 
}

impl VertexBuffer {
    pub fn from_mesh(mesh: Mesh) -> Result<VertexBuffer, String> {
        let mut position_buffer = 0 as GLuint;
        let mut normal_buffer = 0 as GLuint;
        let mut uv_buffer = 0 as GLuint;

        let mut handle = 0 as GLuint;

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
            gl::BindVertexArray(handle);

            gl::GenBuffers(1, &mut position_buffer);
            gl::GenBuffers(1, &mut normal_buffer);
            gl::GenBuffers(1, &mut uv_buffer);

            gl::BindBuffer(gl::ARRAY_BUFFER, position_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.positions.len() * 3 * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                           mem::transmute(mesh.positions.as_ptr()), gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
        }

        Err("not implemented".to_string())
    }
}
