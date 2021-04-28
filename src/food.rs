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
    /*
    pub fn spawn(&mut self, size: Size, snake: &Snake) {
        while snake.tail.contains(&self.position) {
            self.position = Position {
                x: thread_rng().gen_range(0..=(size.width / (BLOCK_SIZE * 2.0)) as i32),
                y: thread_rng().gen_range(0..=(size.height / (BLOCK_SIZE * 2.0)) as i32),
            };
        }
    }
    */
    // NOTE I am rewriting this to fit the college board requirements
    // I would have used the code above if it fit the requirements
    pub fn spawn(&mut self, size: Size, snake: &Snake) {
        loop {
            let mut thing: bool = true;
            for i in snake.tail.iter() {
                if i == &self.position {
                    thing = false;
                    break;
                } else {
                    thing = true;
                }
            }

            if thing {
                break;
            } else {
                self.position = Position {
                    x: thread_rng().gen_range(0..=(size.width / (BLOCK_SIZE * 2.0)) as i32),
                    y: thread_rng().gen_range(0..=(size.height / (BLOCK_SIZE * 2.0)) as i32),
                };
            }
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
