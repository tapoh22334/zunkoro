use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::BBSize;
use crate::constants;

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
    pub position: Vec2,
    pub scale: f32,
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
    #[bundle]
    shape_bundle: ShapeBundle,
}

const DEFAULT_SIZE_X: f32 = 64.0;
const DEFAULT_SIZE_Y: f32 = 64.0;

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
            restitution: Restitution::coefficient(constants::C_MAP_RESTITUTION),
            friction: Friction::coefficient(constants::C_MAP_FRICTION),
            color: Fill::color(Color::BLACK),
            stroke: Stroke::new(Color::BLACK, 1.0),
            bbsize: BBSize{x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            shape_bundle: ShapeBundle{
                path: GeometryBuilder::build_as(&polygon),
                transform: Transform {
                    ..default()
                },
                ..default()
            },

        }
    }
}

impl From<PrimitiveShape> for PrimitiveShapeBundle {
    fn from(primitive_shape: PrimitiveShape) -> Self {
        println!("from primitive shape");
        let shape = primitive_shape.shape.clone();
        let position = primitive_shape.position.clone();
        let scale = primitive_shape.scale.clone();
        let polyline = load_shape_polyline(shape);
        let polygon = shapes::Polygon {points: polyline.clone(), closed: false};

        Self {
            primitive_shape,
            collider: Collider::polyline(polyline, None),
            bbsize: BBSize {
                x: DEFAULT_SIZE_X * scale, y: DEFAULT_SIZE_Y * scale
            },
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&polygon),
                transform: Transform {
                    translation: Vec3::from((position, 0.0)),
                    scale: Vec3::ONE * scale,
                    ..default()
                },
                ..default()
            },
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
            let elem_list: Vec<PrimitiveShape> = serde_json::from_str(&json_str).unwrap();

            for e in elem_list {
                commands.spawn(PrimitiveShapeBundle::from(e));
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &PrimitiveShape)>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<PrimitiveShape> = vec![];

        for (t, e) in q.iter() {
            let mut e = e.clone();
            e.position = t.translation.truncate();
            e.scale = t.scale.truncate().x;
            elem_list.push(e.clone());
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

