use crate::Material;
use bevy::{
    render::render_resource::{
            AsBindGroup,   ShaderRef
        }, 
    reflect::{Reflect, std_traits::ReflectDefault}, 
    asset::Asset,
};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct LineMaterial {

}

impl Default for LineMaterial {
    fn default() -> Self {
        LineMaterial {}
    }
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/color_material.wgsl".into()
    }
}

