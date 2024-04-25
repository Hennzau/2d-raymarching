use winit::{
    event::{
        ElementState,
        KeyEvent,
        MouseButton
    },
    window::Window
};


pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn process_keyboard(&mut self, window: &Window, key_event: KeyEvent) {}

    pub fn process_mouse_input(&mut self, window: &Window, state: ElementState, mouse_button: MouseButton) {}

    pub fn process_mouse_motion(&mut self, delta: (f32, f32)) {}

    pub fn update(&mut self, delta_time: f32) {}
}