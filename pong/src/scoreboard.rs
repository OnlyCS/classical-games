use crate::prelude::*;

pub const FONT_SIZE: f32 = 20.0;
pub const PADDING: Val = Val::Px(20.0 + arena::PADDING);

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub left_score: usize,
    pub right_score: usize,
}

#[derive(Component)]
pub struct LeftScoreboard;

#[derive(Component)]
pub struct RightScoreboard;

#[derive(Resource, Default)]
pub struct Hits {
    pub left: usize,
    pub right: usize,
}

pub fn update_left(
    scoreboard: Res<Scoreboard>,
    mut left_query: Query<&mut Text, With<LeftScoreboard>>,
) {
    let mut left_text = left_query.single_mut();

    left_text.sections[1].value = format!("{}", scoreboard.left_score);
}

pub fn update_right(
    scoreboard: Res<Scoreboard>,
    mut right_query: Query<&mut Text, With<RightScoreboard>>,
) {
    let mut right_text = right_query.single_mut();

    right_text.sections[1].value = format!("{}", scoreboard.right_score);
}

pub mod prelude {
    pub use super::{Hits, LeftScoreboard, RightScoreboard, Scoreboard};
}
