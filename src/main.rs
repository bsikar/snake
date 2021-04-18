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
 * language shut this down listing the reasons for not having a large libary.
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

mod draw;
mod food;
mod game;
mod snake;

use draw::BLOCK_SIZE;
use food::Food;
use game::{Color, Game};
use piston_window::*;
use snake::Snake;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut game = Game::new(
        Snake::new(
            (window.size().width / (BLOCK_SIZE * 2.0)) as i32,
            (window.size().height / (BLOCK_SIZE * 2.0)) as i32,
        ),
        Food::new(window.size()),
        window.size(),
    );

    let mut x = Key::Q;

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(k)) = e.press_args() {
            if k != x {
                x = k;
            }
        }

        window.draw_2d(&e, |c, g, _| {
            clear(Color::BACKGROUND, g);
            game.draw(&c, g);
        });

        e.update(|args| {
            game.update(window.size(), args, x);
        });

        if game.over() {
            if let Some(args) = e.render_args() {
                game.draw_game_over(args);
            }
        }
    }
}
