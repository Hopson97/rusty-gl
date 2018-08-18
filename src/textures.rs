
pub mod rgl {

    use ::enums::*;

    use gl;
    use gl::types::*;
    use std::os::raw::c_void;

    #[derive(Clone, Copy)]
    pub struct GLTexture(GLuint);

    pub fn gen_textures(count: GLsizei, texture: *mut GLTexture) {
        unsafe {
            gl::GenTextures(count, &mut (*texture).0);
        }
    }

    pub fn gen_texture() -> GLTexture {
        let mut tex = GLTexture(0);
        gen_textures(1, &mut tex);
        tex
    }

    pub fn active_texture(texture: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture);
        }
    }

    pub fn bind_texture(target: rgl::TexTarget, texture: GLTexture) {
        unsafe {
            gl::BindTexture(target as GLenum, texture.0);
        }
    }

    pub fn delete_textures(count: GLsizei, texture: *mut GLTexture) {
        unsafe {
            gl::DeleteTextures(count, &mut (*texture).0);
        }
    }

    pub fn tex_image_2d(
        target: rgl::TexTarget,
        level: GLint,
        internal_format: rgl::TexFormat,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: rgl::TexFormat,
        pixels: &Vec<u8>,
    ) {
        unsafe {
            gl::TexImage2D(
                target as GLenum,
                level,
                internal_format as i32,
                width,
                height,
                border,
                format as GLenum,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void,
            );
        }
    }

    pub fn generate_mipmap(target: rgl::TexTarget) {
        unsafe {
            gl::GenerateMipmap(target as GLenum);
        }
    }

    pub fn tex_parameteri(target: rgl::TexTarget, param_name: rgl::TexParamName, param: rgl::TexParam) {
        unsafe {
            gl::TexParameteri(target as GLenum, param_name as GLenum, param as GLenum as i32);
        }
    }
}