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
        let pixels = self.pixels; // Copy the pixels
        self.screen.draw_2d(event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            // Draw each pixel
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    // Determine the color of the pixel
                    let mut color: [f32; 4] = [0.0; 4];
                    match pixels[WIDTH * i + j] {
                        ON => color = [0.0, 0.0, 0.0, 1.0], // White
                        OFF => color = [1.0; 4],            // Black
                        _ => color = [0.0, 1.0, 0.0, 1.0],  // Green (for error)
                    }
                    // Draw the pixel
                    rectangle(
                        color,
                        [
                            i as f64 * SCALE, // x pos
                            j as f64 * SCALE, // y pos
                            SCALE,            // Pixel width
                            SCALE,            // Pixel height
                                              // 1.0 * SCALE + (i as f64 * SCALE), // Pixel width
                                              // 1.0 * SCALE + (j as f64 * SCALE), // Pixel height
                        ],
                        context.transform,
                        graphics,
                    );
                }
            }
        });
    }
}
