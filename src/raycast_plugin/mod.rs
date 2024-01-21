use bevy::{ecs::system::{Commands, ResMut}, pbr::{MaterialMeshBundle, Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin}, math::Vec3, transform::components::Transform, render::{color::Color, mesh::{Mesh, MeshVertexBufferLayout}, render_resource::{ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError, PolygonMode, PrimitiveTopology, AsBindGroup}}, asset::{Assets, Asset}, app::{Plugin, App, Startup}, reflect::TypePath, utils::default};



pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(MaterialPlugin::<LineMaterial>::default())
            .add_systems(Startup, setup_ray);
	}
}

fn setup_ray (mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>) {
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(LineList {
                lines: vec![
                    (Vec3::ZERO, Vec3::new(1.0, 1.0, 0.0)),
                    (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
                ],
            })),
            transform: Transform::from_xyz(-1.5, 0.0, 0.0),
            material: materials.add(LineMaterial {
                color: Color::GREEN,
            }),
            ..default()
        });
    
        // Spawn a line strip that goes from point to point
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(LineStrip {
                points: vec![
                    Vec3::ZERO,
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ],
            })),
            transform: Transform::from_xyz(0.5, 0.0, 0.0),
            material: materials.add(LineMaterial { color: Color::BLUE }),
            ..default()
        });
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        Mesh::new(PrimitiveTopology::LineList)
            // Add the vertices positions as an attribute
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        Mesh::new(PrimitiveTopology::LineStrip)
            // Add the point positions as an attribute
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}