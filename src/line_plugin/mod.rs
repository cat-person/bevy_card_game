use std::fmt::Display;

use bevy::{
    app::{App, Plugin, Update, Startup},
    asset::{AssetApp, Assets, Handle},
    ecs::{
        event::{EventReader, Event},
        system::{Commands, ResMut, Query}, component::Component,
    },

    render::{mesh::Mesh, render_resource::PrimitiveTopology}, pbr::{MaterialMeshBundle, MaterialPlugin}, transform::components::Transform, utils::default, math::{Vec3, Ray},
};

// use bvh::{aabb::{AABB, Bounded}, Point3, Vector3};
// use bvh::bounding_hierarchy::BHShape;
// use bvh::bvh::BVH;
// use bvh::ray::Ray as BvhRay;

use self::{material::LineMaterial, ray_mesh::CameraRay, pickable::Pickable};

mod material;
pub mod pickable;
mod ray_mesh;

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



impl From<Line> for Mesh {
    fn from(line: Line) -> Self {
        Mesh::new(PrimitiveTopology::LineStrip).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![line.start, line.end],
        )
    }
}

fn cast_ray(
    mut cast_ray_event_reader: EventReader<CastRayEvent>,
    mut pickable_query: Query<(&Pickable, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    // let bvh = BVH::build(&mut );
    // let hit_sphere_aabbs = bvh.traverse(&ray, &spheres);

    for ev in cast_ray_event_reader.read() {
        // println!("cast_ray_event {coords}", coords = ev.mouse_coordinates);
        // ray_query.get_single_mut().unwrap().1.look_at(ev.ray.direction, Vec3::NEG_Z);

        // let bvh = BVH::build(&mut shapes);
        // let aaa = bvh.traverse(&BvhRay::new(
        //     Point3 { x: 0., y: 0., z: 0.  }, 
        //     Vector3 { x: 0., y: 0.0, z: -1. }), &shapes);

        // for aa in aaa.iter() {
        //     println!("{}", aa.node_index);
        // }
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