use bevy::{
    ecs::{ entity::Entity, event::Event },
    math::{Ray, Vec2, Vec3},
};

#[derive(Event)]
pub struct CastRayEvent {
    ray: Ray,
}

#[derive(Event)]
pub struct Grab {
    pub entity: Entity,
    pub origin: Vec3
}

#[derive(Event)]
pub struct Drag {
    pub entity: Entity,
    pub cursor_position: Vec2,
    pub origin: Vec3
}

#[derive(Event)]
pub struct Drop {
    pub entity: Entity,
    pub origin: Vec3
}
