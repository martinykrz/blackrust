use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_svg::prelude::*;

fn main() {
    App::new()
        // Set antialiasing to 4 samples
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BlackRust".into(),
                resolution: (1600., 1600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(SvgPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    asset_server: Res<AssetServer>
    ) {
    // Camara
    commands.spawn(Camera2dBundle::default());
    // Plane
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(
            Mesh::from(
                shape::Quad {
                    size: Vec2::new(1600., 1600.),
                    ..Default::default()
                })
            ).into(),
        transform: Transform::default(),
        material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
        ..default()
    });
    // Card
    let svg = asset_server.load("cards/ace_hearts.svg");
    commands.spawn(Svg2dBundle {
        svg,
        origin: Origin::Center,
        ..Default::default()
    });
}

