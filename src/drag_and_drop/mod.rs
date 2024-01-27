pub mod events;
pub mod pojo;

use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use self::events::CastResult;


#[derive(Default)]
pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CastResult>()
            .add_systems(Update, cast_ray);
    }
}

fn cast_ray(
    cursor_ray: Res<CursorRay>, 
    mut raycast: Raycast,
    mut ray_result_event_writer: EventWriter<CastResult>) {
    if let Some(cursor_ray) = **cursor_ray {
        // raycast.debug_cast_ray(cursor_ray, &default(), &mut gizmos);
        let entities = raycast.cast_ray(cursor_ray, 
            &default());

        if !entities.is_empty() {
            ray_result_event_writer.send(CastResult{
                entity: entities.first().unwrap().0,
            })
        }
            
    }
}
  