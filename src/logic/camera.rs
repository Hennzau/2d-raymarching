use std::f32::consts::{
    FRAC_PI_2,
    PI
};

use glam::{
    Mat4,
    Vec2,
    Vec3,
};

use winit::{
    event::{
        ElementState,
        KeyEvent,
    },
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
};

pub struct Camera {
    pub position: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        return Self {
            position: Vec2::ZERO,
        };
    }

    pub fn mvp (&self, (width, height): (u32, u32)) -> Mat4 {
        let projection = Mat4::orthographic_rh(0f32, width as f32, 0f32, height as f32, -1f32, 1f32);
        let view = Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0f32));

        return projection * view;
    }
}

pub struct CameraController {
    movement_speed: f32,
    rotation_speed: f32,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,

    rotate_right: bool,
    rotate_left: bool,
    rotate_up: bool,
    rotate_down: bool,

    horizontal_delta: f32,
    vertical_delta: f32,
}

impl CameraController {
    pub fn new() -> Self {
        return Self {
            movement_speed: 50f32,
            rotation_speed: 1f32,

            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,

            rotate_right: false,
            rotate_left: false,
            rotate_up: false,
            rotate_down: false,

            horizontal_delta: 0f32,
            vertical_delta: 0f32,
        };
    }

    pub fn process_keyboard(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                physical_key,
                state,
                ..
            } => {
                match physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => {
                        self.is_forward_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyS) => {
                        self.is_backward_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyA) => {
                        self.is_left_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyD) => {
                        self.is_right_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::Space) => {
                        self.is_up_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::ShiftLeft) => {
                        self.is_down_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.rotate_left = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.rotate_right = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.rotate_up = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.rotate_down = state == ElementState::Pressed;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: f32, camera: &mut Camera) {

    }
}