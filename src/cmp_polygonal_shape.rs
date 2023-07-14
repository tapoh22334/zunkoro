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


impl From<PolygonalShape> for PolygonalShapeBundle {
    fn from(polygonal_shape: PolygonalShape) -> Self {
        let position = polygonal_shape.position.clone();
        let rotation = polygonal_shape.rotation.clone();
        let scale = polygonal_shape.scale.clone();
        let polyline = polygonal_shape.polygon.clone();
        let bbsize = bounding_box(&polyline);

        let polygon = shapes::Polygon {points: polyline.clone(), closed: false};
        let collider = Collider::polyline(polyline.clone(), None);

        Self {
            polygonal_shape,
            collider,
            bbsize: BBSize {
                x: bbsize.x * scale, y: bbsize.y * scale
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
            let elem_list: Vec<PolygonalShape> = serde_json::from_str(&json_str).unwrap();

            for e in elem_list {
                commands.spawn(PolygonalShapeBundle::from(e));
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &PolygonalShape), Without<Derrived>>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<PolygonalShape> = vec![];

        for (t, e) in q.iter() {
            let mut e = e.clone();
            e.position = t.translation.truncate();
            e.scale = t.scale.truncate().x;
            e.rotation = t.rotation;
            elem_list.push(e.clone());
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

