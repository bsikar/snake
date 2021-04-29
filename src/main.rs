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

    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            game.draw_instructions(args);
        }

        if let Some(Button::Keyboard(_)) = e.press_args() {
            break;
        }
    }

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
