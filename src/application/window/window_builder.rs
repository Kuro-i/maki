use glfw::{Glfw, WindowMode};
use crate::application::window::resolution::Resolution;
use super::Window;
use crate::constants;
use crate::constants::NAME_CAPITALIZED;

#[derive(Debug)]
pub struct WindowBuilder<'a> {
    glfw : &'a mut glfw::Glfw,
    resolution: Resolution,
    title: String,
    window_mode: WindowMode<'a>,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(glfw: &'a mut Glfw) -> WindowBuilder<'a> {
        WindowBuilder {
            glfw,
            resolution: Resolution::default(),
            title: NAME_CAPITALIZED.to_string(),
            window_mode: WindowMode::Windowed,
        }
    }

    pub fn resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn window_mode(mut self, window_mode: WindowMode<'a>) -> Self {
        self.window_mode = window_mode;
        self
    }

    pub fn build(&mut self) -> Window {
        Window::new(
            &mut self.glfw,
            self.resolution.clone(),
            self.title.clone(),
            self.window_mode.clone()
        )
    }

}