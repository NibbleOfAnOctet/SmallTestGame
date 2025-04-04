use bevy::prelude::*;
use std::collections::HashMap;

/// Represents a game action that can be bound to input
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Run,
    Attack,
    Roll,
    SwitchLevel1,
    SwitchLevel2,
}

/// Resource that stores the current input bindings
#[derive(Resource)]
pub struct InputBindings {
    bindings: HashMap<GameAction, Vec<KeyCode>>,
}

impl Default for InputBindings {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        
        // Default bindings
        bindings.insert(GameAction::MoveLeft, vec![KeyCode::KeyA, KeyCode::ArrowLeft]);
        bindings.insert(GameAction::MoveRight, vec![KeyCode::KeyD, KeyCode::ArrowRight]);
        bindings.insert(GameAction::MoveUp, vec![KeyCode::KeyW, KeyCode::ArrowUp]);
        bindings.insert(GameAction::MoveDown, vec![KeyCode::KeyS, KeyCode::ArrowDown]);
        bindings.insert(GameAction::Run, vec![KeyCode::ShiftLeft]);
        bindings.insert(GameAction::Attack, vec![KeyCode::KeyE]);
        bindings.insert(GameAction::Roll, vec![KeyCode::Space]);
        bindings.insert(GameAction::SwitchLevel1, vec![KeyCode::Digit1]);
        bindings.insert(GameAction::SwitchLevel2, vec![KeyCode::Digit2]);
        
        InputBindings { bindings }
    }
}

impl InputBindings {
    /// Check if a specific game action is currently pressed
    pub fn is_pressed(&self, action: GameAction, keyboard: &ButtonInput<KeyCode>) -> bool {
        if let Some(keys) = self.bindings.get(&action) {
            keys.iter().any(|&key| keyboard.pressed(key))
        } else {
            false
        }
    }
    
    /// Check if a specific game action was just pressed
    pub fn just_pressed(&self, action: GameAction, keyboard: &ButtonInput<KeyCode>) -> bool {
        if let Some(keys) = self.bindings.get(&action) {
            keys.iter().any(|&key| keyboard.just_pressed(key))
        } else {
            false
        }
    }
    
}