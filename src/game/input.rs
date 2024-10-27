use std::collections::HashSet;

pub use miniquad::{KeyCode, MouseButton};

pub struct Input {
    keys_down: HashSet<KeyCode>,
    mouse_buttons_down: HashSet<MouseButton>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            mouse_buttons_down: HashSet::new(),
        }
    }

    pub(crate) fn set_key_down(&mut self, key: KeyCode) {
        self.keys_down.insert(key);
    }
    pub(crate) fn set_key_up(&mut self, key: KeyCode) {
        self.keys_down.remove(&key);
    }
    pub(crate) fn set_mb_down(&mut self, button: MouseButton) {
        self.mouse_buttons_down.insert(button);
    }
    pub(crate) fn set_mb_up(&mut self, button: MouseButton) {
        self.mouse_buttons_down.remove(&button);
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }
}
