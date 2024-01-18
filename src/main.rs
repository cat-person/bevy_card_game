use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, prelude::{On, ListenerInput}, events::{Pointer, Click, Drag, Over, Down}, pointer::PointerButton};
use stl_loader_plugin::StlLoaderPlugin;

mod stl_loader_plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StlLoaderPlugin))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, keyboard_input)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(
    DirectionalLightBundle{
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 50000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 10.0,1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let card_bundle = PbrBundle {
        mesh: asset_server.load("card.stl"),
        material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(-0.3, 0.0, 0.25))
            .with_rotation(Quat::from_rotation_x(- PI / 2.0)),
        ..Default::default()
    };

    commands.spawn((
        card_bundle, 
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            println!("{}", drag.delta);
            transform.translation.x += drag.delta.x / 1500.0;
            transform.translation.z += drag.delta.y / 1000.0;
        })
    ));
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>
) {
    // we can check multiple at once with `.any_*`
    if keys.pressed(KeyCode::W) {
        
    } else if keys.pressed(KeyCode::S) {
        
    } else if keys.pressed(KeyCode::A) {
        
    } else if keys.pressed(KeyCode::D) {
        
    }
}
