use glam::{
    Mat4,
    Vec2,
    Vec3,
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