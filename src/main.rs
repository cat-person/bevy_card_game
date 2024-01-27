use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, ecs::entity, prelude::*};
use bevy_mod_raycast::prelude::*;
use drag_and_drop::{events::{CastRayEvent, CastResult}, pojo::Pickable, DragAndDropPlugin};
use stl_loader_plugin::StlLoaderPlugin;

mod stl_loader_plugin;
mod drag_and_drop;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .add_plugins((DefaultRaycastingPlugin, StlLoaderPlugin, DragAndDropPlugin))
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, cast_ray)
        .run();
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<StandardMaterial>>) {

    // commands.spawn((Camera3dBundle::default(), BloomSettings::default()));
    commands.spawn(PointLightBundle{ 
        point_light: PointLight {
            intensity: 0.1,
            ..default()
        },
        ..default() 
    });

    commands.spawn((PbrBundle {
            mesh: asset_server.load("card.stl"),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..Default::default()
        }, Pickable::default()));

    commands.spawn((PbrBundle {
        mesh: asset_server.load("card.stl"),
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..default()
        }),
        transform: Transform::from_xyz(0.2, 0.0, -1.0),
        ..Default::default()
    }, Pickable::default()));

    commands.spawn((PbrBundle {
        mesh: asset_server.load("card.stl"),
        
        material: materials.add(StandardMaterial {
            emissive: Color::rgb_linear(5.32, 2.0, 13.99),
            ..default()
        }),
        transform: Transform::from_xyz(-0.2, 0.0, -1.0),
        ..Default::default()
    }, Pickable::default()));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings {
            intensity: 0.1,

            ..default()
        } // 3. Enable bloom for the camera
    ));
}

fn cast_ray(
    mut ray_result_event_reader: EventReader<CastResult>,
    pickable_entities_q: Query<(Entity, &Handle<StandardMaterial>), With<Pickable>>,
    mut materials: ResMut<Assets<StandardMaterial>> ) {

    for (_, color_handle) in pickable_entities_q.iter(){
        let color = materials.get_mut(color_handle).unwrap();
        color.base_color = Color::BLACK;
        color.emissive = Color::BLACK;
    }

    for cast_event in ray_result_event_reader.read() {
        for (entity, color_handle) in pickable_entities_q.iter(){
            if(entity == cast_event.entity) {
                let color = materials.get_mut(color_handle).unwrap();
                color.base_color = Color::BEIGE;
                color.emissive = Color::GREEN;
            }            
        }    
    }
}