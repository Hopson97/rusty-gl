extern crate gl;
extern crate glutin;
extern crate image;
extern crate rgl;

mod shader_loader;

use shader_loader::load_shader;
use gl::types::*;

use glutin::ContextTrait;
use image::{GenericImage};

static VERTEX_POS: [GLfloat; 8] = [-0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5];


static INDICES: [GLuint; 6] = [0, 1, 2, 2, 3, 0];

static TEX_COORDS: [GLfloat; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];

fn main() {
    //Set up the window
    let mut events_loop = glutin::EventsLoop::new();
    let win_builder = glutin::WindowBuilder::new();
    let ctx_builder = glutin::ContextBuilder::new();

    let windowed_context = ctx_builder.build_windowed(win_builder, &events_loop).unwrap();
    // gl::GetShaderInfoLog();
    unsafe {
        windowed_context.make_current().unwrap();
    }

    //Load gl library
    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    //Create a vertex array object and a vertex buffer object

    //Create a vertex array object and a vertex buffer object
    let mut vao = rgl::gen_vertex_array();
    let mut vbo = rgl::gen_buffer();
    let mut tex_vbo = rgl::gen_buffer();

    rgl::bind_vertex_array(vao);

    //Vertex positions
    rgl::bind_buffer(rgl::Target::ArrayBuffer, vbo);

    //Buffer the vertex data and tell OpenGL the structure
    rgl::buffer_data(rgl::Target::ArrayBuffer, &VERTEX_POS, rgl::Usage::StaticDraw);
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_pointer(0, 2, rgl::Type::Float, false, 0);

    //TEXTURE
    //Generate and bind the VBO
    rgl::bind_buffer(rgl::Target::ArrayBuffer, tex_vbo);

    //Buffer the vertex data and tell OpenGL the structure
    rgl::buffer_data(rgl::Target::ArrayBuffer, &TEX_COORDS, rgl::Usage::StaticDraw);
    rgl::enable_vertex_attrib_array(1);
    rgl::vertex_attrib_pointer(1, 2, rgl::Type::Float, false, 0);

    //ebo
    let mut ebo = rgl::gen_buffer();;
    rgl::bind_buffer(rgl::Target::ElementArrayBuffer, ebo);
    rgl::buffer_data(rgl::Target::ElementArrayBuffer, &INDICES, rgl::Usage::StaticDraw);

    //Shaders!
    let shader_program = load_shader(
        String::from("examples/texture/data/shader.vert"),
        String::from("examples/texture/data/shader.frag"),
    );
    rgl::use_program(shader_program);

    let buffer = image::open("examples/texture/data/texture.png").unwrap();
    let dim = buffer.dimensions();
    let mut texture = rgl::gen_texture();
    rgl::active_texture(0);
    rgl::bind_texture(rgl::TexTarget::_2D, texture); 
    rgl::tex_image_2d(
        rgl::TexTarget::_2D,
        0,
        rgl::TexFormat::RGB,
        dim.0 as i32,
        dim.1 as i32,
        0,
        rgl::TexFormat::RGB,
        &buffer.raw_pixels(),
    );

    rgl::tex_parameteri(rgl::TexTarget::_2D, rgl::TexParamName::MinFilter, rgl::TexParam::Nearest);
    rgl::tex_parameteri(rgl::TexTarget::_2D, rgl::TexParamName::MagFilter, rgl::TexParam::Nearest);

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
        rgl::clear_color(0.5, 0.2, 0.8, 1.0);
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        rgl::draw_elements(rgl::Primitive::Triangles, 6, rgl::Type::UInt);

        windowed_context.swap_buffers().unwrap();
    }

    //Cleanup
    rgl::delete_buffers(1, &mut ebo);
    rgl::delete_buffers(1, &mut vbo);
    rgl::delete_buffers(1, &mut tex_vbo);
    rgl::delete_vertex_arrays(1, &mut vao);

    rgl::delete_textures(1, &mut texture);
    rgl::delete_program(shader_program);
}
