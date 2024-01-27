//! This example will show you how to use your mouse cursor as a ray casting source, cast into the
//! scene, intersect a mesh, and mark the intersection with the built in debug cursor. If you are
//! looking for a more fully-featured mouse picking plugin, try out bevy_mod_picking.

use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use drag_and_drop::{events::{CastRayEvent, CastResult}, DragAndDropPlugin};
use stl_loader_plugin::StlLoaderPlugin;

mod stl_loader_plugin;
mod drag_and_drop;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .add_plugins((DefaultRaycastingPlugin, StlLoaderPlugin, DragAndDropPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, cast_ray)
        .run();
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn(Camera3dBundle::default());
    commands.spawn(PointLightBundle::default());

    commands.spawn(PbrBundle {
            mesh: asset_server.load("card.stl"),
            material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..Default::default()
        });
}

fn cast_ray(
    mut ray_result_event_reader: EventReader<CastResult>) {

    for cast_event in ray_result_event_reader.read() {
        println!("{:?}", cast_event.entity)
    }
}