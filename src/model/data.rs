use std::ops::Add;

use crate::constants::{EMPTY_COLOR, FOOD_COLOR, SNAKE_COLOR};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Food,
    Empty,
    Snake,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up = 1,
    Down = -1,
    Left = 2,
    Right = -2,
    None = 0,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Cell {
    pub fn color(&self) -> &str {
        match *self {
            Cell::Food => FOOD_COLOR,
            Cell::Empty => EMPTY_COLOR,
            Cell::Snake => SNAKE_COLOR,
        }
    }
}

impl Add for Direction {
    type Output = i8;

    fn add(self, other: Self) -> Self::Output {
        (self as i8) + (other as i8)
    }
}

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
