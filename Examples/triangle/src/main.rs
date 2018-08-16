extern crate rusty_gl;
extern crate gl;
extern crate glutin;

use rusty_gl::buffers::*;
use rusty_gl::enums::*;

use gl::types::*;
use glutin::GlContext;

static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

fn main() {
    //Set up the window
    let mut events_loop = glutin::EventsLoop::new();
    let win_builder = glutin::WindowBuilder::new();
    let ctx_builder = glutin::ContextBuilder::new();

    let window = glutin::GlWindow::new(win_builder, ctx_builder, &events_loop).unwrap();
    
    unsafe {
        window.make_current().unwrap();
    }

    //Load gl library
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let mut vao = 0;
    let mut vbo = 0;

    gl_gen_vertex_arrays(1, &mut vao);
    gl_bind_vertex_array(vao);

    gl_gen_buffers(1, &mut vbo);
    gl_bind_buffer(GLTarget::ArrayBuffer, vbo);
    gl_buffer_data(GLTarget::ArrayBuffer, &VERTEX_DATA, GLUsage::StaticDraw);
    gl_enable_vertex_attrib_array(0);
    gl_vertex_attrib_pointer(0, 2, gl::FLOAT, false, 0);

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

        //Render
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers().unwrap();
    }
}
