use bevy::prelude::*;
use line_plugin::{LinePlugin, CastRayEvent, pickable::Pickable};
use stl_loader_plugin::StlLoaderPlugin;

mod line_plugin;
mod stl_loader_plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StlLoaderPlugin, LinePlugin))
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(Update, cusrsor_movement_handler)
        .run();
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn(( PbrBundle {
            mesh: asset_server.load("card.stl"),
            material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
            ..Default::default()
        }, Pickable::default()));
}

fn cusrsor_movement_handler(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut mouse_moution_evr: EventReader<CursorMoved>,
    mut raycast_event_writer: EventWriter<CastRayEvent>
) {
    for ev in mouse_moution_evr.read() {
        let (camera, camera_transform) = camera_q.single();
        
        let Some(ray) = camera.viewport_to_world(camera_transform, ev.position) else {
            return;
        };
        raycast_event_writer.send(CastRayEvent::new(ray));
    }

    
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -0.5).looking_at(Vec3::ZERO, Vec3::NEG_Y),
        ..default()
    });

    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(0.0, 0.0, -1.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Y),
        spot_light: SpotLight {
            intensity: 40.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::WHITE,
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.8,
            ..default()
        },
        ..default()
    });
}
