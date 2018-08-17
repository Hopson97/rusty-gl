# Rusty GL

Note: This is a work in progress.

## Summary

Very thin wrapper over gl-rs, aiming to make code more "rust-like" and safer, while still allowing the control you have with classic OpenGL code.

Wrapper over:

* Vertex buffer objects
* Vertex array objects
* Shaders
* Texture 2D

More to come eventually

# Usage
```toml
[dependancies]
rusty_gl = "0.1.0"
```

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
    let mut vao = 0;
    let mut vbo = 0;

    //Generate and bind the VAO
    gl_gen_vertex_arrays(1, &mut vao);
    gl_bind_vertex_array(vao);

    //Generate and bind the VBO
    gl_gen_buffers(1, &mut vbo);
    gl_bind_buffer(GLTarget::ArrayBuffer, vbo);

    //Buffer the vertex data and tell OpenGL the structure
    gl_buffer_data(GLTarget::ArrayBuffer, &VERTEX_DATA, GLUsage::StaticDraw);
    gl_enable_vertex_attrib_array(0);
    gl_vertex_attrib_pointer(0, 2, GLType::Float, false, 0);
```

Changes include:

* snake_case over PascalCase for function names
* No need for the `unsafe {...}` blocks
* Strongly typed enums over the error-prone GLenum (Where you can easily pass the incorrect enum)
* No need to cast types to `std::os::raw::c_void`, rusty-gl will do this for you
* No need to cast string to `std::ffi::CString`, rusty-gl will do this for you

## More examples

Examples on using this crate can be found in the examples folder: https://github.com/Hopson97/rusty-gl/tree/master/Examples

Run the examples with `cargo run`.

