use crate::food::Food;
use crate::snake::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::*;

#[non_exhaustive]
pub struct Color;

impl Color {
    pub const BACKGROUND: [f32; 4] = [0.3, 0.4, 0.2, 1.0];
    pub const SNAKE_BODY: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const SNAKE_HEAD: [f32; 4] = [0.3, 0.6, 1.2, 1.0];
    pub const FOOD: [f32; 4] = [1.0, 0.6, 0.2, 1.0];
    pub const TEXT: [f32; 4] = [1.0, 0.99, 0.22, 1.0];
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub snake: Snake,
    pub food: Food,
    pub window_size: Size,
}

impl Game {
    // make a new game
    pub fn new(snake: Snake, food: Food, size: Size) -> Game {
        Game {
            snake,
            food,
            window_size: size,
        }
    }
    // call a function to draw the instructions screen
    pub fn draw_instructions(&self, args: RenderArgs) {
        let mut gl = GlGraphics::new(OpenGL::V3_2);
        let font = include_bytes!("../assets/FiraSans-Regular.ttf");
        let mut glyphs = GlyphCache::from_bytes(font, (), TextureSettings::new()).unwrap();

        gl.draw(args.viewport(), |c, g| {
            clear(Color::BACKGROUND, g);
            text(
                Color::TEXT,
                (self.window_size.width / 25.0) as u32,
                "WASD or Arrow Keys: Move",
                &mut glyphs,
                c.transform
                    .trans(self.window_size.width / 4.0, self.window_size.height / 2.0),
                g,
            )
        })
        .expect("Failed to make end screen");

        gl.draw(args.viewport(), |c, g| {
            text(
                Color::TEXT,
                (self.window_size.width / 25.0) as u32,
                "Q: Pause",
                &mut glyphs,
                c.transform
                    .trans(self.window_size.width / 4.0, self.window_size.height / 2.35),
                g,
            )
        })
        .expect("Failed to make end screen");

        gl.draw(args.viewport(), |c, g| {
            text(
                Color::TEXT,
                (self.window_size.width / 25.0) as u32,
                "Esc: Quit",
                &mut glyphs,
                c.transform
                    .trans(self.window_size.width / 4.0, self.window_size.height / 1.75),
                g,
            )
        })
        .expect("Failed to make end screen");
    }

    // update the game by calling functions to move the snake
    // have the snake eat food and spawn new food
    pub fn update(&mut self, size: Size, args: &UpdateArgs, key: Key) {
        self.snake.update(size, args.dt, self.key_direction(key));
        self.window_size = size;
        if self.snake.position == self.food.position {
            self.snake.eat();
            self.food.spawn(size, &self.snake);
        }
    }

    // change the direction the snake is moving based on the players
    // keyboard input
    fn key_direction(&self, key: Key) -> Direction {
        return {
            match key {
                Key::Right | Key::D => Direction::Right,
                Key::Left | Key::A => Direction::Left,
                Key::Down | Key::S => Direction::Down,
                Key::Up | Key::W => Direction::Up,
                Key::Q => Direction::Still,
                _ => self.snake.direction,
            }
        };
    }

    // call functions to draw the snake and the food
    pub fn draw(&mut self, c: &Context, g: &mut G2d) {
        self.snake.draw(c, g);
        self.food.draw(c, g);
    }

    // return is the game is over or not (if the snake is dead)
    pub fn over(&self) -> bool {
        !self.snake.is_alive()
    }

    // draw the game over screen and show the final length of the snake
    pub fn draw_game_over(&self, args: RenderArgs) {
        let mut gl = GlGraphics::new(OpenGL::V3_2);
        let font = include_bytes!("../assets/FiraSans-Regular.ttf");
        let mut glyphs = GlyphCache::from_bytes(font, (), TextureSettings::new()).unwrap();

        gl.draw(args.viewport(), |c, g| {
            clear(Color::BACKGROUND, g);
            text(
                Color::TEXT,
                (self.window_size.width / 13.3) as u32,
                format!("Final Length: {}", self.snake.length).as_str(),
                &mut glyphs,
                c.transform
                    .trans(self.window_size.width / 4.0, self.window_size.height / 2.0),
                g,
            )
        })
        .expect("Failed to make end screen");
    }
}
