use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(target_family = "wasm")]
use web_sys::{window, HtmlElement};
use crate::helper::Vector;

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub pixel_size: usize,
    pub onclick: Option<Rc<RefCell<Box<dyn FnMut(Vector)>>>>,
}

#[allow(dead_code)]
impl Renderer {
    pub fn new(
        width: usize,
        height: usize,
        pixel_size: usize,
        onclick: Option<Box<dyn FnMut(Vector)>>,
    ) -> Self {
        let mut renderer = Self {
            width,
            height,
            pixel_size,
            onclick: None,
        };
        if onclick.is_some() {
            renderer.onclick = Some(Rc::new(RefCell::new(onclick.unwrap())));
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
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Renderer {
    pub fn render(&mut self, object: &impl Renderable) {
        let data = object.renderer_data();
        for y in 0..self.height {
            let mut str = String::from("");
            for x in 0..self.width {
                let value = data.get(&Vector(x as isize, y as isize)).unwrap();
                str.push_str(&format!("{} ", value));
            };
            println!("{}", str);
        };

        println!("\n\n\n");
    }
}

#[allow(unused_macros)]
macro_rules! style {
    ($element:ident { $($property:literal: $value:expr);+$(;)? }) => {{
        let style = $element.style();
        $(style.set_property($property, $value).unwrap();)+
    }};
}


#[cfg(target_family = "wasm")]
impl Renderer {
    pub fn render(&mut self, object: &impl Renderable) {
        let data = object.renderer_data();
        let window = window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let root = match document.get_element_by_id("display-root") {
            Some(root) => root.dyn_into().unwrap_throw(),
            None => {
                let root = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlElement>()
                    .unwrap_throw();
                let body = document.body().unwrap_throw();
                root.set_id("display-root");
                style!(root {
                    "display": "inline-grid";
                    "grid-template-columns": &format!("repeat({}, {}px)", self.width, self.pixel_size);
                    "border": "1px solid black";
                    "position": "absolute";
                    "left": "50vw";
                    "top": "30px";
                    "transform": "translateX(-50%)";
                });
                body
                    .append_child(&root)
                    .unwrap_throw();
                root
            }
        };
        root.set_inner_html("");

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

                root.append_child(&anchor).unwrap_throw();
            }
        }
    }
}
