use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*, window::PrimaryWindow};
use bevy_mod_raycast::prelude::*;
use drag_and_drop::{events::{Drag, Grab, Drop}, pojo::{Draggable, Grabbed, HighlightedDraggable}, DragAndDropPlugin};
use stl_loader_plugin::StlLoaderPlugin;

mod stl_loader_plugin;
mod drag_and_drop;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .add_plugins((DefaultRaycastingPlugin, StlLoaderPlugin, DragAndDropPlugin))
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, (cast_ray, handle_mouse_input, handle_mouse_motion))
        .run();
}

fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<StandardMaterial>>) {

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
        }, Draggable::default()));

    commands.spawn((PbrBundle {
        mesh: asset_server.load("card.stl"),
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..default()
        }),
        transform: Transform::from_xyz(0.2, 0.0, -1.0),
        ..Default::default()
    }, Draggable::default()));

    commands.spawn((PbrBundle {
        mesh: asset_server.load("card.stl"),
        
        material: materials.add(StandardMaterial {
            emissive: Color::rgb_linear(5.32, 2.0, 13.99),
            ..default()
        }),
        transform: Transform::from_xyz(-0.2, 0.0, -1.0),
        ..Default::default()
    }, Draggable::default()));
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
    pickable_entities_q: Query<(&Handle<StandardMaterial>, Option<&HighlightedDraggable>), With<Draggable>>,
    mut materials: ResMut<Assets<StandardMaterial>> ) {

    for (color_handle, highlighted) in pickable_entities_q.iter(){
        let color = materials.get_mut(color_handle).unwrap();
        if let Some(_) = highlighted {
            color.base_color = Color::BLACK;
            color.emissive = Color::PURPLE;
        } else {
            color.base_color = Color::BLACK;
            color.emissive = Color::BLACK;    
        }
        
    }
}

fn handle_mouse_input(
    buttons: Res<Input<MouseButton>>,
    q_highlighted_draggable: Query<(Entity, &Transform), With<HighlightedDraggable>>,
    q_grabbed: Query<(Entity, &Transform), With<Grabbed>>,
    mut ew_grab: EventWriter<Grab>,
    mut ew_drop: EventWriter<Drop>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if !q_highlighted_draggable.is_empty() {
            let (entity, transform) = q_highlighted_draggable.single();
            ew_grab.send(Grab {
                entity,
                origin: transform.translation
            })
        }
    }
    if buttons.just_released(MouseButton::Left) {
        if !q_grabbed.is_empty() {
            let (entity, transform) = q_grabbed.single();
            ew_drop.send(Drop { entity: entity, origin: transform.translation })
        }
    }
}


fn handle_mouse_motion(
    mut evw_drag: EventWriter<Drag>,
    q_grabbed: Query<(Entity, &Grabbed)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if q_grabbed.is_empty() {
        return;
    }

    let (entity, grabbed) = q_grabbed.single();
    if let Some(cursor_position) = q_windows.single().cursor_position() {
        evw_drag.send(Drag{
            entity,
            cursor_position,
            origin: grabbed.origin,
        })
    } else {
        println!("Cursor is not in the game window.");
    }    
}