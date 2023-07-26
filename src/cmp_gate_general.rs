use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use rand::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball;

const BALL_SIZE: f32 = 10.0;
pub const DEFAULT_SIZE_X: f32 = 64.0;
pub const DEFAULT_SIZE_Y: f32 = 64.0;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateGeneral {
    pub remain: i32,
    pub prob: f32,
    pub ball_radius: f32,
}


#[derive(Bundle)]
pub struct GateGeneralBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    bbsize: BBSize,
    gate_general: GateGeneral,
}


impl Default for GateGeneralBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                transform: Transform::from_translation(gate_general.position),
                ..Default::default()
            },
            bbsize: BBSize {x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            gate_general: GateGeneral {
                remain: 1,
                prob: 1.0,
                ball_radius: BALL_SIZE,
            }

        }
    }
}


impl From<(Vec3, Quat, Vec3, GateGeneral)> for GateGeneralBundle {
    fn from(tuple: (Vec3, Quat, Vec3, GateGeneral)) -> Self {
        let (translation, rotation, scale, gate_general) = tuple;

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..default()
            },
            gate_general,
            ..default()
        }
    }
}


pub fn system_setup(
    mut commands: Commands,
    mut query: Query<(&mut Sprite, &GateGeneral)>,
    ) {

    for (mut sprite, gate_general) in query.iter_mut() {
        let color = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0, };
        sprite.color = color;
    }

}

pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut query: Query<(Entity, &Transform, &BBSize, &mut GateGeneral)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, transform, bbsize, mut gate_general) in query.iter_mut() {
        if gate_general.remain > 0 {
            if rng.gen::<f32>() < gate_general.prob {
                let size = Vec2::new(bbsize.x, bbsize.y) * transform.scale.truncate();
                let pos_max = transform.translation.truncate() + (size / 2.0);
                let pos_min = transform.translation.truncate() - (size / 2.0);

                let x = rng.gen_range(pos_min.x .. pos_max.x);
                let y = rng.gen_range(pos_min.y .. pos_max.y);

                cmp_ball::add(&mut commands, &game_assets, Vec2::new(x, y), gate_general.ball_radius, Vec2::new(0.0, 0.0));
                gate_general.remain -= 1;
            }
        } else {
            commands.get_entity(entity).unwrap().despawn();
        }
    }
}


const FILE_NAME: &str = "/gate_general.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, GateGeneral)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, gg) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(GateGeneralBundle::from((t, r, s, ps)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &GateGeneral)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, GateGeneral)> = vec![];

        for (e, t, gg) in q.iter() {
            let mut e = e.clone();

            elem_list.push((e.index(), t.translation, t.rotation, t.scale, gg));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

