use crate::prelude::*;

pub const SIZE: Vec2 = Vec2::new(2.0, 50.0);
pub const SPEED: f32 = 350.0;

pub const PADDING: f32 = 10.0;
pub const RIGHT_X: f32 = arena::RIGHT_X - PADDING;
pub const LEFT_X: f32 = arena::LEFT_X + PADDING;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct LeftPaddle;

#[derive(Component)]
pub struct RightPaddle;

fn move_paddle(mut paddle_transform: Mut<'_, Transform>, movement: f32) {
    let new_pos = paddle_transform.translation.y + movement * SPEED * physics::TIME_STEP;

    let bottom_bound = arena::BOTTOM_Y + SIZE.y / 2.0;
    let top_bound = arena::TOP_Y - SIZE.y / 2.0;

    paddle_transform.translation.y = new_pos.min(top_bound).max(bottom_bound);
}

pub fn on_right_key(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<RightPaddle>>,
) {
    let paddle_transform = query.single_mut();
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        movement += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement -= 1.0;
    }

    move_paddle(paddle_transform, movement);
}

pub fn on_left_key(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<LeftPaddle>>,
) {
    let paddle_transform = query.single_mut();
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        movement += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement -= 1.0;
    }

    move_paddle(paddle_transform, movement);
}

pub mod prelude {
    pub use super::{LeftPaddle, Paddle, RightPaddle};
}
