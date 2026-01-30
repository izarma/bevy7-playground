use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_systems(
        Update,
        (
            turn_on.run_if(in_state(GameState::Title)),
            turn_off.run_if(in_state(GameState::InGame)),
        ),
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Title,
    Loading,
    InGame,
}

#[derive(Component)]
pub struct OnTitleScreen;

#[derive(Component)]
pub struct OnGameplayScreen;

fn turn_on(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    q: Query<Entity, With<OnTitleScreen>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Loading);
        for entity in q.iter() {
            cmd.entity(entity).despawn();
        }
    }
}

fn turn_off(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    q: Query<Entity, With<OnGameplayScreen>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Title);
        for entity in q.iter() {
            cmd.entity(entity).despawn();
        }
    }
}
