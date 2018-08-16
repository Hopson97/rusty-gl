use gl;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTarget {
    ArrayBuffer = gl::ARRAY_BUFFER
}