//! A window to draw OpenGL onto.
//! This is optional for use, any window library can be used
//! This one is just for convenience purposes, as it creates a window
//! and sets it as the OpenGL context in a single line
//! # Example
//! ```
//! let display = rgl::Display::new("My window", 1280.0, 720.0);
//! ```

use glutin;
use glutin::GlContext;
use std::rc::Rc;

use gl;

use drawing;

pub struct Display {
    event_loop: Rc<glutin::EventsLoop>,
    window: glutin::GlWindow,
    is_open: bool
}

impl Display {
    pub fn new(title: &str, width: f64, height: f64) -> Display {
        let win_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));
        let ctx_builder = glutin::ContextBuilder::new();
        let event_loop = Rc::new(glutin::EventsLoop::new());

        let disp = Display {
            event_loop: event_loop.clone(),
            window: glutin::GlWindow::new(win_builder, ctx_builder, &event_loop).unwrap(),
            is_open: true,
        };

        unsafe {
            disp.window.make_current().unwrap();
        }
        
        gl::load_with(|symbol| disp.window.get_proc_address(symbol) as *const _);

        disp
    }

    pub fn display(&self) {
        self.window.swap_buffers().unwrap();
    }
}