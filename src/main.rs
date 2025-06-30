mod model;

use bevy::{prelude::*, window::PrimaryWindow};
use model::Node;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Syna".into(),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let window = window_query.single().unwrap();

    for i in 0..1 {
        // commands.spawn((
        //     Mesh2d(meshes.add(Rectangle::new(i as f32, i as f32))),
        //     MeshMaterial2d(materials.add(Color::WHITE)),
        //     Transform::from_xyz(i as f32 * 4., i as f32 * 4., 0.0),
        //     Node {},
        // ));
    }
}
