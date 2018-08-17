extern crate rusty_gl;
extern crate gl;
extern crate glutin;

use rusty_gl::buffers::*;
use rusty_gl::enums::*;
use rusty_gl::drawing::*;

use gl::types::*;

use glutin::GlContext;



static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

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
        gl_draw_arrays(GLPrimitive::Triangles, 0, 3);


        window.swap_buffers().unwrap();
    }

    //Cleanup
    gl_delete_buffers(1, &mut vbo);
    gl_delete_vertex_arrays(1, &mut vao);
}
