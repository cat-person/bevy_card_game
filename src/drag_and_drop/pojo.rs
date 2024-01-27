use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Pickable {
    
}

impl Pickable {
    fn new() -> Self { Self { } }
}

