use std::fmt::{Debug, Display, Formatter};

/// A structure that contain window size
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            width: 640,
            height: 480,
        }
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl From<(u32, u32)> for Resolution {
    fn from((width, height): (u32, u32)) -> Self {
        Resolution {
            width,
            height,
        }
    }
}

impl Into<(u32, u32)> for Resolution {
    fn into(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Resolution {
    /// Creates a Resolution object
    pub fn new(width: u32, height: u32) -> Self {
        Resolution {
            width,
            height,
        }
    }
}