use crate::helper::{CallbackFn, Vector};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
    pub onclick: Option<CallbackFn<Vector>>,
    pub score: usize,
    pub high_score: usize,
}

#[allow(dead_code)]
impl Renderer {
    pub fn new(
        width: usize,
        height: usize,
        pixel_size: usize,
        onclick: Option<Box<dyn FnMut(Vector)>>,
        high_score: usize,
    ) -> Self {
        let mut renderer = Self {
            width,
            height,
            pixel_size,
            onclick: None,
            score: 0,
            high_score,
        };
        if let Some(onclick) = onclick {
            renderer.onclick = Some(Rc::new(RefCell::new(onclick)));
        }
        renderer
    }
}

pub trait Renderable {
    fn renderer_data(&self) -> HashMap<Vector, char>;
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            pixel_size: 30,
            onclick: None,
            score: 0,
            high_score: 0,
        }
    }
}

#[allow(unused_macros)]
macro_rules! style {
    ($element:ident { $($property:literal: $value:expr);+$(;)? }) => {{
        let style = $element.style();
        $(style.set_property($property, $value).unwrap();)+
    }};
}

impl Renderer {
    pub fn render(&mut self, object: &impl Renderable) {
        let data = object.renderer_data();
        let window = window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let (display, score_counter) = match document.get_element_by_id("display") {
            Some(display) => (
                display.dyn_into().unwrap_throw(),
                document
                    .get_element_by_id("score-counter")
                    .unwrap_throw()
                    .dyn_into()
                    .unwrap_throw(),
            ),
            None => {
                let root = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();
                let display = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();
                let body = document.body().unwrap_throw();
                let score_counter = document
                    .create_element("h3")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();
                root.set_id("renderer-root");
                display.set_id("display");
                score_counter.set_id("score-counter");
                style!(root {
                    "position": "absolute";
                    "left": "50vw";
                    "top": "30px";
                    "transform": "translateX(-50%)";
                });
                style!(display {
                    "display": "inline-grid";
                    "grid-template-columns": &format!("repeat({}, {}px)", self.width, self.pixel_size);
                    "border": "1px solid black";
                });
                style!(score_counter {
                    "text-align": "center";
                    "font-family": "Roboto, sans serif";
                });
                root.append_child(&display).unwrap_throw();
                root.append_child(&score_counter).unwrap_throw();
                body.append_child(&root).unwrap_throw();
                (display, score_counter)
            }
        };
        display.set_inner_html("");
        score_counter.set_inner_html(&format!("score: {}", self.score));

        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Vector(x as isize, y as isize);
                let value = data.get(&coords).unwrap_throw();

                let anchor = document
                    .create_element("a")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();
                anchor.set_text_content(Some(&format!("{}", value)));
                if self.onclick.is_some() {
                    let onclick = self.onclick.as_ref().unwrap();
                    let onclick_closure = Closure::wrap(Box::new({
                        let onclick = onclick.clone();
                        move || {
                            (onclick.borrow_mut())(coords);
                        }
                    }) as Box<dyn FnMut()>);
                    anchor.set_onclick(onclick_closure.as_ref().dyn_ref())
                }
                style!(anchor {
                    "width": &format!("{}px", self.pixel_size);
                    "height": &format!("{}px", self.pixel_size)
                });

                display.append_child(&anchor).unwrap_throw();
            }
        }
    }
}
