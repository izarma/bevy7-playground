use bevy::prelude::*;

pub(crate) mod engine;
mod player;
mod ui;
pub(crate) mod world;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        engine::plugin,
        ui::plugin,
        player::plugin,
        world::plugin,
    ))
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
    .add_systems(Startup, setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    let main_camera = Camera2d::default();
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::AutoMin {
            min_width: (1020.0),
            min_height: (720.0),
        },
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((Name::new("Camera"), main_camera, projection));
}
