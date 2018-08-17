use gl;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTarget {
    ArrayBuffer = gl::ARRAY_BUFFER
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLUsage {
    StaticDraw = gl::STATIC_DRAW
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLType {
    Float = gl::FLOAT,

    UByte = gl::UNSIGNED_BYTE,
    UShort = gl::UNSIGNED_SHORT,
    UInt = gl::UNSIGNED_INT
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLClearMask {
    ColorBufferBit = gl::COLOR_BUFFER_BIT,
    DepthBufferBit = gl::DEPTH_BUFFER_BIT,
    StencilBufferBit = gl::STENCIL_BUFFER_BIT,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLPrimitive {
    Triangles = gl::TRIANGLES
}
pub type GLMode = GLPrimitive;