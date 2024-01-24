//! Create a custom material to draw basic lines in 3D

// mod color_material_3d;
// use color_material_3d::ColorMaterial3dPlugin;
// use color_material_3d::ColorMaterial3d;

use bevy::prelude::*;
use line_plugin::{LinePlugin, CastRayEvent};

mod line_plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LinePlugin))
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(Update, cusrsor_movement_handler)
        .run();
}

fn setup() {
}

fn cusrsor_movement_handler(
    mut window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut mouse_moution_evr: EventReader<CursorMoved>,
    mut raycast_event_writer: EventWriter<CastRayEvent>
) {
    for ev in mouse_moution_evr.read() {
        let window = window_q.get_single_mut().unwrap();

        let x = ev.position.x - window.width() / 2.;
        let y = ev.position.y - window.height() / 2.;
        
        let (camera, camera_transform) = camera_q.single();
        
        let Some(ray) = camera.viewport_to_world(camera_transform, ev.position) else {
            return;
        };
        // println!("Ray: origin: {origin} => direction: {direction}", origin = ray.origin, direction = ray.direction);

        raycast_event_writer.send(CastRayEvent::new(ray));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -5.0).looking_at(Vec3::ZERO, Vec3::NEG_Y),
        ..default()
    });
}
