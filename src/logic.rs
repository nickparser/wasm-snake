use rand::seq::SliceRandom;
use std::{cell::RefCell, rc::Rc};

use crate::constants::INITIAL_TAIL_LEN;
use crate::model::{
    board::Board,
    data::{Cell, Direction, Position},
    snake::Snake,
};

pub struct Logic {
    board: Board,
    food: Position,
    snake: Rc<RefCell<Snake>>,
}

impl Logic {
    pub fn new(width: u32, height: u32) -> Logic {
        let mut board = Board::new(width, height);
        let food = Logic::position(&mut board, Cell::Food);
        let snake = Snake::new(Logic::position(&mut board, Cell::Snake));
        let snake = Rc::new(RefCell::new(snake));
        Logic { snake, board, food }
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.snake.borrow_mut().reset();
        self.snake
            .borrow_mut()
            .set_head(Logic::position(&mut self.board, Cell::Snake));
        self.food = Logic::position(&mut self.board, Cell::Food);
    }

    pub fn position(board: &mut Board, cell: Cell) -> Position {
        let empty_indexes = board.empty_indexes();
        let index = empty_indexes.choose(&mut rand::thread_rng()).unwrap();
        board.set(*index, cell);
        board.to_position(*index as u32)
    }

    pub fn step(&mut self) -> bool {
        let position = match self.snake.borrow().step() {
            Some(position) => position,
            None => return false,
        };

        if !self.proceed(position) && !self.snake.borrow().current(position) {
            return false;
        }
        let index = self.board.to_index(position);
        let cell = self.board.cells()[index as usize];

        match cell {
            Cell::Food => {
                self.snake.borrow_mut().grow(position);
                self.food = Logic::position(&mut self.board, Cell::Food);
            }
            _ => self.snake.borrow_mut().move_(position),
        }

        self.board
            .set(self.board.to_index(position) as usize, Cell::Snake);
        true
    }

    fn proceed(&self, position: Position) -> bool {
        if position.x() >= self.board.width() || position.y() >= self.board.height() {
            return false;
        }
        let index = self.board.to_index(position);
        let cell = match self.board.get(index as usize) {
            Some(cell) => cell,
            None => return false,
        };

        *cell != Cell::Snake
    }

    pub fn snake(&self) -> Vec<Position> {
        let mut body: Vec<Position> = self.snake.borrow().tail()[..].to_vec();
        body.insert(0, self.snake.borrow().head());
        body
    }

    pub fn food(&self) -> Position {
        self.food
    }

    pub fn snake_ref(&self) -> &Rc<RefCell<Snake>> {
        &self.snake
    }

    pub fn score(&self) -> usize {
        self.snake().len() - INITIAL_TAIL_LEN - 1
    }

    pub fn passed(&mut self) -> Option<Position> {
        if self.snake.borrow().direction() == Direction::None {
            return None;
        }
        let position = match self.snake.borrow().last() {
            Some(position) => Some(position),
            None => Some(self.snake.borrow().head()),
        };

        let passed_index = self.board.to_index(position.unwrap());
        self.board.set(passed_index as usize, Cell::Empty);
        position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let mut logic = Logic::new(2, 2);
        let empty_indexes = logic.board.empty_indexes();

        assert_eq!(empty_indexes.len(), 2);
    }

    #[test]
    fn test_reset() {
        let mut logic = Logic::new(2, 2);
        logic.board.set(0, Cell::Snake);
        logic.board.set(1, Cell::Snake);
        logic.board.set(2, Cell::Snake);
        logic.board.set(3, Cell::Food);
        logic.reset();
        let empty_indexes = logic.board.empty_indexes();

        assert_eq!(empty_indexes.len(), 2);
    }

    #[test]
    fn test_position() {
        let mut logic = Logic::new(2, 2);
        let position = Logic::position(&mut logic.board, Cell::Snake);
        assert!(position.x() < logic.board.width());
        assert!(position.y() < logic.board.height());
    }

    #[test]
    fn test_alive() {
        let mut logic = Logic::new(2, 2);
        logic.board.reset();
        logic.snake.borrow_mut().set_head(Position::new(0, 0));
        logic.snake.borrow_mut().set_direction(Direction::Down);
        let alive = logic.step();

        let position = logic.snake.borrow().head();
        assert!(alive);
        assert_eq!((position.x(), position.y()), (0, 1));
    }

    #[test]
    fn test_no_alive() {
        let mut logic = Logic::new(2, 2);
        logic.board.reset();
        logic.snake.borrow_mut().set_head(Position::new(2, 2));
        logic.snake.borrow_mut().set_direction(Direction::Up);
        let alive = logic.step();

        assert!(!alive);
    }

    #[test]
    fn test_passed() {
        let mut logic = Logic::new(2, 2);
        logic.snake.borrow_mut().set_head(Position::new(1, 1));
        logic.snake.borrow_mut().set_direction(Direction::Up);

        let new = logic.passed().unwrap();
        assert_eq!(new.x(), 1);
        assert_eq!(new.y(), 1);
    }
}
