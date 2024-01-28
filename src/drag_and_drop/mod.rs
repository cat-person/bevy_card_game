pub mod events;
pub mod pojo;

use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use self::{events::Grab, pojo::{Draggable, Dragged, HighlightedDraggable}};


#[derive(Default)]
pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Grab>()
            .add_systems(Update, (cast_ray, handle_grab));
    }
}

fn cast_ray(
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
    mut grab_er: EventReader<Grab> ) {
    if let Some(grab_e) = grab_er.read().last() {
        commands.entity(grab_e.entity).remove::<HighlightedDraggable>()
            .insert(Dragged { origin: grab_e.origin });

        println!("{:?} grabbed", grab_e.entity);
    }
}