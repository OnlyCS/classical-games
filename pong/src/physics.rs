use crate::prelude::*;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const WALL_THICKNESS: f32 = 1.0;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum CollisionEvent {
    #[default]
    LeftPaddle,
    RightPaddle,
    RightWall,
    LeftWall,
    TopWall,
    BottomWall,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    pub fn new(location: WallLocation) -> Self {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: match location {
                        WallLocation::Left => Vec3::new(arena::LEFT_X, CENTER.y, 0.0),
                        WallLocation::Right => Vec3::new(arena::RIGHT_X, CENTER.y, 0.0),
                        WallLocation::Top => Vec3::new(CENTER.x, arena::TOP_Y, 0.0),
                        WallLocation::Bottom => Vec3::new(CENTER.x, arena::BOTTOM_Y, 0.0),
                    },
                    scale: match location {
                        WallLocation::Left | WallLocation::Right => {
                            Vec3::new(WALL_THICKNESS, arena::HEIGHT, 1.0)
                        }
                        WallLocation::Top | WallLocation::Bottom => {
                            Vec3::new(arena::WIDTH, WALL_THICKNESS, 1.0)
                        }
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: colors::WALL,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

pub fn handle_collision_physics(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut collision_events: EventWriter<CollisionEvent>,
    collider_query: Query<(&Transform, Option<&Paddle>), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for (transform, maybe_paddle) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            let paddle_hit = maybe_paddle.is_some();
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => {
                    if paddle_hit {
                        reflect_x = ball_velocity.x > 0.0;
                        collision_events.send(CollisionEvent::RightPaddle);
                    } else {
                        collision_events.send(CollisionEvent::LeftWall);
                    }
                }
                Collision::Right => {
                    if paddle_hit {
                        reflect_x = ball_velocity.x < 0.0;
                        collision_events.send(CollisionEvent::LeftPaddle);
                    } else {
                        collision_events.send(CollisionEvent::RightWall);
                    }
                }
                Collision::Top => {
                    reflect_y = ball_velocity.y < 0.0;
                    collision_events.send(CollisionEvent::BottomWall);
                }
                Collision::Bottom => {
                    reflect_y = ball_velocity.y > 0.0;
                    collision_events.send(CollisionEvent::TopWall);
                }
                Collision::Inside => {}
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

pub mod prelude {
    pub use super::{Collider, CollisionEvent, Velocity, WallBundle, WallLocation};
}
