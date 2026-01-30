use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    engine::{
        asset_loader::ImageAssets,
        state_manager::{GameState, OnGameplayScreen},
    },
    player::{
        animation::{AnimationState, PlayerAnimation},
        controller::{Jump, Movement},
    },
};

mod animation;
mod controller;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, controller::plugin))
        .add_systems(OnEnter(GameState::InGame), setup_character);
}

#[derive(Component)]
#[require(AnimationState)]
pub struct Character;

#[derive(Component, Default)]
pub struct CharacterPhysics {
    pub velocity: Vec2,
    pub is_grounded: bool,
}

fn setup_character(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    img_assets: Res<ImageAssets>,
) {
    let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 128),
        12,
        3,
        None,
        None,
    ));
    let player_animation = PlayerAnimation::new();
    commands.spawn((
        Character,
        OnGameplayScreen,
        Sprite::from_atlas_image(
            img_assets.character.clone(),
            TextureAtlas {
                layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        player_animation,
        Transform::from_translation(Vec3::Y * (300.0)),
        CharacterPhysics::default(),
        actions!(Character[
            (
                Action::<Movement>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(128.0),
                Bindings::spawn((
                    Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                    Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                    Axial::left_stick(),
                )),
            ),
            (
                Action::<Jump>::new(),
                bindings![KeyCode::Space, GamepadButton::South],
            )
        ]),
    ));
}
