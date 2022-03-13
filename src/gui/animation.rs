use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::gui::web::Web;

pub struct Animation;

impl Animation {
    pub fn request_animation_frame(closure: &Closure<dyn FnMut()>) {
        Web::window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    pub fn set_timeout(closure: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
        Web::window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                timeout_ms,
            )
            .expect("should register `setTimeout` OK")
    }

    pub fn render(fps: i32, mut draw: impl FnMut() -> bool + 'static) {
        let tick = Rc::new(RefCell::new(None));
        let timeout = Rc::new(RefCell::new(None));

        let animate = Rc::clone(&tick);
        let delay = Rc::clone(&timeout);

        *timeout.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            Animation::request_animation_frame(tick.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        *animate.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let continue_ = draw();
            if continue_ {
                Animation::set_timeout(delay.borrow().as_ref().unwrap(), 1_000 / fps);
            }
        }) as Box<dyn FnMut()>));

        Animation::request_animation_frame(animate.borrow().as_ref().unwrap());
    }
}
