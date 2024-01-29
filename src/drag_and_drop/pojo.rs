use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Default)]
pub struct Draggable {
    
}

#[derive(Component, Default)]
pub struct HighlightedDraggable {

}

#[derive(Component, Default)]
pub struct Grabbed {
    pub origin: Vec3 // original coordinates
}

#[derive(Component, Default)]
pub struct Dragged {
    pub origin: Vec3
}

#[derive(Component, Default)]
pub struct Dropped {
    pub destination: Vec3
}

#[derive(Component, Default)]
pub struct DropArea {
    pub destination: Vec3 // Option<vec3> if destination is None you can drop wherever // Maybe shape ?
}

