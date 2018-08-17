use gl;
use gl::types::*;

use std::ptr;

use super::enums::*;

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

pub fn gl_draw_arrays(primitive: GLPrimitive, first: GLint, count: GLsizei) {
    unsafe {
        gl::DrawArrays(primitive as u32, first, count);
    }
}

pub fn gl_draw_elements(primitive: GLPrimitive, count: GLsizei, type_: GLType) {
    unsafe {
        gl::DrawElements(primitive as GLenum, count, type_ as GLenum, ptr::null());
    }
}