use crate::model::data::{Direction, Position};

pub struct Snake {
    direction: Direction,
    head: Position,
    tail: Vec<Position>,
}

impl Snake {
    pub fn new(head: Position) -> Snake {
        Snake {
            head,
            tail: vec![],
            direction: Direction::None,
        }
    }

    pub fn reset(&mut self) {
        self.tail = vec![];
    }

    pub fn head(&self) -> Position {
        self.head
    }

    pub fn current(&self, postion: Position) -> bool {
        self.head.x() == postion.x() && self.head.y() == postion.y()
    }

    pub fn tail(&self) -> &Vec<Position> {
        &self.tail
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_head(&mut self, position: Position) {
        self.head = position;
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = match self.direction + new_direction == 0 {
            true => self.direction,
            false => new_direction,
        };
    }

    pub fn grow(&mut self, position: Position) {
        self.tail.insert(0, self.head);
        self.set_head(position);
    }

    pub fn move_(&mut self, position: Position) {
        self.grow(position);
        if self.tail.len() > 3 {
            self.tail.pop().unwrap();
        }
    }

    pub fn last(&self) -> Option<Position> {
        self.tail.last().copied()
    }

    pub fn step(&self) -> Option<Position> {
        let head_x = self.head().x() as i32;
        let head_y = self.head().y() as i32;

        let (x, y) = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
            Direction::None => (head_x, head_y),
        };

        match x >= 0 && y >= 0 {
            true => Some(Position::new(x as u32, y as u32)),
            false => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reset() {
        let mut snake = Snake::new(Position::new(0, 0));
        snake.grow(Position::new(1, 0));
        snake.reset();
        assert_eq!(snake.tail().len(), 0);
    }

    #[test]
    fn test_current() {
        let snake = Snake::new(Position::new(1, 1));
        assert!(snake.current(Position::new(1, 1)));
    }

    #[test]
    fn test_grow() {
        let mut snake = Snake::new(Position::new(0, 0));
        snake.grow(Position::new(1, 0));
        snake.grow(Position::new(1, 1));
        assert_eq!(snake.tail.len(), 2);
    }

    #[test]
    fn test_move() {
        let mut snake = Snake::new(Position::new(0, 0));
        snake.grow(Position::new(1, 0));
        snake.move_(Position::new(1, 1));
        snake.move_(Position::new(2, 1));
        snake.grow(Position::new(2, 2));
        assert_eq!(snake.head.x(), 2);
        assert_eq!(snake.head.y(), 2);
    }

    #[test]
    fn test_set_direction() {
        let mut snake = Snake::new(Position::new(0, 0));
        snake.set_direction(Direction::Down);
        snake.set_direction(Direction::Up);
        assert_eq!(snake.direction(), Direction::Down);
    }

    #[test]
    fn test_step() {
        let mut snake = Snake::new(Position::new(1, 1));
        snake.set_direction(Direction::Up);

        let new = snake.step().unwrap();
        assert_eq!(new.x(), 1);
        assert_eq!(new.y(), 0);
    }
}
