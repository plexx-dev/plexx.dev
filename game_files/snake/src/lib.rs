// https://rustwasm.github.io/docs/book/game-of-life/implementing.html

mod utils;

use wasm_bindgen::prelude::*;

//extern crate js_sys;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Void = 0,
    Snake = 1,
    Apple = 2,
}

impl Cell {}

pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    snake_pos: Vec<usize>,
    direction: Direction,
    score: u32,
    speed: u32,
    rng: SmallRng
}

/// Private methods.
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_row_column(&self, id: usize) -> (u32, u32) {
        let row = id as u32 / self.width();
        let col = id as u32 % self.width();

        (row, col)
    }

    fn spawn_apple(&mut self) {
        let mut next_apple = self.rng.gen_range(0..(self.width() * self.width())) as usize;

        while self.snake_pos.contains(&next_apple) {
            next_apple = self.rng.gen_range(0..(self.width() * self.width())) as usize;
        }

        self.cells[next_apple] = Cell::Apple;
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 16;
        let height = 16;

        let cells = (0..width * height).map(|_i| Cell::Void).collect();

        let snake_pos = vec![];

        let direction = Direction::Right;

        let score = 0;
        let speed = 3;
        let rng = SmallRng::from_entropy();

        Universe {
            width,
            height,
            cells,
            snake_pos,
            direction,
            score,
            speed,
            rng,
        }
    }

    pub fn spawn_snake(&mut self) {
        let idx = self.get_index((self.width() - 1) / 2, (self.height() - 1) / 2);
        self.cells[idx - 1] = Cell::Snake;
        self.snake_pos.push(idx - 1);

        self.cells[idx] = Cell::Snake;
        self.snake_pos.push(idx);

        self.cells[idx + 1] = Cell::Snake;
        self.snake_pos.push(idx + 1);

        self.spawn_apple();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        //the last element of the array is the snake head
        let temp_nake_pos = self.snake_pos.clone();
        let snake_head = temp_nake_pos.last().unwrap();
        let snake_tail = temp_nake_pos.first().unwrap();

        let (mut next_row, mut next_column) = self.get_row_column(*snake_head);

        match self.direction {
            Direction::Up => {
                next_row = (next_row - 1) % (self.height());
            }
            Direction::Right => {
                next_column = (next_column + 1) % (self.width());
            }
            Direction::Down => {
                next_row = (next_row + 1) % (self.height());
            }
            Direction::Left => {
                next_column = (next_column - 1) % (self.width());
            }
        };

        let next_snake_head = self.get_index(next_row, next_column);

        match next[next_snake_head] {
            Cell::Void => {
                next[next_snake_head] = Cell::Snake;
                self.snake_pos.push(next_snake_head);
                self.snake_pos.remove(0);
            }
            Cell::Snake => {
                //GAME OVER
                self.clear();
            }
            Cell::Apple => {
                self.score += 1;
                self.speed += 1;

                next[next_snake_head] = Cell::Snake;
                self.spawn_apple();
                self.snake_pos.push(next_snake_head);

            }
        }

        next[*snake_tail] = Cell::Void;

        self.cells = next;
    }

    pub fn clear(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = Cell::Void;
            }
        }

        self.score = 0;
        self.speed = 5;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn change_direction(&mut self, direction: u8) {
        let current_direction = &self.direction;
        match current_direction {
            Direction::Up => match direction {
                0 => {}
                1 => {
                    self.direction = Direction::Right;
                }
                2 => {}
                3 => {
                    self.direction = Direction::Left;
                }
                _ => {}
            },
            Direction::Right => match direction {
                0 => {
                    self.direction = Direction::Up;
                }
                1 => {}
                2 => {
                    self.direction = Direction::Down;
                }
                3 => {}
                _ => {}
            },
            Direction::Down => match direction {
                0 => {}
                1 => {
                    self.direction = Direction::Right;
                }
                2 => {}
                3 => {
                    self.direction = Direction::Left;
                }
                _ => {}
            },
            Direction::Left => match direction {
                0 => {
                    self.direction = Direction::Up;
                }
                1 => {}
                2 => {
                    self.direction = Direction::Down;
                }
                3 => {}
                _ => {}
            },
        }
    }
}
