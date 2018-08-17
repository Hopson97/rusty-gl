use super::enums::*;

use gl;
use gl::types::*;
use std::os::raw::c_void;

#[derive(Clone, Copy)]
pub struct GLTexture(GLuint);

pub fn gl_gen_textures(count: GLsizei, texture: *mut GLTexture) {
    unsafe {
        gl::GenTextures(count, &mut (*texture).0);
    }
}

pub fn gl_gen_texture() -> GLTexture {
    let mut tex = GLTexture(0);
    gl_gen_textures(1, &mut tex);
    tex
}

pub fn gl_active_texture(texture: GLuint) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + texture);
    }
}

pub fn gl_bind_texture(target: GLTexTarget, texture: GLTexture) {
    unsafe {
        gl::BindTexture(target as GLenum, texture.0);
    }
}

pub fn gl_delete_textures(count: GLsizei, texture: *mut GLTexture) {
    unsafe {
        gl::DeleteTextures(count, &mut (*texture).0);
    }
}

pub fn gl_tex_image_2d(
    target: GLTexTarget,
    level: GLint,
    internal_format: GLTexFormat,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLTexFormat,
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

pub fn gl_generate_mipmap(target: GLTexTarget) {
    unsafe {
        gl::GenerateMipmap(target as GLenum);
    }
}

pub fn gl_tex_parameteri(target: GLTexTarget, param_name: GLTexParamName, param: GLTexParam) {
    unsafe {
        gl::TexParameteri(target as GLenum, param_name as GLenum, param as GLenum as i32);
    }
}
