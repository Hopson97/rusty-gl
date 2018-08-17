extern crate rusty_gl;
extern crate gl;
extern crate glutin;

mod shader_loader;

use shader_loader::load_shader;

use rusty_gl::buffers::*;
use rusty_gl::enums::*;
use rusty_gl::drawing::*;
use rusty_gl::shaders::*;

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
    let mut vao = 0;
    let mut vbo = 0;
    let mut colour_vbo = 0;

    //Generate and bind the VAO
    gl_gen_vertex_arrays(1, &mut vao);
    gl_bind_vertex_array(vao);

    //VERTEX POSITIONS
    //Generate and bind the VBO
    gl_gen_buffers(1, &mut vbo);
    gl_bind_buffer(GLTarget::ArrayBuffer, vbo);

    //Buffer the vertex data and tell OpenGL the structure
    gl_buffer_data(GLTarget::ArrayBuffer, &VERTEX_POS, GLUsage::StaticDraw);
    gl_enable_vertex_attrib_array(0);
    gl_vertex_attrib_pointer(0, 2, GLType::Float, false, 0);

    //COLOURS
    //Generate and bind the VBO
    gl_gen_buffers(1, &mut colour_vbo);
    gl_bind_buffer(GLTarget::ArrayBuffer, colour_vbo);

    //Buffer the vertex data and tell OpenGL the structure
    gl_buffer_data(GLTarget::ArrayBuffer, &COLOURS, GLUsage::StaticDraw);
    gl_enable_vertex_attrib_array(1);
    gl_vertex_attrib_pointer(1, 3, GLType::Float, false, 0);

    //ebo
    let mut ebo = 0;
    gl_gen_buffers(1, &mut ebo);
    gl_bind_buffer(GLTarget::ElementArrayBuffer, ebo);
    gl_buffer_data(GLTarget::ElementArrayBuffer, &INDICES, GLUsage::StaticDraw);

    //Shaders!
    let shader_program = load_shader(String::from("data/shader.vert"), String::from("data/shader.frag"));
    gl_use_program(shader_program);

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
        gl_clear_color(0.5, 0.5, 0.2, 1.0);
        unsafe {  gl::Clear(gl::COLOR_BUFFER_BIT); }
        gl_draw_elements(GLPrimitive::Triangles, 6, GLType::UInt);

        window.swap_buffers().unwrap();
    }
}
