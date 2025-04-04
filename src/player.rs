use bevy::prelude::*;
use crate::input::{GameAction, InputBindings};

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub speed: f32,
    pub run_speed_multiplier: f32,
    pub roll_speed: f32,
    pub roll_timer: Timer,
    pub roll_cooldown: Timer,
    pub attack_timer: Timer,
    pub roll_direction: Vec2,
}

#[derive(Component, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Walking,
    Running,
    Rolling,
    Attacking,
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    input_bindings: Res<InputBindings>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();
    let delta = time.delta().as_secs_f32();

    // Update roll cooldown
    player.roll_cooldown.tick(time.delta());

    // Handle rolling movement separately
    if player.state == PlayerState::Rolling {
        let roll_movement = player.roll_direction * player.roll_speed * delta;
        transform.translation.x += roll_movement.x;
        transform.translation.y += roll_movement.y;
        return;
    }

    // Don't process movement if attacking
    if player.state == PlayerState::Attacking {
        return;
    }

    // Get movement direction from input system
    let mut direction = Vec2::ZERO;
    
    if input_bindings.is_pressed(GameAction::MoveLeft, &keyboard) {
        direction.x -= 1.0;
    }
    if input_bindings.is_pressed(GameAction::MoveRight, &keyboard) {
        direction.x += 1.0;
    }
    if input_bindings.is_pressed(GameAction::MoveUp, &keyboard) {
        direction.y += 1.0;
    }
    if input_bindings.is_pressed(GameAction::MoveDown, &keyboard) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        let speed = if input_bindings.is_pressed(GameAction::Run, &keyboard) {
            player.speed * player.run_speed_multiplier
        } else {
            player.speed
        };
        
        let movement = direction * speed * delta;
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        player.state = if input_bindings.is_pressed(GameAction::Run, &keyboard) {
            PlayerState::Running
        } else {
            PlayerState::Walking
        };

        // Handle rolling - only allow when walking or running and cooldown is finished
        if input_bindings.just_pressed(GameAction::Roll, &keyboard) && player.roll_cooldown.finished() {
            player.state = PlayerState::Rolling;
            player.roll_timer.reset();
            player.roll_direction = direction;
            player.roll_speed = player.speed * player.run_speed_multiplier * 2.0;
            player.roll_cooldown.reset();
        }
    } else {
        player.state = PlayerState::Idle;
    }

    // Handle attacking
    if input_bindings.just_pressed(GameAction::Attack, &keyboard) && player.state != PlayerState::Attacking {
        player.state = PlayerState::Attacking;
        player.attack_timer.reset();
    }
}

pub fn player_state_update(
    time: Res<Time>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();

    match player.state {
        PlayerState::Rolling => {
            player.roll_timer.tick(time.delta());
            if player.roll_timer.finished() {
                player.state = PlayerState::Idle;
            }
        }
        PlayerState::Attacking => {
            player.attack_timer.tick(time.delta());
            if player.attack_timer.finished() {
                player.state = PlayerState::Idle;
            }
        }
        _ => {}
    }
}

pub fn update_player_color(
    mut query: Query<(&Player, &mut Sprite)>,
) {
    let (player, mut sprite) = query.single_mut();
    
    sprite.color = match player.state {
        PlayerState::Idle => Color::srgb(0.25, 0.25, 0.75),    // Blue
        PlayerState::Walking => Color::srgb(0.25, 0.75, 0.25), // Green
        PlayerState::Running => Color::srgb(0.75, 0.25, 0.25), // Red
        PlayerState::Rolling => Color::srgb(0.75, 0.75, 0.25), // Yellow
        PlayerState::Attacking => Color::srgb(0.75, 0.25, 0.75), // Purple
    };
} 

impl Player{
    pub fn default()->Self{
        Self {
            state: PlayerState::Idle,
            speed: 300.0,
            run_speed_multiplier: 2.0,
            roll_speed: 400.0,
            roll_timer: Timer::from_seconds(0.2, TimerMode::Once),
            roll_cooldown: Timer::from_seconds(0.35, TimerMode::Once),
            attack_timer: Timer::from_seconds(0.3, TimerMode::Once),
            roll_direction: Vec2::ZERO,
        }
    }

    pub fn systems() -> bevy::ecs::schedule::NodeConfigs<Box<dyn System<In = (), Out = ()>>> {
        (player_movement,
            player_state_update,
            update_player_color).chain()
    }
}