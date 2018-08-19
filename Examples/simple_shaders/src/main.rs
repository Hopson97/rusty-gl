extern crate rgl;
extern crate gl;
extern crate glutin;

mod shader_loader;

use shader_loader::load_shader;

use gl::types::*;

use glutin::GlContext;

static VERTEX_POS: [GLfloat; 8] = [
    -0.5, -0.5, 
    -0.5, 0.5, 
    0.5, 0.5, 
    0.5, -0.5
];

static COLOURS: [GLfloat; 12] = [
    1.0, 0.0, 0.0, 
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
    1.0, 1.0, 1.0,
];

static INDICES: [GLuint; 6] = [
    0, 1, 2,
    2, 3, 0
];

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
    let mut vao = rgl::gen_vertex_array();
    let mut vbo = rgl::gen_buffer();
    let mut colour_vbo = rgl::gen_buffer();

    rgl::bind_vertex_array(vao);

    //Vertex positions
    rgl::bind_buffer(rgl::Target::ArrayBuffer, vbo);

    //Buffer the vertex data and tell OpenGL the structure
    rgl::buffer_data(rgl::Target::ArrayBuffer, &VERTEX_POS, rgl::Usage::StaticDraw);
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_pointer(0, 2, rgl::Type::Float, false, 0);

    //COLOURS
    rgl::bind_buffer(rgl::Target::ArrayBuffer, colour_vbo);

    //Buffer the vertex data and tell OpenGL the structure
    rgl::buffer_data(rgl::Target::ArrayBuffer, &COLOURS, rgl::Usage::StaticDraw);
    rgl::enable_vertex_attrib_array(1);
    rgl::vertex_attrib_pointer(1, 3, rgl::Type::Float, false, 0);

    //ebo
    let mut ebo = rgl::gen_buffer();
    rgl::bind_buffer(rgl::Target::ElementArrayBuffer, ebo);
    rgl::buffer_data(rgl::Target::ElementArrayBuffer, &INDICES, rgl::Usage::StaticDraw);

    //Shaders!
    let shader_program = load_shader(String::from("data/shader.vert"), String::from("data/shader.frag"));
    rgl::use_program(shader_program);

    //Main loop
    let mut is_running = true;
    while is_running {
        //Poll window events
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => is_running = false,
                    _ => (),
                },
                _ => ()
            }
        });

        //Draw stuff 
        rgl::clear_color(0.5, 0.5, 0.2, 1.0);
        unsafe {  gl::Clear(gl::COLOR_BUFFER_BIT); }
        rgl::draw_elements(rgl::Primitive::Triangles, 6, rgl::Type::UInt);

        window.swap_buffers().unwrap();
    }

    //Cleanupgt
    rgl::delete_buffers(1, &mut ebo);
    rgl::delete_buffers(1, &mut vbo);
    rgl::delete_buffers(1, &mut colour_vbo);
    rgl::delete_vertex_arrays(1, &mut vao);

    rgl::delete_program(shader_program);
}
