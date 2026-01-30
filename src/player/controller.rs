use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    engine::state_manager::GameState,
    player::{Character, CharacterPhysics},
    world::{GROUND_LEVEL, GROUND_WIDTH},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_systems(
            Update,
            calculate_physics.run_if(in_state(GameState::InGame)),
        )
        .add_input_context::<Character>()
        .add_observer(apply_movement)
        .add_observer(apply_jump);
}

const PLAYER: Vec2 = Vec2::new(128.0, 128.0);
const JUMP_VELOCITY: f32 = 300.0;
const GRAVITY: f32 = 900.0;

#[derive(Debug, InputAction)]
#[action_output(f32)]
pub struct Movement;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Jump;

fn apply_movement(movement: On<Fire<Movement>>, mut query: Query<&mut CharacterPhysics>) {
    let mut physics = query.get_mut(movement.context).unwrap();
    physics.velocity.x = movement.value;
}

fn apply_jump(jump: On<Fire<Jump>>, mut query: Query<&mut CharacterPhysics>) {
    let mut physics = query.get_mut(jump.context).unwrap();
    if physics.is_grounded {
        // Jump only if on the ground.
        physics.velocity.y = JUMP_VELOCITY;
        physics.is_grounded = false;
    }
}

fn calculate_physics(time: Res<Time>, mut query: Query<(&mut Transform, &mut CharacterPhysics)>) {
    for (mut transform, mut physics) in query.iter_mut() {
        physics.velocity.y -= GRAVITY * time.delta_secs();
        transform.translation.y += physics.velocity.y * time.delta_secs();
        transform.translation.x += physics.velocity.x * time.delta_secs();

        // Prevent moving off screen.
        const MAX_X: f32 = GROUND_WIDTH / 2.0 - PLAYER.x / 2.0;
        transform.translation.x = transform.translation.x.clamp(-MAX_X, MAX_X);

        // Check for ground collision.
        const GROUNDED_Y: f32 = GROUND_LEVEL + PLAYER.y / 2.0;
        if transform.translation.y <= GROUNDED_Y {
            transform.translation.y = GROUNDED_Y;
            physics.velocity.y = 0.0;
            physics.is_grounded = true;
        }
    }
}
