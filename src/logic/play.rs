use glam::{IVec2, Mat4, UVec2, Vec2, Vec3, Vec4, Vec4Swizzles};

use winit::{
    event::{
        ElementState,
        KeyEvent,
        MouseButton,
    },
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
    window::Window,
};

use crate::logic::camera::{
    Camera,
    CameraController,
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
    pub model: Mat4,
}

impl Play {
    pub fn new() -> Self {
        return Self {
            camera: Camera::new(),
            controller: CameraController::new(),

            state: PlayState::Playing,
            pipeline: PipelineType::TestRasterizer,

            world: World::new(),
            model: Mat4::from_translation(Vec3::new(140.0, 60.0, 0.0)),
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

    pub fn process_mouse_position(&mut self, position: (u32, u32)) {
        let x = 2.0 * position.0 as f32 / 1600.0 - 1.0;
        let y = 1.0 - (2.0 * position.1 as f32) / 900.0;
        let z = 1.0f32;

        let ray_nds = Vec3::new(x, y, z);
        let ray_clip = Vec4::new(ray_nds.x, ray_nds.y, -1.0, 1.0);

        let ray_eye = (self.camera.mvp((1600, 900)) * self.model).inverse() * ray_clip;
        let tile = IVec2::new(ray_eye.x as i32 / 20, ray_eye.y as i32 / 20);

        println!("{:?}", tile);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.controller.update(delta_time, &mut self.camera);
    }
}

