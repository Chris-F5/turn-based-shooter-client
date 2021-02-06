use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

struct KeyboardState {
    up_arrow: bool,
    down_arrow: bool,
    right_arrow: bool,
    left_arrow: bool,
}
impl KeyboardState {
    pub fn all_up() -> KeyboardState {
        KeyboardState {
            up_arrow: false,
            down_arrow: false,
            right_arrow: false,
            left_arrow: false,
        }
    }
    pub fn set_key(&mut self, key_code: u32, state: bool) {
        match key_code {
            38 => self.up_arrow = state,
            40 => self.down_arrow = state,
            39 => self.right_arrow = state,
            37 => self.left_arrow = state,
            _ => (),
        }
    }
}

pub struct Input {
    keyboard_state: Rc<RefCell<KeyboardState>>,
}
impl Input {
    pub fn new() -> Input {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let keyboard_state = Rc::new(RefCell::new(KeyboardState::all_up()));

        let keyboard_state_clone = keyboard_state.clone();
        let key_down_handler = Closure::wrap(Box::new(move |event: JsValue| {
            let event = event.dyn_into::<KeyboardEvent>().unwrap();
            let mut keyboard_state = keyboard_state_clone.borrow_mut();
            keyboard_state.set_key(event.key_code(), true);
        }) as Box<dyn FnMut(JsValue)>);

        let keyboard_state_clone = keyboard_state.clone();
        let key_up_handler = Closure::wrap(Box::new(move |event: JsValue| {
            let event = event.dyn_into::<KeyboardEvent>().unwrap();
            let mut keyboard_state = keyboard_state_clone.borrow_mut();
            keyboard_state.set_key(event.key_code(), false);
        }) as Box<dyn FnMut(JsValue)>);

        document
            .add_event_listener_with_callback("keydown", key_down_handler.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("keyup", key_up_handler.as_ref().unchecked_ref())
            .unwrap();
        key_down_handler.forget();
        key_up_handler.forget();

        Input { keyboard_state }
    }

    pub fn up_arrow(&self) -> bool {
        self.keyboard_state.borrow().up_arrow
    }
    pub fn down_arrow(&self) -> bool {
        self.keyboard_state.borrow().down_arrow
    }
    pub fn right_arrow(&self) -> bool {
        self.keyboard_state.borrow().right_arrow
    }
    pub fn left_arrow(&self) -> bool {
        self.keyboard_state.borrow().left_arrow
    }
}
