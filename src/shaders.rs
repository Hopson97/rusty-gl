use super::enums::*;

use gl;
use gl::types::*;
use std::ptr;
use std::ffi::CString;

pub fn gl_create_shader(type_: GLShaderType) -> GLuint {
    unsafe {
        gl::CreateShader(type_ as GLenum)
    }
}

/// TODO: MISSING TWO PARAMS (to do with count)
pub fn gl_shader_source(shader: GLuint, source: &str) {
    unsafe {
        gl::ShaderSource(shader, 1, &CString::new(source).unwrap().as_ptr(), ptr::null());
    }
}

pub fn gl_compile_shader(shader: GLuint) {
    unsafe {
        gl::CompileShader(shader);
    }
}

pub fn gl_get_shader_iv(shader: GLuint, parameter: GLShaderInfoParam, status: &mut GLint) {
    unsafe {
        gl::GetShaderiv(shader, parameter as GLenum, status);
    }
}

pub fn get_shader_info_log(shader: GLuint, buffer_size: GLsizei, length: &mut GLsizei, info_log: &mut Vec<GLchar>) {
    unsafe {
        gl::GetShaderInfoLog(shader, buffer_size, length, info_log.as_mut_ptr() as *mut GLchar);
    }
}