use gl;
use gl::types::*;

use std::ptr;

use super::enums::*;

/// Specify clear values for the color buffers
///
/// # Examples
/// ```
/// gl_clear_color(0.5, 0.5, 0.2, 1.0);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml
pub fn gl_clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, aplha: GLfloat) {
    unsafe {
        gl::ClearColor(red, green, blue, aplha);
    }
}

/*
pub fn gl_clear(mask: GLClearMask) {
    unsafe {
        gl::Clear(mask as GLenum);
    }
}
*/

/// Render primitives from array data
///
/// # Examples
/// ```
/// gl_bind_buffer(GLTarget::ArrayBuffer, vbo);
/// gl_draw_arrays(GLPrimitive::Triangles, 0, 3);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml
pub fn gl_draw_arrays(primitive: GLPrimitive, first: GLint, count: GLsizei) {
    unsafe {
        gl::DrawArrays(primitive as u32, first, count);
    }
}

/// Render primitives from array data
///
/// # Examples
/// ```
/// gl_bind_vertex_array(GLTarget::ArrayBuffer, vao);
/// gl_draw_elements(GLPrimitive::Triangles, 36, GLType::UInt);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml
/// TODO: ptr thing
pub fn gl_draw_elements(primitive: GLPrimitive, count: GLsizei, type_: GLType) {
    unsafe {
        gl::DrawElements(primitive as GLenum, count, type_ as GLenum, ptr::null());
    }
}

/// Draw multiple instances of a set of elements
///
/// # Examples
/// ```
/// gl_bind_vertex_array(GLTarget::ArrayBuffer, vao);
/// gl_draw_elements_instanced(GLPrimitive::Triangles, 36, GLType::UInt, 8);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstanced.xhtml
/// TODO: Ptr thing
pub fn gl_draw_elements_instanced(
    primitive: GLPrimitive,
    count: GLsizei,
    type_: GLType,
    prim_count: GLsizei,
) {
    unsafe {
        gl::DrawElementsInstanced(
            primitive as GLenum,
            count,
            type_ as GLenum,
            ptr::null(),
            prim_count,
        );
    }
}


/// render primitives from array data
/// 
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElements.xhtml
pub fn gl_draw_range_elements(mode: GLPrimitive, start: GLuint, end: GLuint, count: GLsizei, type_: GLType) {
    unsafe {
        gl::glDrawRangeElements(mode as GLenum, start, end, count, type_ as GLenum, ptr::null());
    }
}