use std::mem;

use crate::model::data::{Cell, Position};

pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let cells: Vec<Cell> = Board::empty(width, height);
        Board {
            width,
            height,
            cells,
        }
    }

    pub fn reset(&mut self) {
        self.cells = Board::empty(self.width, self.height);
    }

    pub fn empty(width: u32, height: u32) -> Vec<Cell> {
        (0..width * height).map(|_| Cell::Empty).collect()
    }

    pub fn empty_indexes(&mut self) -> Vec<usize> {
        self.cells()
            .iter()
            .enumerate()
            .filter(|(_, &cell)| cell == Cell::Empty)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>()
    }

    pub fn to_index(&self, position: Position) -> u32 {
        position.x() * self.width + position.y()
    }

    pub fn to_position(&self, index: u32) -> Position {
        let x = index / self.width();
        let y = index % self.width();
        Position::new(x, y)
    }

    pub fn set(&mut self, index: usize, cell: Cell) -> Cell {
        mem::replace(&mut self.cells[index], cell)
    }

    pub fn get(&self, index: usize) -> Option<&Cell> {
        self.cells.get(index)
    }

    pub fn cells(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reset() {
        let mut board = Board::new(2, 2);
        board.set(0, Cell::Snake);
        board.set(1, Cell::Food);
        board.reset();
        assert_eq!(board.empty_indexes().len(), 4);
    }

    #[test]
    fn test_to_index() {
        let board = Board::new(2, 2);
        let position = Position::new(1, 0);
        assert_eq!(board.to_index(position), 2);
    }

    #[test]
    fn test_to_position() {
        let board = Board::new(2, 2);
        let position = board.to_position(2);
        assert_eq!(position.x(), 1);
        assert_eq!(position.y(), 0);
    }
}
