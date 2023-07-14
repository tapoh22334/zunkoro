use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cmp_primitive_shape;
use crate::cmp_vibrator;
use crate::constants;

pub struct VibratingShape {
    pub vibrator: Vibrator,
    pub primitive_shape: PrimitiveShape,
}

#[derive(Default, Bundle, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct VibratingShapeBundle{
    pub vibrator: Vibrator,
    #[bundle]
    pub primitive_shape_bundle: PrimitiveShapeBundle,
}

impl Default for VibratingShapeBundle {
    fn default() -> Self {
        Self {
            vibrator: Vibrator::default(),
            primitive_shape_bundle: PrimitiveShapeBundle::Default(),
        }
    }
}

impl From<VibratingShape> for VibratingShapeBundle {
    fn from(vibrating_shape) -> Self {
        Self {
            ..default(),
        }
    }
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
            let elem_list: Vec<(Vibrator, PrimitiveShape)> = serde_json::from_str(&json_str).unwrap();

            for e in elem_list {
                commands.spawn(VibratingShapeBundle::from(()))
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &Vibrator, &PrimitiveShape)>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(Vibrator, PrimitiveShape)> = vec![];

        for (t, e1, e2) in q.iter() {
            let mut e = e.clone();
            e2.position = t.translation.truncate();
            e2.scale = t.scale.truncate().x;
            elem_list.push((e1.clone(), e2.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

