use bevy::prelude::*;

use crate::{
    consts::{GROUND_LEVEL, GROUND_WIDTH},
    engine::state_manager::{GameState, OnGameplayScreen},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_ground);
}
fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(GROUND_WIDTH, 5.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.5))),
        Transform::from_translation(Vec3::Y * GROUND_LEVEL),
        OnGameplayScreen,
    ));
}
