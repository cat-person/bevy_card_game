pub mod events;
pub mod pojo;

use bevy::{ecs::entity, prelude::*, transform};
use bevy_mod_raycast::prelude::*;

use self::{events::{Grab, Drag, Drop}, pojo::{Draggable, Grabbed, HighlightedDraggable, Dropped}};


#[derive(Default)]
pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Grab>()
            .add_event::<Drag>()
            .add_event::<Drop>()
            .add_systems(Update, (handle_cast_ray, handle_grab, handle_drag, handle_drop));
    }
}

fn handle_cast_ray(
    mut commands: Commands,
    cursor_ray: Res<CursorRay>, 
    mut raycast: Raycast,
    draggable_q: Query<Entity, With<Draggable>>,) {
    if let Some(cursor_ray) = **cursor_ray {
        // raycast.debug_cast_ray(cursor_ray, &default(), &mut gizmos);
        let entities = raycast.cast_ray(cursor_ray, &default());

        for entity in draggable_q.iter() {
            if entities.iter().any(|(e, _)| *e == entity) {
                commands.entity(entity).insert(HighlightedDraggable {});
            } else {
                commands.entity(entity).remove::<HighlightedDraggable>();
            }
        }
    }
}

fn handle_grab(mut commands: Commands,
    mut er_grab: EventReader<Grab> ) {
    if let Some(e_drop) = er_grab.read().last() {
        commands.entity(e_drop.entity).remove::<HighlightedDraggable>()
            .insert(Grabbed { origin: e_drop.origin });

        println!("{:?} Grabbed", e_drop.entity);
    }
}

fn handle_drag(mut commands: Commands,
    cursor_ray: Res<CursorRay>, 
    meshes: ResMut<Assets<Mesh>>,
    mut q_grabbed: Query<(&Handle<Mesh>, &mut Transform), With<Grabbed>>,
    mut er_drag: EventReader<Drag> ) {
    if let Some(e_drag) = er_drag.read().last() {

        if let (mesh, mut transform) = q_grabbed.single_mut(){
            if let Some(cursor_ray) = cursor_ray.0 {
                if let Some(intersection) = ray_intersection_over_mesh(
                meshes.get(mesh).unwrap(),
        &transform.compute_matrix(), 
                        &cursor_ray, 
                        Backfaces::Include) { // The hell is Backfaces ?
                    

                    transform.translation = intersection.position();
                    println!("MEOW");

                }
                // let entities = raycast.cast_ray(cursor_ray, &default());
            }
        }

        // println!("{:?} Dragged cursor_position: {:?}", e_drag.entity, e_drag.cursor_position);
    }
}

fn handle_drop(mut commands: Commands,
    mut er_drop: EventReader<Drop> ) {
    if let Some(e_drop) = er_drop.read().last() {
        commands.entity(e_drop.entity)
            .remove::<Grabbed>()
            .insert(Dropped { destination: e_drop.origin });

        println!("{:?} Dropped", e_drop.entity);
    }
}