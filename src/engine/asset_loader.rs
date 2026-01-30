use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};

use crate::engine::state_manager::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::InGame)
            .load_collection::<ImageAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "player/character.png")]
    pub character: Handle<Image>,
}
