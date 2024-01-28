use bevy::{
    ecs::{ entity::Entity, event::Event },
    math::{Ray, Vec3},
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
pub struct Grab {
    pub entity: Entity,
    pub origin: Vec3
}
