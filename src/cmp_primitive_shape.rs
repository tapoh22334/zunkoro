use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::BBSize;
use crate::constants;

#[derive(Component, Reflect, FromReflect, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Shape {
    SBox,
    SCircle,
    SDia,
    SStar,
    STriangle,
}

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct PrimitiveShape {
    pub shape: Shape,
    pub position: Vec2,
    pub scale: f32,
}

pub fn load_shape_polyline(s: Shape) -> Vec<Vec<Vec2>> {
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

    return map[0];
}


pub fn add(commands: &mut Commands, primitive_shape: PrimitiveShape) -> Entity {
    let shape = primitive_shape.shape.clone();
    let position = primitive_shape.position.clone();
    let scale = primitive_shape.scale.clone();

    let polyline = load_shape_polyline(shape);

    let shape = shapes::Polygon {points: polyline.clone(), closed: false};

    let mut entity = commands.spawn(primitive_shape);
    entity
        .insert(Collider::polyline(polyline, None))
        .insert(Restitution::coefficient(constants::C_MAP_RESTITUTION))
        .insert(Friction::coefficient(constants::C_MAP_FRICTION))
        .insert(ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform {
                translation: Vec3::from((position, 0.0)),
                scale: Vec3::ONE * scale,
                ..default()
            },
            ..default()
        })
        .insert(Fill::color(Color::BLACK))
        .insert(Stroke::new(Color::BLACK, 1.0))
        .insert(BBSize{x: 64.0 * scale, y: 64.0 * scale})
    ;

    return entity.id();
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
                add(&mut commands, e);
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

