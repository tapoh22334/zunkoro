use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::BBSize;
use crate::constants;
use crate::ev_save_load_world::Derrived;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct PolygonalShape {
    pub polygon: Vec<Vec2>,
    pub position: Vec2,
    pub rotation: Quat,
    pub scale: f32,
}

impl Default for PolygonalShape {
    fn default() -> Self {
        Self {
            polygon: vec![],
            position: Vec2::ZERO,
            rotation: Quat::default(),
            scale: 1.0,
        }
    }
}

#[derive(Bundle)]
pub struct PolygonalShapeBundle {
    polygonal_shape: PolygonalShape,
    collider: Collider,
    restitution: Restitution,
    friction: Friction,
    color: Fill,
    stroke: Stroke,
    bbsize: BBSize,
    velocity: Velocity,
    rigid_body: RigidBody,
    mass_property: ColliderMassProperties,
    #[bundle]
    shape_bundle: ShapeBundle,
}

impl Default for PolygonalShapeBundle {
    fn default() -> Self {
        let polygon = shapes::Polygon {
            closed: false,
            ..default()
        };

        Self {
            polygonal_shape: PolygonalShape {
                ..default()
            },
            collider: Collider::default(),
            restitution: Restitution::coefficient(constants::C_MAP_RESTITUTION),
            friction: Friction::coefficient(constants::C_MAP_FRICTION),
            color: Fill::color(Color::BLACK),
            stroke: Stroke::new(Color::BLACK, 1.0),
            bbsize: BBSize{x: 0.0, y: 0.0},
            velocity: Velocity::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            mass_property: ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec2::new(0.0, 0.0),
                mass: constants::C_DEFAULT_MASS,
                principal_inertia: constants::C_DEFAULT_INERTIA,
            }),
            shape_bundle: ShapeBundle{
                path: GeometryBuilder::build_as(&polygon),
                transform: Transform {
                    scale: Vec3::ONE,
                    ..default()
                },
                ..default()
            },

        }
    }
}


fn bounding_box(polyline: &Vec<Vec2>) -> Vec2 {
    let mut left_bottom = Vec2::new(std::f32::MAX, std::f32::MAX);
    let mut right_top = Vec2::new(std::f32::MIN, std::f32::MIN);

    for point in polyline {
        if point.x < left_bottom.x {
            left_bottom.x = point.x;
        }

        if point.x > right_top.x {
            right_top.x = point.x;
        }

        if point.y < left_bottom.y {
            left_bottom.y = point.y;
        }

        if point.y > right_top.y {
            right_top.y = point.y;
        }
    }

    let bbsize = right_top - left_bottom;

    return bbsize;
}


impl From<(Vec3, Quat, Vec3, PolygonalShape)> for PolygonalShapeBundle {
    fn from(tuple: (Vec3, Quat, Vec3, PolygonalShape)) -> Self {
        let (t, r, s, ps) = tuple;

        let polyline = ps.polygon.clone();
        let bbsize = bounding_box(&polyline);

        let polygon = shapes::Polygon {points: polyline.clone(), closed: false};
        let collider = Collider::polyline(polyline.clone(), None);

        Self {
            polygonal_shape: ps,
            collider,
            bbsize: BBSize {
                x: bbsize.x * s.x, y: bbsize.y * s.y
            },
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&polygon),
                transform: Transform {
                    translation: t,
                    rotation: r,
                    scale: s,
                },
                ..default()
            },
            ..default()
        }
    }
}


const FILE_NAME: &str = "/polygonal_shape.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, PolygonalShape)>
                = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, ps) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(PolygonalShapeBundle::from((t, r, s, ps)));
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &PolygonalShape), Without<Derrived>>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, PolygonalShape)> = vec![];

        for (e, t, ps) in q.iter() {
            let mut e = e.clone();
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, ps.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}
