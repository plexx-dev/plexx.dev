// https://rustwasm.github.io/docs/book/game-of-life/implementing.html

mod utils;

use wasm_bindgen::prelude::*;

extern crate js_sys;

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

impl Direction {
    fn rotate_right(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn rotate_left(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    snake_pos: Vec<usize>,
    direction: Direction,
}

/// Private methods.
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_row(&self, id: usize) -> u32 {
        id.saturating_div(self.width as usize) as u32
    }

    fn get_column(&self, id: usize) -> u32 {
        (id % self.width as usize) as u32
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

        Universe {
            width,
            height,
            cells,
            snake_pos,
            direction,
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

        let id = self.get_index(4, 7);
        assert_eq!(self.get_row(id) = 4);
        assert_eq!(self.get_column(id) = 7);

        //the last element of the array is the snake head
        //let next_pos = match self.direction {
        //    Direction::Up => 1// snake pos tatsächllich pos usw
        //};

        self.cells = next;
    }

    pub fn spawn_apple(&mut self) {
        let idx = self.get_index(1, 1);
        self.cells[idx] = Cell::Apple;
    }

    pub fn clear(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = Cell::Void;
            }
        }
    }

    pub fn rotate_left(&mut self) {
        self.direction.rotate_left();
    }

    pub fn rotate_right(&mut self) {
        self.direction.rotate_right();
    }
}
