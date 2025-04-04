use bevy::prelude::*;
use bevy::core_pipeline::core_2d::Camera2d;
use crate::player::Player;
use super::LevelEntity;

pub fn load_level(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera2d::default(),
        LevelEntity,
    ));

    // Player
    commands.spawn((
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player::default(),
        LevelEntity,
    ));
}