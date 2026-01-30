use bevy::prelude::*;

pub(crate) mod asset_loader;
pub(crate) mod state_manager;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((asset_loader::plugin, state_manager::plugin));
    app.configure_sets(Update, (AppSystems::TickTimers, AppSystems::Update).chain());
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(crate) enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
