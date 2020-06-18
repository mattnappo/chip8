use super::interpreter::chip8::{HEIGHT, OFF, ON, WIDTH};
use piston_window::*;

const SCALE: f64 = 20.0;

pub struct Display {
    pub pixels: [u8; WIDTH * HEIGHT], // The pixels themselves
    pub screen: PistonWindow,
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [OFF; WIDTH * HEIGHT],
            screen: WindowSettings::new(
                "CHIP-8 Interpreter by @xoreo",
                [WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE],
            )
            .exit_on_esc(true)
            .build()
            .unwrap(),
        }
    }

    // draw draws the array of pixels.
    pub fn draw(&mut self, event: &Event) {
        self.screen.draw_2d(event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // Red (RGBA)
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
