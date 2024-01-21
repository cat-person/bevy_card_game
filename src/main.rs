//! Create a custom material to draw basic lines in 3D

mod color_material_3d;
use color_material_3d::ColorMaterial3dPlugin;
use color_material_3d::ColorMaterial3d;

use bevy::{
    prelude::*,
    render::mesh::PrimitiveTopology,
};



fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ColorMaterial3dPlugin))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            // This tells wgpu that the positions are list of lines
            // where every pair is a start and end point
            PrimitiveTopology::LineList,
            // RenderAssetPersistencePolicy::Unload,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial3d>>,
) {
    // Spawn a line strip that goes from point to point
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineStrip {
            points: vec![
                Vec3::ZERO,
                Vec3::new(1.0, 1.0, 0.0),
            ],
        })),
        material: materials.add(ColorMaterial3d{}),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// A list of lines with a start and end position

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
            // This tells wgpu that the positions are a list of points
            // where a line will be drawn between each consecutive point
            PrimitiveTopology::LineStrip,
            // RenderAssetPersistencePolicy::Unload,
        )
        // Add the point positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}