use bevy::ecs::component::Component;

use super::CastShape;

#[derive(Component, Default)]
pub struct Pickable {
    pub cast_shape: CastShape
}

impl Pickable {
    fn new(cast_shape: CastShape) -> Self { Self { cast_shape } }
}

pub struct PickCandidate {
    
}

