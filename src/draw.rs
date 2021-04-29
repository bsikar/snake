use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: f64 = 25.0;

// return input pixel to the size of a block
// (the head or 1 body segment of the snake)
pub fn to_block_size(size: u32) -> f64 {
    f64::from(size) * BLOCK_SIZE
}

// draw a rectangle on the screen with the parameters inputted
pub fn draw(color: Color, x: u32, y: u32, width: u32, height: u32, c: &Context, g: &mut G2d) {
    rectangle(
        color,
        [
            to_block_size(x),
            to_block_size(y),
            to_block_size(width),
            to_block_size(height),
        ],
        c.transform,
        g,
    );
}
