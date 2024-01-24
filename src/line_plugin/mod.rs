use bevy::{
    app::{App, Plugin, Update, Startup},
    asset::{AssetApp, Assets},
    ecs::{
        event::{EventReader, Event},
        system::{Commands, ResMut, Query}, component::Component,
    },

    render::{mesh::Mesh, render_resource::PrimitiveTopology}, pbr::{MaterialMeshBundle, MaterialPlugin}, transform::components::Transform, utils::default, math::{Vec3, Ray},
};

use self::material::LineMaterial;

mod material;

#[derive(Default)]
pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<LineMaterial>::default())
            .register_asset_reflect::<LineMaterial>()
            .add_event::<CastRayEvent>()
            .add_systems(Startup, create_ray)
            .add_systems(Update, cast_ray);

        // app.world.resource_mut::<Assets<Line>>().insert(
        //     Handle::<Line>::default(),
        //     Line::default(),
        // );
    }
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
}

impl Line {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }
}

#[derive(Component)]
pub struct CameraRay {
}

impl From<Line> for Mesh {
    fn from(line: Line) -> Self {
        Mesh::new(PrimitiveTopology::LineStrip).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![line.start, line.end],
        )
    }
}

#[derive(Event)]
pub struct CastRayEvent {
    ray: Ray,
}

impl CastRayEvent {
    pub fn new(ray: Ray) -> Self {
        Self { ray }
    }
}

fn cast_ray(
    mut cast_ray_event_reader: EventReader<CastRayEvent>,
    mut ray_query: Query<(&CameraRay, &mut Transform)>
){
    for ev in cast_ray_event_reader.read() {
        // println!("cast_ray_event {coords}", coords = ev.mouse_coordinates);
        ray_query.get_single_mut().unwrap().1.look_at(ev.ray.direction, Vec3::NEG_Z)
    }
}

fn create_ray(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>
) {
    commands.spawn((
        CameraRay {},
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Line {
                start: Vec3::ZERO,
                end: Vec3::new(0.0, 10.0, 0.0), 
            })),
            material: line_materials.add(LineMaterial {}),
            ..default()
        }
    ));
}
