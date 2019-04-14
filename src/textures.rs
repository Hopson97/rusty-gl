use super::enums;

use gl;
use gl::types::*;
use std::os::raw::c_void;

#[derive(Clone, Copy)]
pub struct Texture(GLuint);

pub fn gen_textures(count: GLsizei, texture: *mut Texture) {
    unsafe {
        gl::GenTextures(count, &mut (*texture).0);
    }
}

pub fn gen_texture() -> Texture {
    let mut tex = Texture(0);
    gen_textures(1, &mut tex);
    tex
}

pub fn active_texture(texture: GLuint) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + texture);
    }
}

pub fn bind_texture(target: enums::TexTarget, texture: Texture) {
    unsafe {
        gl::BindTexture(target as GLenum, texture.0);
    }
}

pub fn delete_textures(count: GLsizei, texture: *mut Texture) {
    unsafe {
        gl::DeleteTextures(count, &mut (*texture).0);
    }
}

pub fn delete_texture(texture: *mut Texture) {
    delete_textures(1, texture);
}

pub fn tex_image_2d(
    target: enums::TexTarget,
    level: GLint,
    internal_format: enums::TexFormat,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: enums::TexFormat,
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

pub fn generate_mipmap(target: enums::TexTarget) {
    unsafe {
        gl::GenerateMipmap(target as GLenum);
    }
}

pub fn tex_parameteri(target: enums::TexTarget, param_name: enums::TexParamName, param: enums::TexParam) {
    unsafe {
        gl::TexParameteri(target as GLenum, param_name as GLenum, param as GLenum as i32);
    }
}
