use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::Receiver;
use std::thread::Builder;
use glfw::{Context, Glfw, WindowEvent, WindowMode};
use log::{debug, error};
use window_builder::WindowBuilder;
use crate::application::window::resolution::Resolution;

pub mod resolution;
pub mod window_builder;


pub struct Window {
    glfw_window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    fn builder(glfw: &mut Glfw) -> WindowBuilder {
        WindowBuilder::new(glfw)
    }

    fn new(glfw: &mut glfw::Glfw, resolution: Resolution, title: String, window_mode: WindowMode) -> Self {
        match glfw.create_window(
            resolution.width,
            resolution.height,
            title.as_str(),
            window_mode,
        ) {
            Some((mut window, events)) => {
                window.set_framebuffer_size_polling(true);
                window.set_key_polling(true);
                debug!("New window \"{}\" created", title);
                Window {
                    glfw_window: window,
                    events,
                }
            }
            None => {
                error!("Can't create a window class");
                panic!()
            }
        }
    }
}
