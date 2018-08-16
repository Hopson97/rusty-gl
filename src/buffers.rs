use gl;
use gl::types::*;
use std::mem;
use std::ptr;


/// Generates vertex array objects
/// 
/// More info: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml
pub fn gl_gen_vertex_arrays(count: GLsizei, arrays: *mut GLuint) {
    unsafe {
        gl::GenVertexArrays(count, arrays);
    }
}

/// Generates some buffer objects
/// 
/// More info: https://www.khronos.org/registry/OpenGL-Refpages/es2.0/xhtml/glGenBuffers.xml
pub fn gl_gen_buffers(count: GLsizei, buffers: *mut GLuint) {
    unsafe {
        gl::GenBuffers(count, buffers);
    }
}

/// Bind a vertex array object
/// 
/// More info: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml
pub fn gl_bind_vertex_array(array: GLuint) {
    unsafe {
        gl::BindVertexArray(array);
    }
}

/// Bind a vertex buffer
/// 
/// More info: https://www.khronos.org/registry/OpenGL-Refpages/es2.0/xhtml/glGenBuffers.xml
pub fn gl_bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe {
        gl::BindBuffer(target, buffer);
    }
}

/// Enable a generic vertex attribute array
/// 
/// More info: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml
pub fn gl_enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

/// Define an array of generic vertex attribute data
/// 
/// More info: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml
/// TODO that last param in a rusty way (null default for now)
pub fn gl_vertex_attrib_pointer(
    index: GLuint, 
    size: GLint, 
    type_: GLenum, 
    normalised: bool, 
    stride: GLsizei) 
{
    unsafe {
        gl::VertexAttribPointer(
            index, 
            size, 
            type_, 
            normalised as GLboolean, 
            stride, 
            ptr::null());
    }
}

/// Creates and initalizes a buffer object data store
/// 
/// More info: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
pub fn gl_buffer_data<T>(target: GLenum, data: &[T], usage: GLenum) {
    unsafe {
        gl::BufferData(
            target,
            (data.len() * mem::size_of::<T>()) as GLsizeiptr,
            mem::transmute(&data[0]),
            usage
        );
    }
}