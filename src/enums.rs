use gl;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTarget {
    ArrayBuffer = gl::ARRAY_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLUsage {
    StaticDraw = gl::STATIC_DRAW,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLType {
    Float = gl::FLOAT,

    UByte = gl::UNSIGNED_BYTE,
    UShort = gl::UNSIGNED_SHORT,
    UInt = gl::UNSIGNED_INT,
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
    Triangles = gl::TRIANGLES,
}
pub type GLMode = GLPrimitive;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLShaderInfoParam {
    ShaderType = gl::SHADER_TYPE,
    DeleteStatus = gl::DELETE_STATUS,
    CompileStatus = gl::COMPILE_STATUS,
    InfoLogLength = gl::INFO_LOG_LENGTH,
    ShaderSourceLength = gl::SHADER_SOURCE_LENGTH,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTexTarget {
    _2D = gl::TEXTURE_2D,
    CubeMap = gl::TEXTURE_CUBE_MAP
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTexFormat {
    RGB = gl::RGB,
    RGBA = gl::RGBA,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTexParamName {
    MinFilter = gl::TEXTURE_MIN_FILTER,
    MagFilter = gl::TEXTURE_MAG_FILTER,
    WrapS = gl::TEXTURE_WRAP_S,
    WrapT = gl::TEXTURE_WRAP_T  
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GLTexParam {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST
}