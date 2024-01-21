use crate::{Material, MaterialPlugin};
use bevy::{
    render::render_resource::{
            AsBindGroup,   ShaderRef
        }, 
    reflect::{Reflect, std_traits::ReflectDefault}, 
    asset::{Asset, Handle, Assets, AssetApp}, app::{App, Plugin}, math::{Vec4, Vec3},
};

#[derive(Default)]
pub struct ColorMaterial3dPlugin;

impl Plugin for ColorMaterial3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ColorMaterial3d>::default())
            .register_asset_reflect::<ColorMaterial3d>();

        app.world.resource_mut::<Assets<ColorMaterial3d>>().insert(
            Handle::<ColorMaterial3d>::default(),
            ColorMaterial3d::default(),
        );
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct ColorMaterial3d {
}

impl Default for ColorMaterial3d {
    fn default() -> Self {
        ColorMaterial3d {}
    }
}

impl Material for ColorMaterial3d {
    fn fragment_shader() -> ShaderRef {
        "shaders/color_material.wgsl".into()
    }
}


