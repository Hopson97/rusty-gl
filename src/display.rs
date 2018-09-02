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

pub struct Display {
    event_loop: DisplayEventLoop,
    window: glutin::GlWindow,
    is_open: bool
}

pub struct DisplayEventLoop {
    event_loop: Rc<glutin::EventsLoop>,
}

#[derive(Clone)]
pub enum WinEvent {
    Close,
    NoEvent
}

impl Display {
    /// Creates a new RGL window
    pub fn new(title: &str, width: f64, height: f64) -> Display {
        //Create some Glutin util to help create the window
        let win_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));
        let ctx_builder = glutin::ContextBuilder::new();
        let event_loop = Rc::new(glutin::EventsLoop::new());

        //Create the display itself...
        let disp = Display {
            event_loop: DisplayEventLoop::new(Rc::clone(&event_loop)),
            window: glutin::GlWindow::new(win_builder, ctx_builder, &event_loop).unwrap(),
            is_open: true,
        };

        //Allow OpenGL to work with the window
        unsafe {
            disp.window.make_current().unwrap();
        }
        gl::load_with(|symbol| disp.window.get_proc_address(symbol) as *const _);

        disp
    }

    pub fn get_events(&self) -> &DisplayEventLoop {
        &self.event_loop
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Swap the window buffers, displaying what has been drawn to the window
    pub fn display(&self) {
        self.window.swap_buffers().unwrap();
    }
}

impl DisplayEventLoop {
    pub fn new(el: Rc<glutin::EventsLoop>) -> DisplayEventLoop {
        DisplayEventLoop {
            event_loop: el
        }
    }

    pub fn poll_events<F>(&mut self, callback: F) 
        where F: FnMut (WinEvent)
    {
        let event_loop = Rc::get_mut(&mut self.event_loop).unwrap();
        event_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => callback(WinEvent::Close),
                _ => (),
            },
            _ => (),
        });
    }
}