use gl;
use gl::types::*;
use std::mem;
use std::ptr;

use super::enums::*;

/// Generates vertex array objects
/// 
/// # Examples
/// ```
/// let mut vao = 0;
/// gl_gen_vertex_arrays(1, &mut vao);
/// ```
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml
pub fn gl_gen_vertex_arrays(count: GLsizei, arrays: *mut GLuint) {
    unsafe {
        gl::GenVertexArrays(count, arrays);
    }
}

/// Generates some buffer objects
/// 
/// # Examples
/// ```
/// let mut vbo = 0;
/// gl_gen_buffers(1, &mut vbo);
/// ```
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/es2.0/xhtml/glGenBuffers.xml
pub fn gl_gen_buffers(count: GLsizei, buffers: *mut GLuint) {
    unsafe {
        gl::GenBuffers(count, buffers);
    }
}

/// Bind a vertex array object
/// 
/// # Examples
/// ```
/// let mut vao = 0;
/// gl_gen_vertex_arrays(1, &mut vao);
/// gl_bind_vertex_array(vao);
/// ```
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml
pub fn gl_bind_vertex_array(array: GLuint) {
    unsafe {
        gl::BindVertexArray(array);
    }
}

/// Bind a vertex buffer
/// 
/// # Examples
/// ```
/// let mut vbo = 0;
/// gl_gen_buffers(1, &mut vbo);
/// gl_bind_vertex_array(vao);
/// ```
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/es2.0/xhtml/glGenBuffers.xml
pub fn gl_bind_buffer(target: GLTarget, buffer: GLuint) {
    unsafe {
        gl::BindBuffer(target as u32, buffer);
    }
}

/// Enable a generic vertex attribute array
/// 
/// # Examples
/// ```
/// gl_enable_vertex_attrib_array(0);
/// ```
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml
pub fn gl_enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

/// Define an array of generic vertex attribute data
/// 
/// # Examles
/// ```
/// gl_vertex_attrib_pointer(0, 2, GLType::Float, false, 0);
/// ```
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml
/// TODO that last param in a rusty way (null default for now)
pub fn gl_vertex_attrib_pointer(
    index: GLuint, 
    size: GLint, 
    type_: GLType, 
    normalised: bool, 
    stride: GLsizei) 
{
    unsafe {
        gl::VertexAttribPointer(
            index, 
            size, 
            type_ as GLenum, 
            normalised as GLboolean, 
            stride, 
            ptr::null());
    }
}

/// Creates and initalizes a buffer object data store
/// 
/// # Examples
/// ```
/// let vertex_data: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
/// //...
/// gl_buffer_data(GLTarget::ArrayBuffer, &vertex_data, GLUsage::StaticDraw);
/// ```
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
pub fn gl_buffer_data<T>(target: GLTarget, data: &[T], usage: GLUsage) {
    unsafe {
        gl::BufferData(
            target as GLenum,
            (data.len() * mem::size_of::<T>()) as GLsizeiptr,
            mem::transmute(&data[0]),
            usage as GLenum
        );
    }
}