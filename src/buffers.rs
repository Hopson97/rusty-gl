use gl;
use gl::types::*;
use std::mem;
use std::ptr;

use super::enums::*;

#[derive(Clone, Copy)]
pub struct VAO(GLuint);

#[derive(Clone, Copy)]
pub struct VBO(GLuint);

/// Generates vertex array objects
///
/// # Examples
/// ```
/// let mut vao = 0;
/// rgl::gen_vertex_arrays(1, &mut vao);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml
pub fn gen_vertex_arrays(count: GLsizei, arrays: *mut VAO) {
    unsafe {
        gl::GenVertexArrays(count, &mut (*arrays).0);
    }
}


/// Generates a single VAO
/// No need to create the VAO seperatly!
/// 
/// # Examples
/// ```
/// let mut vao = rgl::gen_vertex_array();
/// ```
pub fn gen_vertex_array() -> VAO {
    let mut vao = VAO(0);
    gen_vertex_arrays(1, &mut vao);
    vao
}

/// Generates some buffer objects
///
/// # Examples
/// ```
/// let mut vbo = 0;
/// rgl::gen_buffers(1, &mut vbo);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/es2.0/xhtml/glGenBuffers.xml
pub fn gen_buffers(count: GLsizei, buffers: *mut VBO) {
    unsafe {
        gl::GenBuffers(count, &mut (*buffers).0);
    }
}

/// Generates a single VBO
/// No need to create the VBO seperatly!
/// 
/// # Examples
/// ```
/// let mut vbo = rgl::gen_vertex_buffer();
/// ```
pub fn gen_buffer() -> VBO {
    let mut vbo = VBO(0);
    gen_buffers(1, &mut vbo);
    vbo
}

/// Bind a vertex array object
///
/// # Examples
/// ```
/// let mut vao = 0;
/// rgl::gen_vertex_arrays(1, &mut vao);
/// rgl::bind_vertex_array(vao);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml
pub fn bind_vertex_array(array: VAO) {
    unsafe {
        gl::BindVertexArray(array.0);
    }
}

/// Bind a vertex buffer
///
/// # Examples
/// ```
/// let mut vbo = 0;
/// rgl::gen_buffers(1, &mut vbo);
/// rgl::bind_vertex_array(vao);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/es2.0/xhtml/glGenBuffers.xml
pub fn bind_buffer(target: GLTarget, buffer: VBO) {
    unsafe {
        gl::BindBuffer(target as u32, buffer.0);
    }
}

/// Enable a generic vertex attribute array
///
/// # Examples
/// ```
/// rgl::enable_vertex_attrib_array(0);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml
pub fn enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }
}

/// Define an array of generic vertex attribute data
///
/// # Examles
/// ```
/// rgl::vertex_attrib_pointer(0, 2, GLType::Float, false, 0);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml
/// TODO that last param in a rusty way (null default for now)
pub fn vertex_attrib_pointer(
    index: GLuint,
    size: GLint,
    type_: GLType,
    normalised: bool,
    stride: GLsizei,
) {
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            type_ as GLenum,
            normalised as GLboolean,
            stride,
            ptr::null(),
        );
    }
}

/// Creates and initalizes a buffer object data store
///
/// # Examples
/// ```
/// let vertex_data: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
/// //...
/// rgl::buffer_data(GLTarget::ArrayBuffer, &vertex_data, GLUsage::StaticDraw);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml
pub fn buffer_data<T>(target: GLTarget, data: &[T], usage: GLUsage) {
    unsafe {
        gl::BufferData(
            target as GLenum,
            (data.len() * mem::size_of::<T>()) as GLsizeiptr,
            mem::transmute(&data[0]),
            usage as GLenum,
        );
    }
}

pub fn delete_buffers(count: GLsizei, buffers: *mut VBO) {
    unsafe {
        gl::DeleteBuffers(count, &mut (*buffers).0);
    }
}

pub fn delete_vertex_arrays(count: GLsizei, arrays: *mut VAO) {
    unsafe {
        gl::DeleteVertexArrays(count, &mut (*arrays).0);
    }
}
