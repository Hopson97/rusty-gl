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
pub fn gl_uniform1f(location: GLUniformLocation, v0: GLfloat) {
    unsafe {
        gl::Uniform1f(location.0, v0);
    }
}

pub fn gl_uniform2f(location: GLUniformLocation, v0: GLfloat, v1: GLfloat) {
    unsafe {
        gl::Uniform2f(location.0, v0, v1);
    }
}

pub fn gl_uniform3f(location: GLUniformLocation, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    unsafe {
        gl::Uniform3f(location.0, v0, v1, v2);
    }
}

pub fn gl_uniform4f(location: GLUniformLocation, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    unsafe {
        gl::Uniform4f(location.0, v0, v1, v2, v3);
    }
}

pub fn gl_uniform1i(location: GLUniformLocation, v0: GLint) {
    unsafe {
        gl::Uniform1i(location.0, v0);
    }
}

pub fn gl_uniform2i(location: GLUniformLocation, v0: GLint, v1: GLint) {
    unsafe {
        gl::Uniform2i(location.0, v0, v1);
    }
}

pub fn gl_uniform3i(location: GLUniformLocation, v0: GLint, v1: GLint, v2: GLint) {
    unsafe {
        gl::Uniform3i(location.0, v0, v1, v2);
    }
}

pub fn gl_uniform4i(location: GLUniformLocation, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    unsafe {
        gl::Uniform4i(location.0, v0, v1, v2, v3);
    }
}

pub fn gl_uniform1ui(location: GLUniformLocation, v0: GLuint) {
    unsafe {
        gl::Uniform1u(location.0, v0);
    }
}

pub fn gl_uniform2ui(location: GLUniformLocation, v0: GLuint, v1: GLuint) {
    unsafe {
        gl::Uniform2u(location.0, v0, v1);
    }
}

pub fn gl_uniform3ui(location: GLUniformLocation, v0: GLuint, v1: GLuint, v2: GLuint) {
    unsafe {
        gl::Uniform3i(location.0, v0, v1, v2);
    }
}

pub fn gl_uniform4ui(location: GLUniformLocation, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    unsafe {
        gl::Uniform4i(location.0, v0, v1, v2, v3);
    }
}