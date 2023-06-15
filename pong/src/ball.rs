use crate::prelude::*;

// use z-coord for overlapping
pub const START_POS: Vec3 = Vec3::new(CENTER.x, CENTER.y, 5.0);
pub const SIZE: Vec2 = Vec2::new(4.0, 4.0);

pub const START_DIRECTION: Vec2 = Vec2::new(CENTER.x + 0.5, CENTER.y - 0.25);
pub const START_SPEED: f32 = 200.0;

pub fn start_velocity() -> Vec2 {
    START_DIRECTION.normalize() * START_SPEED
}

#[derive(Component)]
pub struct Ball;

pub mod prelude {
    pub use super::Ball;
}
