use bevy::prelude::*;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .run();
}
