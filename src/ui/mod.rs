use bevy::prelude::*;

use crate::engine::state_manager::{GameState, OnTitleScreen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Title), setup_title);
}

fn setup_title(mut cmd: Commands) {
    cmd.spawn((
        Text2d::new("Press P to start"),
        Transform::from_xyz(0., 0., 0.),
        OnTitleScreen,
    ));
}
