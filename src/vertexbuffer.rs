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
pub struct VertexBuffer {
    position_buffer: GLuint,
    normal_buffer: GLuint,
    uv_buffer: GLuint,

    handle: GLuint,

    vertex_count: usize,

    initialized: bool
}

impl Default for VertexBuffer {
    fn default() -> VertexBuffer {
        VertexBuffer {
            position_buffer: 0,
            normal_buffer: 0,
            uv_buffer: 0,
            handle: 0,
            vertex_count: 0,

            initialized: false
        }
    }
}

impl VertexBuffer {
    pub fn from_mesh(mesh: Mesh) -> Result<VertexBuffer, String> {
        let mut position_buffer = 0 as GLuint;
        let mut normal_buffer = 0 as GLuint;
        let mut uv_buffer = 0 as GLuint;

        let mut handle = 0 as GLuint;

        unsafe {
            let ptr = &(mesh.positions[0].z) as *const f32;
            let ptr2 = &(mesh.positions[1].x) as *const f32;

            gl::GenVertexArrays(1, &mut handle);
            gl::BindVertexArray(handle);

            gl::GenBuffers(1, &mut position_buffer);
            gl::GenBuffers(1, &mut normal_buffer);
            gl::GenBuffers(1, &mut uv_buffer);

            gl::BindBuffer(gl::ARRAY_BUFFER, position_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.positions.len() * 3 * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                           mesh.positions.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.normals.len() * 3 * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                           mesh.normals.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, uv_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.uvs.len() * 2 * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                           mesh.uvs.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
            gl::EnableVertexAttribArray(2);

            println!("{}", gl::GetError());
        }

        Ok(VertexBuffer { position_buffer: position_buffer, normal_buffer: normal_buffer, uv_buffer: uv_buffer, 
                            handle: handle, vertex_count: mesh.positions.len(), initialized: true })
    }

    pub fn draw(&self) {
        if self.initialized {
            unsafe {
                gl::BindVertexArray(self.handle);
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count as i32);
            }
        }
    }
}
