use js_sys::Function;
use std::cell::RefCell;
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
    static GAME: RefCell<SnakeGame> = RefCell::new(SnakeGame::new(GAME_WIDTH, GAME_HEIGHT, 50));
    static INTERVAL_ID: RefCell<Option<i32>> = RefCell::new(None);
    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        GAME.with(|game| {
            RENDERER.with(|renderer| {
                let renderer = &mut *renderer.borrow_mut();
                game.borrow_mut().tick();
                let game = game.borrow();
                renderer.score = game.snake.len() - 1;
                renderer.render(&*game);
                if game.finished {
                    INTERVAL_ID.with(|interval_id| {
                        let interval_id = *interval_id.borrow();
                        if let Some(interval_id) = interval_id {
                            window().unwrap_throw().clear_interval_with_handle(interval_id);
                        }
                    });
                };
                if renderer.score > renderer.high_score {
                    let _ = save_game_data(GameData { high_score: renderer.score });
                    renderer.high_score = renderer.score;
                }
            })
        })
    }));
    static RENDERER: RefCell<Renderer> = RefCell::new(Renderer {
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
        pixel_size: 18,
        onclick: None,
        score: 0,
        high_score: load_game_data()
            .unwrap_or_default()
            .high_score
    });
    static CONTROLLER: Controller = Controller::new(Box::new({
        move |direction| {
            TICK_CLOSURE.with(|tick_closure| {
                GAME.with(|game| {
                        let mut game = game.borrow_mut();
                        game.change_direction(direction);
                        if game.finished { return };
                        INTERVAL_ID.with(|interval_id| {
                            let interval_id = &mut *interval_id.borrow_mut();

                            RENDERER.with(|renderer| {
                                let renderer = &mut *renderer.borrow_mut();
                                game.tick();
                                renderer.score = game.snake.len() - 1;
                                renderer.render(&*game);
                                if game.finished {
                                    if let Some(id) = *interval_id {
                                        window().unwrap_throw().clear_interval_with_handle(id);
                                        *interval_id = None;
                                    }
                                };
                                if renderer.score > renderer.high_score {
                                    let _ = save_game_data(GameData { high_score: renderer.score });
                                    renderer.high_score = renderer.score;
                                }
                            });

                            if let Some(id) = *interval_id {
                                let window = window().unwrap_throw();
                                window.clear_interval_with_handle(id);
                                *interval_id = Some(
                                    window
                                        .set_interval_with_callback_and_timeout_and_arguments_0(
                                            tick_closure
                                                .as_ref()
                                                .dyn_ref::<Function>()
                                                .unwrap_throw(),
                                            500
                                        ).unwrap_throw()
                                    );
                            };
                    });
                });
            });
        }
    }) as Box<dyn FnMut(Direction)>);
}

#[wasm_bindgen(start)]
fn main() {
    let window = window().unwrap_throw();
    TICK_CLOSURE.with(|tick_closure| {
        INTERVAL_ID.with(|interval_id| {
            *interval_id.borrow_mut() = Some(
                window
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                        500,
                    )
                    .unwrap_throw(),
            );
        });
    });
    CONTROLLER.with(|_| {});
}
