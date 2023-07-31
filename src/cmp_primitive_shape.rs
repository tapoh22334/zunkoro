use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::BBSize;
use crate::constants;
use crate::ev_save_load_world::Derrived;
use crate::edit_context::*;


#[derive(Default, Component, Reflect, FromReflect, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Shape {
    #[default]
    SBox,
    SCircle,
    SDia,
    SStar,
    STriangle,
}

#[derive(Default, Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct PrimitiveShape {
    pub shape: Shape,
}

#[derive(Bundle)]
pub struct PrimitiveShapeBundle {
    primitive_shape: PrimitiveShape,
    collider: Collider,
    restitution: Restitution,
    friction: Friction,
    color: Fill,
    stroke: Stroke,
    bbsize: BBSize,
    velocity: Velocity,
    rigid_body: RigidBody,
    mass_property: ColliderMassProperties,
    map_object: MapObject,
    #[bundle]
    shape_bundle: ShapeBundle,
}

pub const DEFAULT_SIZE_X: f32 = 64.0;
pub const DEFAULT_SIZE_Y: f32 = 64.0;

impl Default for PrimitiveShapeBundle {
    fn default() -> Self {
        let shape = Shape::SBox;
        let polygon = shapes::Polygon {
            points: load_shape_polyline(shape),
            closed: false
        };

        Self {
            primitive_shape: PrimitiveShape {
                ..default()
            },
            collider: Collider::polyline(load_shape_polyline(shape), None),
            restitution: Restitution::coefficient(constants::C_PRIMITIVE_SHAPE_RESTITUTION),
            friction: Friction::coefficient(constants::C_PRIMITIVE_SHAPE_FRICTION),
            color: Fill::color(Color::BLACK),
            stroke: Stroke::new(Color::BLACK, 1.0),
            bbsize: BBSize{x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            velocity: Velocity::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            //rigid_body: RigidBody::Dynamic,
            //mass_property: ColliderMassProperties::Mass(1.0), 
            mass_property: ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec2::new(0.0, 0.0),
                mass: constants::C_DEFAULT_MASS,
                principal_inertia: constants::C_DEFAULT_INERTIA,
            }),
            map_object: MapObject::PrimitiveShape(Shape::SBox),
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

impl From<(Vec3, Quat, Vec3, PrimitiveShape)> for PrimitiveShapeBundle {
    fn from(tuple: (Vec3, Quat, Vec3, PrimitiveShape)) -> Self {
        let (translation, rotation, scale, primitive_shape) = tuple;

        let polyline = load_shape_polyline(primitive_shape.shape.clone());
        let polygon = shapes::Polygon {points: polyline.clone(), closed: false};

        Self {
            primitive_shape: primitive_shape.clone(),
            collider: Collider::polyline(polyline, None),
            bbsize: BBSize {
                x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y
            },
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&polygon),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..default()
            },
            map_object: MapObject::PrimitiveShape(primitive_shape.shape),
            ..default()
        }
    }
}


fn load_shape_polyline(s: Shape) -> Vec<Vec2> {
    let map_file = match s {
        Shape::SBox => {
            include_bytes!("../assets/map_element/primitive_64_box.map").as_slice()
        }
        Shape::SCircle => {
            include_bytes!("../assets/map_element/primitive_64_circle.map").as_slice()
        }
        Shape::SDia => {
            include_bytes!("../assets/map_element/primitive_64_dia.map").as_slice()
        }
        Shape::SStar => {
            include_bytes!("../assets/map_element/primitive_64_star.map").as_slice()
        }
        Shape::STriangle => {
            include_bytes!("../assets/map_element/primitive_64_triangle.map").as_slice()
        }
    };

    let file_contents = String::from_utf8_lossy(map_file);
    let map: Vec<Vec<Vec2>> = serde_json::from_str(&file_contents).unwrap();

    return map.iter().next().unwrap().clone();
}


const FILE_NAME: &str = "/primitive_shape.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, PrimitiveShape)>
                = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, ps) in elem_list {
                println!("{:?}", i);
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(PrimitiveShapeBundle::from((t, r, s, ps)));
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &PrimitiveShape), Without<Derrived>>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, PrimitiveShape)> = vec![];

        for (e, t, ps) in q.iter() {
            let mut e = e.clone();
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, ps.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}
