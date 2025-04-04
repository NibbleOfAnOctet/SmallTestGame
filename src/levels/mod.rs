pub mod level1;
pub mod level2;

use bevy::prelude::*;
use crate::input::{GameAction, InputBindings};

#[derive(Component)]
pub struct LevelEntity;

pub fn debug_switch_level(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    input_bindings: Res<InputBindings>,
    query: Query<Entity, With<LevelEntity>>,
) {
    if input_bindings.just_pressed(GameAction::SwitchLevel1, &keyboard) || 
       input_bindings.just_pressed(GameAction::SwitchLevel2, &keyboard) {
        // Despawn all level entities
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        if input_bindings.just_pressed(GameAction::SwitchLevel1, &keyboard) {
            level1::load_level(commands);
        } else if input_bindings.just_pressed(GameAction::SwitchLevel2, &keyboard) {
            level2::load_level(commands);
        }
    }
} 