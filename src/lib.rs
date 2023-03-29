use js_sys::Function;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::window;
mod controller;
mod helper;
mod random;
mod renderer;
mod snake;
use controller::*;
use renderer::*;
use snake::*;

const GAME_WIDTH: usize = 20;
const GAME_HEIGHT: usize = 20;

thread_local! {
    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(GAME_WIDTH, GAME_HEIGHT, 50)));
    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        GAME.with(|game| {
            RENDERER.with(|renderer| {
                let renderer = &mut *renderer.borrow_mut();
                game.borrow_mut().tick();
                renderer.render(&*game.borrow());
            })
        })
    }));
    static RENDERER: RefCell<Renderer> = RefCell::new(Renderer {
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
        pixel_size: 18,
        onclick: None

    });
    static CONTROLLER: Controller = Controller::new(Box::new({
        let game = GAME.with(|game| game.clone());
        move |direction| {
            game.borrow_mut().change_direction(direction);
        }
    }) as Box<dyn FnMut(Direction)>);
}

#[wasm_bindgen(start)]
fn main() {
    let window = window().unwrap_throw();
    TICK_CLOSURE.with(|tick_closure| {
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure
                    .clone()
                    .as_ref()
                    .dyn_ref::<Function>()
                    .unwrap_throw(),
                500,
            )
            .unwrap_throw();
    });
    CONTROLLER.with(|_| {});
}

#[cfg(test)]
mod tests {
    use crate::random::random_range;

    #[test]
    fn checking_random() -> Result<(), &'static str> {
        let random = random_range(0, 10);
        match random {
            0..=9 => Ok(()),
            _ => Err("Expected a value between 0 to 10")
        }
    }
}
