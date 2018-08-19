use gl;
use gl::types::*;

use std::ptr;

use super::enums;

/// Specify clear values for the color buffers
///
/// # Examples
/// ```
/// rgl::clear_color(0.5, 0.5, 0.2, 1.0);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml
pub fn clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, aplha: GLfloat) {
    unsafe {
        gl::ClearColor(red, green, blue, aplha);
    }
}

/*
pub fn clear(mask: GLClearMask) {
    unsafe {
        gl::Clear(mask as GLenum);
    }
}
*/

/// Render primitives from array data
///
/// # Examples
/// ```
/// rgl::bind_buffer(rgl::Target::ArrayBuffer, vbo);
/// rgl::draw_arrays(rgl::Primitive::Triangles, 0, 3);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml
pub fn draw_arrays(primitive: enums::Primitive, first: GLint, count: GLsizei) {
    unsafe {
        gl::DrawArrays(primitive as u32, first, count);
    }
}

/// Render primitives from array data
///
/// # Examples
/// ```
/// rgl::bind_vertex_array(rgl::Target::ArrayBuffer, vao);
/// rgl::draw_elements(rgl::Primitive::Triangles, 36, rgl::Type::UInt);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml
/// TODO: ptr thing
pub fn draw_elements(primitive: enums::Primitive, count: GLsizei, type_: enums::Type) {
    unsafe {
        gl::DrawElements(primitive as GLenum, count, type_ as GLenum, ptr::null());
    }
}

/// Draw multiple instances of a set of elements
///
/// # Examples
/// ```
/// rgl::bind_vertex_array(rgl::Target::ArrayBuffer, vao);
/// rgl::draw_elements_instanced(rgl::Primitive::Triangles, 36, rgl::Type::UInt, 8);
/// ```
///
/// More: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstanced.xhtml
/// TODO: Ptr thing
pub fn draw_elements_instanced(
    primitive: enums::Primitive,
    count: GLsizei,
    type_: enums::Type,
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
pub fn draw_range_elements(mode: enums::Primitive, start: GLuint, end: GLuint, count: GLsizei, type_: enums::Type) {
    unsafe {
        gl::DrawRangeElements(mode as GLenum, start, end, count, type_ as GLenum, ptr::null());
    }
}