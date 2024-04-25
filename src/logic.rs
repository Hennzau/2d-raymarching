use winit::{
    window::Window,
    event::{
        ElementState,
        KeyEvent,
        MouseButton,
    },
};

use crate::logic::{
    menu::Menu,
    play::Play
};

pub mod camera;
pub mod play;
pub mod menu;

#[derive(PartialEq)]
pub enum LogicState {
    Playing,
    Menu,
}

pub struct Logic {
    pub state: LogicState,

    pub play: Play,
    pub menu: Menu,
}

impl Logic {
    pub fn new() -> Self {
        return Self {
            state: LogicState::Playing,
            play: Play::new(),
            menu: Menu::new(),
        };
    }

    pub fn process_keyboard(&mut self, window: &Window, key_event: KeyEvent) {
        match self.state {
            LogicState::Playing => {
                self.play.process_keyboard(window, key_event);
            }
            LogicState::Menu => {
                self.menu.process_keyboard(window, key_event);
            }
        }
    }

    pub fn process_mouse_input(&mut self, window: &Window, state: ElementState, mouse_button: MouseButton) {
        match self.state {
            LogicState::Playing => {
                self.play.process_mouse_input(window, state, mouse_button);
            }
            LogicState::Menu => {
                self.menu.process_mouse_input(window, state, mouse_button);
            }
        }
    }

    pub fn process_mouse_motion(&mut self, delta: (f32, f32)) {
        match self.state {
            LogicState::Playing => {
                self.play.process_mouse_motion(delta);
            }
            LogicState::Menu => {
                self.menu.process_mouse_motion(delta);
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        match self.state {
            LogicState::Playing => {
                self.play.update(delta_time);
            }
            LogicState::Menu => {
                self.menu.update(delta_time);
            }
        }
    }
}