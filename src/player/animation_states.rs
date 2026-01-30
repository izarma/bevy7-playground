use std::time::Duration;

use bevy::prelude::*;

use crate::{consts::ANIMATION_FPS, engine::AppSystems, player::CharacterPhysics};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (update_animation_movement, update_animation_atlas).chain(),
        ),
    );
}

fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_q: Query<(&CharacterPhysics, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (physics, mut sprite, mut animation) in &mut player_q {
        sprite.flip_x = physics.velocity.x < 0.0;
        let next_state = if !physics.is_grounded {
            AnimationState::Jump
        } else if physics.velocity.x.abs() > 5. {
            AnimationState::Walk
        } else {
            AnimationState::Idle
        };
        animation.update_state(next_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Jump,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub timer: Timer,
    pub frame: usize,
    pub state: AnimationState,
}

impl PlayerAnimation {
    const IDLE_FRAMES: usize = 7;
    const WALK_FRAMES: usize = 8;
    const JUMP_FRAMES: usize = 12;
    fn idling() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / ANIMATION_FPS, TimerMode::Repeating),
            frame: 0,
            state: AnimationState::Idle,
        }
    }
    fn walking() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / ANIMATION_FPS, TimerMode::Repeating),
            frame: 0,
            state: AnimationState::Walk,
        }
    }
    fn jumping() -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / ANIMATION_FPS, TimerMode::Repeating),
            frame: 0,
            state: AnimationState::Jump,
        }
    }
    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                AnimationState::Idle => Self::IDLE_FRAMES,
                AnimationState::Walk => Self::WALK_FRAMES,
                AnimationState::Jump => Self::JUMP_FRAMES,
            };
    }
    fn update_state(&mut self, state: AnimationState) {
        if self.state != state {
            match state {
                AnimationState::Idle => *self = Self::idling(),
                AnimationState::Walk => *self = Self::walking(),
                AnimationState::Jump => *self = Self::jumping(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            AnimationState::Idle => self.frame,
            AnimationState::Walk => 12 + self.frame,
            AnimationState::Jump => 24 + self.frame,
        }
    }
}
