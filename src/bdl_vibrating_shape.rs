use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cmp_primitive_shape;
use crate::cmp_vibrator;
use crate::constants;

#[derive(Default, Bundle, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct VibratingShape {
    pub vibrator: Vibrator,
    pub primitive_shape: PrimitiveShape,
    pub polyline: Collider::polyline,
    pub restitution: Restitution::coefficient,
    pub friction: Friction::coefficient,
    pub fill: Fill,
    pub stroke: Stroke,
    pub bbsize: BBSize,

    #[bundle]
    pub shape_bundle: ShapeBundle,
}


pub fn add(commands: &mut Commands, vibrating_shape: VibratingShape) -> Entity {
    let mut entity = commands.spawn(vibrating_shape);

    return entity.id();
}


const FILE_NAME: &str = "/vibrating_shape.map";
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

