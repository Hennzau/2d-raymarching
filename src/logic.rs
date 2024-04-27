use crate::logic::play::Play;

pub mod camera;
pub mod play;


pub struct Logic {
    pub play: Play,
}

impl Logic {
    pub fn new() -> Self {
        return Self {
            play: Play::new(),
        };
    }

    pub fn process_mouse_position(&mut self, position: (u32, u32)) {
        self.play.process_mouse_position(position);
    }
}