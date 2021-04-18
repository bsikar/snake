/* MIT License
 *
 * Copyright (c) 2021 Brighton Sikarskie
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

/* The code that I am using that is not a part of the standard library are:
 *
 * rand:
 * https://crates.io/crates/rand
 *
 * piston_window:
 * https://crates.io/crates/piston_window
 *
 * piston2d-opengl_graphics
 * https://crates.io/crates/piston2d-opengl_graphics
 *
 *
 * **** NOTE ****
 * Rust has a really small standard library so it is common to 'import' others code
 * for more information about this read this:
 * https://users.rust-lang.org/t/rust-should-have-a-big-standard-library-and-heres-why/37449
 * it talks about making rust have a larger standard library and the creaters of the
 * language shut this down listing the reasons for not having a large library.
 *
 * Also refer to this to learn some more about cargo (the package manager for rust)
 * https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html
 *
 * Cargo is a convention and is standard even though I am taking code from a third party source
 * it is standard.
 *
 * The only asset that I used in this code is a font:
 *
 * FiraSans-Regular.ttf:
 * https://www.ffonts.net/Fira-Sans-Regular.font
 */

use crate::draw::{draw, BLOCK_SIZE};
use crate::game::{Color, Position};
use crate::snake::Snake;
use piston_window::{Context, G2d, Size};
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq)]
pub struct Food {
    pub position: Position,
}

impl Food {
    // make a new food
    pub fn new(size: Size) -> Food {
        Food {
            position: Position {
                x: thread_rng().gen_range(0..=(size.width / (BLOCK_SIZE * 2.0)) as i32),
                y: thread_rng().gen_range(0..=(size.height / (BLOCK_SIZE * 2.0)) as i32),
            },
        }
    }

    // spawn the food on the screen in a valid location
    pub fn spawn(&mut self, size: Size, snake: &Snake) {
        while snake.tail.contains(&self.position) {
            self.position = Position {
                x: thread_rng().gen_range(0..=(size.width / (BLOCK_SIZE * 2.0)) as i32),
                y: thread_rng().gen_range(0..=(size.height / (BLOCK_SIZE * 2.0)) as i32),
            };
        }
    }

    // draw the food on screen
    pub fn draw(&self, c: &Context, g: &mut G2d) {
        draw(
            Color::FOOD,
            self.position.x as u32,
            self.position.y as u32,
            1,
            1,
            c,
            g,
        );
    }
}
