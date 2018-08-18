# Rusty GL

Note: This is a work in progress.

Crate: https://crates.io/crates/rgl

## Summary

Very thin wrapper over gl-rs, aiming to make code more "rust-like" and safer, while still allowing the control you have with classic OpenGL code.

# Current features

* Vertex buffer objects
* Vertex array objects
* Shaders
* Texture 2D
* Single-tuple struct such as `VAO(GLuint)` to enforce correct OpenGL object type is passed in functions

# Tuple Struct

Current tuple structs:

* `pub struct VBO(GLuint);` for vertex buffer objects
* `pub struct VAO(GLuint);` for vertex array objects
* `pub struct GLTexture(GLuint);` for texture objects
* `pub struct GLShader(GLuint);` for programs/ shader object

These structs have no implemention (assosiated function), they are just there to enforce that the correct OpenGL functions are passed into functions.
For example:

```rust
let mut vao = gl_gen_vertex_array();
gl_bind_buffer(GLTarget::ArrayBuffer, vao);
```

would not work, as `gl_bind_buffer` expects type `struct VBO(GLuint)`, but vao is of type `VAO(GLuint)`.

# Roadmap

* Framebuffer objects
* More OpenGL functions (Right now there is basically only the minimum!)

# Usage
```toml
[dependancies]
rgl = "0.1.1"
gl = "0.6.0"
```

The `gl-rs` create (`gl`) is still needed for certain things such as window proc address and types (eg `GLuint`, `GLfloat` etc)

## Examples

As mentioned above, this crate aims to be a more rust-like alternative to gl-rs.

For example, code from gl-rs such as:

```rust
let mut vao = 0;
let mut vbo = 0;

unsafe {
    // Create Vertex Array Object
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        mem::transmute(&VERTEX_DATA[0]),
        gl::STATIC_DRAW,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
        0,
        2,
        gl::FLOAT,
        gl::FALSE as GLboolean,
        0,
        ptr::null(),
    );
}
```

Is instead written like

```rust
    //Create a vertex array object and a vertex buffer object
    let mut vao = rgl::gen_vertex_array();
    let mut vbo = rgl::gen_buffer();

    //Generate and bind the VAO
    rgl::gl_bind_vertex_array(vao);

    //Generate and bind the VBO
    rgl::gl_bind_buffer(rgl::GLTarget::ArrayBuffer, vbo);

    //Buffer the vertex data and tell OpenGL the structure
    rgl::buffer_data(rgl::GLTarget::ArrayBuffer, &VERTEX_DATA, rgl::GLUsage::StaticDraw);
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_pointer(0, 2, rgl::GLType::Float, false, 0);
```

Changes include:

* snake_case over PascalCase for function names
* rgl crate
* No need for the `unsafe {...}` blocks
* Strongly typed enums over the error-prone GLenum (Where you can easily pass the incorrect enum)
* No need to cast types to `std::os::raw::c_void`, rusty-gl will do this for you
* No need to cast string to `std::ffi::CString`, rusty-gl will do this for you

## More examples

Examples on using this crate can be found in the examples folder: https://github.com/Hopson97/rusty-gl/tree/master/Examples

Run the examples with `cargo run`.

