mod area;
mod ball;
mod colors;
mod paddle;
mod physics;
mod prelude;
mod scoreboard;

#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resolution: (500.0, 500.0).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_event::<CollisionEvent>()
        .insert_resource(Hits::default())
        .insert_resource(Scoreboard::default())
        .insert_resource(ClearColor(colors::BACKGROUND))
        .add_systems(
            (
                physics::handle_collision_physics,
                physics::apply_velocity.before(physics::handle_collision_physics),
                handle_collision_events.after(physics::handle_collision_physics),
                paddle::on_left_key
                    .before(physics::handle_collision_physics)
                    .after(physics::apply_velocity),
                paddle::on_right_key
                    .after(paddle::on_left_key)
                    .before(physics::handle_collision_physics),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_startup_system(setup)
        .insert_resource(FixedTime::new_from_secs(physics::TIME_STEP))
        .add_systems((scoreboard::update_left, scoreboard::update_right))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    info!("Camera spawned");

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(paddle::LEFT_X, CENTER.y, 0.0),
                scale: paddle::SIZE.extend(0.0),
                ..default()
            },
            ..default()
        },
        Paddle,
        LeftPaddle,
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(paddle::RIGHT_X, CENTER.y, 0.0),
                scale: paddle::SIZE.extend(0.0),
                ..default()
            },
            ..default()
        },
        Paddle,
        RightPaddle,
        Collider,
    ));

    info!("Paddles spawned");

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: ball::START_POS,
                scale: Vec3::new(ball::SIZE.x, ball::SIZE.y, 0.0),
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity(ball::start_velocity()),
    ));

    info!("Ball spawned");

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));

    info!("Walls spawned");

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(CENTER.x, CENTER.y, 0.0),
                scale: Vec3::new(1.0, arena::HEIGHT, 0.0),
                ..default()
            },
            ..default()
        },
        Divider,
    ));

    info!("Divider spawned");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/meslo_bold.ttf"),
                    font_size: scoreboard::FONT_SIZE,
                    color: colors::TEXT,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/meslo_bold.ttf"),
                font_size: scoreboard::FONT_SIZE,
                color: colors::TEXT,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: scoreboard::PADDING,
                left: scoreboard::PADDING,
                ..default()
            },
            ..default()
        }),
        LeftScoreboard,
    ));

    info!("Left Scoreboard spawned");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/meslo_bold.ttf"),
                    font_size: scoreboard::FONT_SIZE,
                    color: colors::TEXT,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/meslo_bold.ttf"),
                font_size: scoreboard::FONT_SIZE,
                color: colors::TEXT,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: scoreboard::PADDING,
                right: scoreboard::PADDING,
                ..default()
            },
            ..default()
        }),
        RightScoreboard,
    ));

    info!("Right Scoreboard spawned");
}

fn handle_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut hits: ResMut<Hits>,
    mut ball_query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let events = collision_events.into_iter();

    let (mut ball_velocity, mut ball_transform) = ball_query.single_mut();

    for event in events {
        match *event {
            CollisionEvent::LeftWall => {
                *ball_velocity = Velocity(ball::start_velocity());
                ball_transform.translation = ball::START_POS;

                scoreboard.right_score += 1;
            }
            CollisionEvent::RightWall => {
                *ball_velocity = Velocity(ball::start_velocity());
                ball_transform.translation = ball::START_POS;

                scoreboard.left_score += 1;
            }
            CollisionEvent::LeftPaddle => {
                hits.left += 1;
            }
            CollisionEvent::RightPaddle => {
                hits.right += 1;
            }
            _ => {}
        }

        info!("Received collision event: {:?}", event);
    }
}
