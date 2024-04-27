use glam::Vec2;

use crate::logic::camera::Camera;
use crate::logic::play::world::World;

pub mod world;

pub struct Play {
    pub camera: Camera,

    pub world: World,
    pub mouse_position: Vec2,
}

impl Play {
    pub fn new() -> Self {
        return Self {
            camera: Camera::new(),

            world: World::new(),
            mouse_position: Vec2::ZERO,
        };
    }

    pub fn process_mouse_position(&mut self, position: (u32, u32)) {
        self.mouse_position = Vec2::new(position.0 as f32, position.1 as f32);
    }
}

