use std::ops::Add;
use wasm_bindgen::prelude::*;

#[allow(unused_macros)]
macro_rules! style {
    ($element:ident { $($property:literal: $value:expr);+$(;)? }) => {{
        let style = $element.style();
        $(style.set_property($property, $value).unwrap();)+
    }};
}

#[allow(unused_macros)]
macro_rules! onclick {
    ($element:ident -> $onclick:expr) => {
        let onclick = Closure::wrap(Box::new($onclick) as Box<dyn FnMut()>);
        $element.set_onclick(onclick.as_ref().dyn_ref());
        onclick.forget();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(s: &str);
}

#[allow(unused_macros)]
macro_rules! log {
    ($( $value:expr ),+) => {
        $(
            console_log(&format!("{:?}", $value));
        )+
    }
}

pub(crate) use style;
pub(crate) use onclick;
pub(crate) use log;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vector(pub isize, pub isize);

impl Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}
