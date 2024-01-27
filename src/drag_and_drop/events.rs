use bevy::{
    ecs::{ entity::Entity, event::Event },
    math::Ray,
};

#[derive(Event)]
pub struct CastRayEvent {
    ray: Ray,
}

impl CastRayEvent {
    pub fn new(ray: Ray) -> Self {
        Self { ray }
    }
}

#[derive(Event)]
pub struct CastResult {
    pub entity: Entity,
}

// impl CastResult {
//     pub fn new(entities: [(Entity, IntersectionData)]) -> Self {
//         Self { entities }
//     }
// }
