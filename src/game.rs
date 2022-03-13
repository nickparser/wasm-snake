use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::closure::Closure;

use crate::constants;
use crate::gui::{animation::Animation, paint::Paint, web::Web};
use crate::logic::Logic;
use crate::model::{
    data::{Cell, Direction},
    snake::Snake,
};
use crate::utils;

pub struct Game {
    logic: Rc<RefCell<Logic>>,
    paint: Rc<RefCell<Paint>>,
}

impl Game {
    pub fn new(canvas_id: &str, width: u32, height: u32, px: u32) -> Game {
        let document = Web::document(&Web::window());
        let canvas = Web::canvas(&document, canvas_id);

        canvas.set_width(width * px);
        canvas.set_height(height * px);

        let logic = Logic::new(width, height);
        let listener = Closure::wrap(Game::turn(Rc::clone(logic.snake_ref())));

        Web::set_listener(&document, "keypress", listener);
        Game {
            logic: Rc::new(RefCell::new(logic)),
            paint: Rc::new(RefCell::new(Paint::new(canvas, px))),
        }
    }

    fn turn(snake: Rc<RefCell<Snake>>) -> Box<dyn FnMut(web_sys::KeyboardEvent)> {
        Box::new(move |event: web_sys::KeyboardEvent| {
            let new_direction = match &event.code()[..] {
                constants::KEY_UP_CODE => Direction::Up,
                constants::KEY_DOWN_CODE => Direction::Down,
                constants::KEY_LEFT_CODE => Direction::Left,
                constants::KEY_RIGHT_CODE => Direction::Right,
                _ => snake.borrow().direction(),
            };
            snake.borrow_mut().set_direction(new_direction);
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>
    }

    pub fn paint(logic: Rc<RefCell<Logic>>, paint: Rc<RefCell<Paint>>) {
        {
            let snake = logic.borrow().snake();
            paint.borrow().fill_rect_vec_px(snake, Cell::Snake.color());
        }
        {
            let food = logic.borrow().food();
            paint.borrow().fill_rect_px(food, Cell::Food.color());
        }
    }

    pub fn play(&self, fps: i32) {
        self.logic.borrow_mut().reset();
        self.paint.borrow().fill(Cell::Empty.color());

        let render_logic = Rc::clone(&self.logic);
        let render_paint = Rc::clone(&self.paint);

        Animation::render(fps, move || {
            let passed = render_logic.borrow_mut().passed();
            let alive = render_logic.borrow_mut().step();
            if !alive {
                let score = &format!("Score: {}", render_logic.borrow().score())[..];
                utils::alert(score);
                utils::save_file(score, constants::SCORE_FILETYPE, constants::SCORE_FILENAME);
            }
            Game::paint(Rc::clone(&render_logic), Rc::clone(&render_paint));
            if let Some(position) = passed {
                render_paint
                    .borrow()
                    .fill_rect_px(position, Cell::Empty.color());
            }
            alive
        });
    }
}
