pub mod window;

use crate::application::window::window_builder::WindowBuilder;
use glfw;
use log::{debug, error, info};
use crate::application::window::Window;
use crate::constants::NAME_CAPITALIZED;

pub struct Application {
    glfw: Box<glfw::Glfw>,
    window: Option<window::Window>,
}

fn error_callback(_: glfw::Error, description: String, _: &()) {
    error!("GLFW Error: {}", description);
    panic!();
}

static ERROR_CALLBACK: Option<glfw::ErrorCallback<()>> = Some(glfw::Callback {
    f: error_callback as fn(glfw::Error, String, &()),
    data: (),
});

impl Application {
    pub fn new() -> Self {
        if let Ok(glfw) = glfw::init(ERROR_CALLBACK) {
            info!("GLFW Initialized succesefully, Welcome to {} ", NAME_CAPITALIZED);
            Self {
                glfw: Box::new(glfw),
                window: None,
            }
        } else {
            error!("Failed to initialize glfw");
            panic!()
        }

    }

    pub fn add_window(&mut self) {
        self.window = Some(WindowBuilder::new(&mut self.glfw).build());
    }

    fn run(&self) {
        todo!()
    }
}
