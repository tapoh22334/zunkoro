use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Default, Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Vibrator {
    pub direction: Direction,
    pub speed: f32,
    pub range: (f32, f32),
}

pub fn system(
    mut q: Query<(&mut Transform, &mut Velocity, &Vibrator)>,
) {
    for (t, mut v, vb) in q.iter_mut() {
        match vb.direction {
            Direction::Vertical => {
                if t.translation.y <= vb.range.0 {
                    v.linvel.y = vb.speed;
                } else if t.translation.y >= vb.range.1 {
                    v.linvel.y = -vb.speed;
                } else if v.linvel.y == 0.0 {
                    v.linvel.y = vb.speed;
                }
            }

            Direction::Horizontal => {
                if t.translation.x <= vb.range.0 {
                    v.linvel.x = vb.speed;
                } else if t.translation.x >= vb.range.1 {
                    v.linvel.x = -vb.speed;
                } else if v.linvel.x == 0.0 {
                    v.linvel.x = vb.speed;
                }

            }

        }

    }
}

const FILE_NAME: &str = "/vibrating_shape.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    )
{

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let filename = dir + FILE_NAME;
        let json_str = std::fs::read_to_string(filename);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vibrator)> = serde_json::from_str(&json_str).unwrap();

            for (id, v) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(id));
                entity.insert(v);
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Vibrator)>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vibrator)> = vec![];

        for (e, vi) in q.iter() {
            elem_list.push((e.index(), vi.clone()));
        }

        let filename = dir + FILE_NAME;
        std::fs::write(filename, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

