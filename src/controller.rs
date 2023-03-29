use crate::helper::{console_log, log, onclick, style};
use regex::RegexSet;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlButtonElement, HtmlElement, KeyboardEvent};

use crate::snake::Direction;

fn is_mobile() -> bool {
    let window = window().unwrap_throw();
    let user_agent = window.navigator().user_agent().unwrap_throw();
    let mobile_agents = RegexSet::new(&[
        "/Android/i",
        "/webOS/i",
        "/iPhone/i",
        "/iPad/i",
        "/iPod/i",
        "/BlackBerry/i",
        "/Windows Phone/i",
    ])
    .unwrap();
    mobile_agents.matches(&user_agent).len() != 0
}

pub struct Controller {
    oninput: Rc<RefCell<Box<dyn FnMut(Direction)>>>,
}

impl Controller {
    pub fn new(oninput: Box<dyn FnMut(Direction)>) -> Self {
        let mut controller = Self {
            oninput: Rc::new(RefCell::new(oninput)),
        };
        controller.add_listeners();
        controller
    }
    fn add_listeners(&mut self) {
        let is_mobile = is_mobile();
        log!(is_mobile);
        let window = window().unwrap_throw();
        if is_mobile {
            let document = window.document().unwrap_throw();
            let root = document.get_element_by_id("controller-root");

            if root.is_none() {
                let body = document.body().unwrap_throw();
                let root = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();
                root.set_id("controller-root");
                body.append_child(&root).unwrap_throw();
                style!(root {
                    "position": "absolute";
                    "bottom": "0";
                    "left": "50vw";
                    "transform": "translateX(-50%)";
                    "display": "flex";
                    "flex-direction": "column";
                    "height": "250px";
                });
                let top = document
                    .create_element("button")
                    .unwrap_throw()
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap_throw();
                top.set_inner_html("^");
                style!(top {
                    "height": "33%";
                });
                root.append_child(&top).unwrap_throw();
                let middle = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();
                let left = document
                    .create_element("button")
                    .unwrap_throw()
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap_throw();
                let right = document
                    .create_element("button")
                    .unwrap_throw()
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap_throw();
                left.set_inner_html("<");
                right.set_inner_html(">");
                style!(middle {
                    "display": "flex";
                    "width": "300px";
                    "height": "34%";
                });
                style!(left {
                    "width": "50%";
                    "height": "100%";
                });
                style!(right {
                    "width": "50%";
                    "height": "100%";
                });
                middle.append_child(&left).unwrap_throw();
                middle.append_child(&right).unwrap_throw();
                root.append_child(&middle).unwrap_throw();
                let bottom = document
                    .create_element("button")
                    .unwrap_throw()
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap_throw();
                bottom.set_inner_html("|");
                style!(bottom {
                    "height": "33%";
                });
                root.append_child(&bottom).unwrap_throw();

                onclick!(top -> {
                    let oninput = self.oninput.clone();
                    move || {
                        oninput.borrow_mut()(Direction::Up);
                    }
                });
                onclick!(left -> {
                    let oninput = self.oninput.clone();
                    move || {
                        oninput.borrow_mut()(Direction::Left);
                    }
                });
                onclick!(right -> {
                    let oninput = self.oninput.clone();
                    move || {
                        oninput.borrow_mut()(Direction::Right);
                    }
                });
                onclick!(bottom -> {
                    let oninput = self.oninput.clone();
                    move || {
                        oninput.borrow_mut()(Direction::Down);
                    }
                });
            };
            return;
        };
        let keydown_closure = Closure::wrap(Box::new({
            let oninput = self.oninput.clone();
            move |e: KeyboardEvent| {
                let direction = match e.key().as_ref() {
                    "ArrowUp" => Some(Direction::Up),
                    "ArrowDown" => Some(Direction::Down),
                    "ArrowLeft" => Some(Direction::Left),
                    "ArrowRight" => Some(Direction::Right),
                    _ => None
                };
                if let Some(direction) = direction {
                    oninput.borrow_mut()(direction);
                };
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.set_onkeydown(keydown_closure.as_ref().dyn_ref());
    }
}
