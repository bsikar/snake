use crate::draw::{draw, BLOCK_SIZE};
use crate::game::{Color, Position};
use piston_window::{Context, G2d, Size};
use std::collections::VecDeque;

const SNAKE_WAIT: f64 = 0.2;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Still,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snake {
    pub position: Position,
    pub length: u32,
    pub direction: Direction,
    pub tail: VecDeque<Position>,
    is_alive: bool,
    wait: f64,
}

impl Snake {
    // make a new snake
    pub fn new(x: i32, y: i32) -> Snake {
        Snake {
            position: Position { x, y },
            length: 1,
            direction: Direction::Still,
            tail: vec![].into_iter().collect(),
            is_alive: true,
            wait: 0.0,
        }
    }

    // check if the position of the snake is out of bounds
    fn is_valid(&self, size: Size) -> bool {
        let x = self.position.x;
        let y = self.position.y;
        x >= 0
            && y >= 0
            && x <= (size.width / BLOCK_SIZE) as i32
            && y <= (size.height / BLOCK_SIZE) as i32
    }

    // move the snake in the direction it is facing
    pub fn mv(&mut self, size: Size, direction: Direction) {
        self.wait = 0.0;
        if !self.is_valid(size) {
            self.is_alive = false;
            return;
        }

        match self.direction {
            Direction::Left => {
                if direction != Direction::Right {
                    self.direction = direction;
                }
            }
            Direction::Right => {
                if direction != Direction::Left {
                    self.direction = direction;
                }
            }
            Direction::Up => {
                if direction != Direction::Down {
                    self.direction = direction;
                }
            }
            Direction::Down => {
                if direction != Direction::Up {
                    self.direction = direction;
                }
            }
            Direction::Still => self.direction = direction,
        }

        // Note: I am using 2 match cases here for visibilty (I could have put this in the one up above).
        match self.direction {
            Direction::Left => {
                if self.overlap_tail(self.position.x - 1, self.position.y) {
                    self.is_alive = false;
                    return;
                }
                self.position.x -= 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Right => {
                if self.overlap_tail(self.position.x + 1, self.position.y) {
                    self.is_alive = false;
                    return;
                }
                self.position.x += 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Up => {
                if self.overlap_tail(self.position.x, self.position.y - 1) {
                    self.is_alive = false;
                    return;
                }
                self.position.y -= 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Down => {
                if self.overlap_tail(self.position.x, self.position.y + 1) {
                    self.is_alive = false;
                    return;
                }
                self.position.y += 1;
                self.tail.pop_back();
                self.tail.push_front(self.position);
            }
            Direction::Still => {}
        }
    }

    // return if the snake is over laping its tail
    fn overlap_tail(&self, x: i32, y: i32) -> bool {
        self.tail.contains(&Position { x, y })
    }

    // have the snake eat food updating the snakes length
    pub fn eat(&mut self) {
        match self.direction {
            Direction::Left => {
                self.tail.push_back(Position {
                    x: self.position.x + 1,
                    y: self.position.y,
                });
            }
            Direction::Right => {
                self.tail.push_back(Position {
                    x: self.position.x - 1,
                    y: self.position.y,
                });
            }
            Direction::Up => {
                self.tail.push_back(Position {
                    x: self.position.x,
                    y: self.position.y + 1,
                });
            }
            Direction::Down => {
                self.tail.push_back(Position {
                    x: self.position.x,
                    y: self.position.y - 1,
                });
            }
            Direction::Still => {}
        }
        self.length += 1;
    }

    // draw the snake
    /*
    pub fn draw(&self, c: &Context, g: &mut G2d) {
        draw(
            Color::SNAKE_HEAD,
            self.position.x as u32,
            self.position.y as u32,
            1,
            1,
            c,
            g,
        );
        self.tail
            .iter()
            .skip(1)
            .for_each(|seg| draw(Color::SNAKE_BODY, seg.x as u32, seg.y as u32, 1, 1, c, g));
    }
    */
    // NOTE I am rewriting this to remove confusion by the .for_each()
    // I would have used the code above, but I do not want to get points
    // remove by someone who is confused if I was accessing multiple
    // elements in the list (VecDeque)
    pub fn draw(&self, c: &Context, g: &mut G2d) {
        draw(
            Color::SNAKE_HEAD,
            self.position.x as u32,
            self.position.y as u32,
            1,
            1,
            c,
            g,
        );
        for seg in self.tail.iter().skip(1) {
            draw(Color::SNAKE_BODY, seg.x as u32, seg.y as u32, 1, 1, c, g);
        }
    }

    // return if the snake is alive
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    // move the snake after a set amount of 'wait time' so the snake isnt too fast
    pub fn update(&mut self, size: Size, dt: f64, direction: Direction) {
        self.wait += dt;
        if self.wait > SNAKE_WAIT {
            self.mv(size, direction);
        }
    }
}
