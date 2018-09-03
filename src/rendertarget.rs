use gl;
use gl::types::*;

// Again, this is a very early implementation.
// For now, we'll only use the render target
// to create the gbuffer.
// So, we'll have a defined numer of attachments.
pub struct RenderTarget {
    handle: GLuint,
    albedo_tex: GLuint,
    normal_tex: GLuint,
    position_tex: GLuint,
    depth_rb: GLuint
}

impl RenderTarget {
    pub fn new(width: u8, height: u8) -> Result<RenderTarget, String> {
        let mut handle = 0 as GLuint;
        let mut albedo_tex= 0 as GLuint;
        let mut normal_tex = 0 as GLuint;
        let mut position_tex = 0 as GLuint;
        let mut depth_rb = 0 as GLuint;

        unsafe {
            gl::GenTextures(1, &mut albedo_tex);
            gl::BindTexture(gl::TEXTURE_2D, albedo_tex);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA, width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE, 0);

            gl::GenTextures(1, &mut normal_tex);
            gl::BindTexture(gl::TEXTURE_2D, normal_tex);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB, width as GLsizei, height as GLsizei, 0, gl::RGB, gl::UNSIGNED_BYTE, 0);

            gl::GenTextures(1, &mut position_tex);
            gl::BindTexture(gl::TEXTURE_2D, position_tex);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB, width as GLsizei, height as GLsizei, 0, gl::RGB, gl::UNSIGNED_BYTE, 0);

            gl::GenRenderBuffers(1, &mut depth_rb);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depth_rb);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width as GLsizei, height as GLsizei);

            gl::GenFramebuffers(1, &mut handle);
            gl::BindFramebuffer(gl::FRAMEBUFFER, handle);

            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, albedo_tex);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D, normal_tex);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT2, gl::TEXTURE_2D, position_tex);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_rb);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE as GLenum {
                return Err("Framebuffer is not complete!");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        let rt = RenderTarget {
            handle: handle,
            albedo_tex: albedo_tex,
            normal_tex: normal_tex,
            position_tex: position_tex,
            depth_rb: depth_rb
        }

        Ok(rt)
    }

    pub fn use(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
        }
    }
}
