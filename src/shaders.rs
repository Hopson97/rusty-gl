use super::enums::*;

use gl;
use gl::types::*;
use std::ffi::CString;
use std::ptr;

#[derive(Clone, Copy)]
pub struct GLShader(GLuint);

#[derive(Clone, Copy)]
pub struct GLProgram(GLuint);

#[derive(Clone, Copy)]
pub struct GLUniformLocation(GLint);

pub fn gl_create_program() -> GLProgram {
    unsafe { GLProgram(gl::CreateProgram()) }
}

pub fn gl_create_shader(type_: GLShaderType) -> GLShader {
    unsafe { GLShader(gl::CreateShader(type_ as GLenum)) }
}

/// TODO: MISSING TWO PARAMS (to do with count)
pub fn gl_shader_source(shader: GLShader, source: &str) {
    unsafe {
        gl::ShaderSource(
            shader.0,
            1,
            &CString::new(source).unwrap().as_ptr(),
            ptr::null(),
        );
    }
}

pub fn gl_compile_shader(shader: GLShader) {
    unsafe {
        gl::CompileShader(shader.0);
    }
}

pub fn gl_get_shader_iv(shader: GLShader, parameter: GLShaderInfoParam, status: &mut GLint) {
    unsafe {
        gl::GetShaderiv(shader.0, parameter as GLenum, status);
    }
}

pub fn gl_get_shader_info_log(
    shader: GLShader,
    buffer_size: GLsizei,
    length: &mut GLsizei,
    info_log: &mut Vec<GLchar>,
) {
    unsafe {
        gl::GetShaderInfoLog(
            shader.0,
            buffer_size,
            length,
            info_log.as_mut_ptr() as *mut GLchar,
        );
    }
}

pub fn gl_attach_shader(program: GLProgram, shader: GLShader) {
    unsafe {
        gl::AttachShader(program.0, shader.0);
    }
}

pub fn gl_link_program(program: GLProgram) {
    unsafe {
        gl::LinkProgram(program.0);
    }
}

pub fn gl_use_program(program: GLProgram) {
    unsafe {
        gl::UseProgram(program.0);
    }
}

pub fn gl_delete_program(program: GLProgram) {
    unsafe {
        gl::DeleteProgram(program.0);
    }
}

pub fn gl_delete_shader(shader: GLShader) {
    unsafe {
        gl::DeleteShader(shader.0);
    }
}


///Shader uniforms


pub fn gl_get_uniform_location(program: GLProgram, name: &str) -> GLUniformLocation {
    unsafe {
        GLUniformLocation(gl::GetUniformLocation(program.0, CString::new(name).unwrap().as_ptr()))
    }
}


//Only going to use commonly used ones for now, may add the rest later