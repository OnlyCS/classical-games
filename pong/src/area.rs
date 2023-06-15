pub mod window {
    use crate::prelude::*;

    pub const WIDTH: f32 = 500.0;
    pub const HEIGHT: f32 = 500.0;
    pub const TOP_Y: f32 = 250.0;
    pub const BOTTOM_Y: f32 = -250.0;
    pub const LEFT_X: f32 = -250.0;
    pub const RIGHT_X: f32 = 250.0;
    pub const CENTER: Vec2 = Vec2::new(0.0, 0.0);
}

pub mod arena {
    use crate::prelude::*;

    pub const PADDING: f32 = 5.0;
    pub const WIDTH: f32 = window::WIDTH - PADDING * 2.0;
    pub const HEIGHT: f32 = window::HEIGHT - PADDING * 2.0;
    pub const TOP_Y: f32 = window::TOP_Y - PADDING;
    pub const BOTTOM_Y: f32 = window::BOTTOM_Y + PADDING;
    pub const LEFT_X: f32 = window::LEFT_X + PADDING;
    pub const RIGHT_X: f32 = window::RIGHT_X - PADDING;

    #[derive(Component)]
    pub struct Divider;
}

pub mod prelude {
    pub use super::{arena, arena::Divider, window, window::CENTER};
}
