use glam::Vec2;

use winit::{
    event::{
        ElementState,
        KeyEvent,
        MouseButton
    },
    keyboard::{
        KeyCode,
        PhysicalKey
    },
    window::Window,
};

use crate::logic::camera::{
    Camera,
    CameraController
};
use crate::logic::play::world::World;

pub mod world;

#[derive(PartialEq)]
pub enum PlayState {
    Playing
}

#[derive(PartialEq)]
pub enum PipelineType {
    TestRasterizer,
    TestRayMarcher,
}

pub struct Play {
    pub camera: Camera,
    pub controller: CameraController,

    pub state: PlayState,
    pub pipeline: PipelineType,

    pub world: World,
}

impl Play {
    pub fn new() -> Self {
        return Self {
            camera: Camera::new(),
            controller: CameraController::new(),

            state: PlayState::Playing,
            pipeline: PipelineType::TestRasterizer,

            world: World::new()
        };
    }

    pub fn process_keyboard(&mut self, window: &Window, key_event: KeyEvent) {
        match key_event {
            KeyEvent {
                physical_key,
                state,
                ..
            } => {
                match physical_key {
                    PhysicalKey::Code(KeyCode::KeyE) => {
                        if state == ElementState::Pressed {
                            self.pipeline = PipelineType::TestRasterizer;
                            self.camera.position = Vec2::new(0f32, 0f32);
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyR) => {
                        if state == ElementState::Pressed {
                            self.pipeline = PipelineType::TestRayMarcher;
                            self.camera.position = Vec2::new(0f32, 0f32);
                        }
                    }
                    PhysicalKey::Code(KeyCode::Enter) => {
                        self.camera.position = Vec2::new(0f32, 0f32);
                    }
                    _ => {}
                }
            }
        }

        self.controller.process_keyboard(key_event);
    }

    pub fn process_mouse_input(&mut self, window: &Window, state: ElementState, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => {}
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Back => {}
            MouseButton::Forward => {}
            MouseButton::Other(_) => {}
        }
    }

    pub fn process_mouse_motion(&mut self, delta: (f32, f32)) {}

    pub fn update(&mut self, delta_time: f32) {
        self.controller.update(delta_time, &mut self.camera);
    }
}

