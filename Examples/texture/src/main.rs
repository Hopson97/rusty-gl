extern crate gl;
extern crate glutin;
extern crate image;
extern crate rusty_gl;

mod shader_loader;

use shader_loader::load_shader;

use rusty_gl::buffers::*;
use rusty_gl::drawing::*;
use rusty_gl::enums::*;
use rusty_gl::shaders::*;
use rusty_gl::textures::*;

use gl::types::*;

use glutin::GlContext;
use image::{GenericImage};

static VERTEX_POS: [GLfloat; 8] = [-0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5];


static INDICES: [GLuint; 6] = [0, 1, 2, 2, 3, 0];

static TEX_COORDS: [GLfloat; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];

fn main() {
    //Set up the window
    let mut events_loop = glutin::EventsLoop::new();
    let win_builder = glutin::WindowBuilder::new();
    let ctx_builder = glutin::ContextBuilder::new();

    let window = glutin::GlWindow::new(win_builder, ctx_builder, &events_loop).unwrap();
    // gl::GetShaderInfoLog();
    unsafe {
        window.make_current().unwrap();
    }

    //Load gl library
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    //Create a vertex array object and a vertex buffer object

    //Create a vertex array object and a vertex buffer object
    let mut vao = gl_gen_vertex_array();
    let mut vbo = gl_gen_buffer();
    let mut tex_vbo = gl_gen_buffer();

    gl_bind_vertex_array(vao);

    //Vertex positions
    gl_bind_buffer(GLTarget::ArrayBuffer, vbo);

    //Buffer the vertex data and tell OpenGL the structure
    gl_buffer_data(GLTarget::ArrayBuffer, &VERTEX_POS, GLUsage::StaticDraw);
    gl_enable_vertex_attrib_array(0);
    gl_vertex_attrib_pointer(0, 2, GLType::Float, false, 0);

    //TEXTURE
    //Generate and bind the VBO
    gl_bind_buffer(GLTarget::ArrayBuffer, tex_vbo);

    //Buffer the vertex data and tell OpenGL the structure
    gl_buffer_data(GLTarget::ArrayBuffer, &TEX_COORDS, GLUsage::StaticDraw);
    gl_enable_vertex_attrib_array(1);
    gl_vertex_attrib_pointer(1, 2, GLType::Float, false, 0);

    //ebo
    let mut ebo = gl_gen_buffer();;
    gl_bind_buffer(GLTarget::ElementArrayBuffer, ebo);
    gl_buffer_data(GLTarget::ElementArrayBuffer, &INDICES, GLUsage::StaticDraw);

    //Shaders!
    let shader_program = load_shader(
        String::from("data/shader.vert"),
        String::from("data/shader.frag"),
    );
    gl_use_program(shader_program);

    let buffer = image::open("data/texture.png").unwrap();
    let dim = buffer.dimensions();
    let mut texture = gl_gen_texture();
    gl_active_texture(0);
    gl_bind_texture(GLTexTarget::_2D, texture); 
    gl_tex_image_2d(
        GLTexTarget::_2D,
        0,
        GLTexFormat::RGB,
        dim.0 as i32,
        dim.1 as i32,
        0,
        GLTexFormat::RGB,
        &buffer.raw_pixels(),
    );

    gl_tex_parameteri(GLTexTarget::_2D, GLTexParamName::MinFilter, GLTexParam::Nearest);
    gl_tex_parameteri(GLTexTarget::_2D, GLTexParamName::MagFilter, GLTexParam::Nearest);

    //Main loop
    let mut is_running = true;
    while is_running {
        //Poll window events
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => is_running = false,
                _ => (),
            },
            _ => (),
        });

        //Draw stuff
        gl_clear_color(0.5, 0.2, 0.8, 1.0);
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        gl_draw_elements(GLPrimitive::Triangles, 6, GLType::UInt);

        window.swap_buffers().unwrap();
    }

    //Cleanup
    gl_delete_buffers(1, &mut ebo);
    gl_delete_buffers(1, &mut vbo);
    gl_delete_buffers(1, &mut tex_vbo);
    gl_delete_vertex_arrays(1, &mut vao);

    gl_delete_textures(1, &mut texture);
    gl_delete_program(shader_program);
}
