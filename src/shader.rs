extern crate gl;

use gl::types::*;

use std::str;
use std::ptr;
use std::ffi::CString;

use util;

pub struct Shader {
    program: GLuint
}

impl Shader {
    pub fn new(shaders: &[(GLenum, &str)]) -> Result<Shader, String> {
        let mut handles = Vec::new();

        for (shader_type, file_name) in shaders {
            if shader_type != &gl::VERTEX_SHADER && shader_type != &gl::FRAGMENT_SHADER &&
               shader_type != &gl::COMPUTE_SHADER && shader_type != &gl::GEOMETRY_SHADER {
                return Err("shader_type isnt supported".to_string());
            }

            let source = util::read_file(file_name)?;

            unsafe {
                let shader = gl::CreateShader(*shader_type);
                let c_src = CString::new((&source).as_bytes()).unwrap();

                gl::ShaderSource(shader, 1, &c_src.as_ptr(), ptr::null());
                gl::CompileShader(shader);

                let mut status = gl::FALSE as GLint;
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

                if status != (gl::TRUE as GLint) {
                    let mut len = 0;
                    gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                    let mut buf = Vec::with_capacity(len as usize);
                    buf.set_len((len as usize) - 1);
                    gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);

                    let err = str::from_utf8(&buf).expect("driver spat non-utf8 things");

                    return Err(err.to_string());
                }

                handles.push(shader);
            }
        }

        unsafe {
            let program = gl::CreateProgram();
            for shader in handles.iter() {
                gl::AttachShader(program, *shader);
            }

            gl::LinkProgram(program);

            let mut link_status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_status);

            if link_status != (gl::TRUE as GLint) {
                return Err("Shader program failed to link".to_string());
            }

            let ret = Shader { program: program };

            Ok(ret)
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}
