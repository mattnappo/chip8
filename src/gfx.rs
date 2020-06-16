use super::interpreter::chip8::{HEIGHT, OFF, ON, WIDTH};
use piston_window::*;

pub struct Display {
    pub pixels: [u8; WIDTH * HEIGHT], // The pixels themselves
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [OFF; WIDTH * HEIGHT],
        }
    }
}
