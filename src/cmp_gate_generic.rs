use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_ball_zundamon::Zundamon;
use crate::cmp_ball_zombie::Zombie;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball_zundamon;
use crate::cmp_ball_zombie;

use crate::cmp_ball_type1;
use crate::cmp_ball_type2;
use crate::cmp_ball_type3;
use crate::cmp_ball_type4;

use crate::cmp_rotator::Rotator;
use crate::edit_context::*;

const BALL_SIZE: f32 = 20.0;
pub const DEFAULT_SIZE_X: f32 = 10.0;
pub const DEFAULT_SIZE_Y: f32 = 10.0;


#[derive(Component, Reflect, FromReflect, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum BallType {
    Zundamon,
    Zombie,
    Type1P1,
    Type1P2,
    Type2P1,
    Type2P2,
    Type3P1,
    Type3P2,
    Type4P1,
    Type4P2,
}

#[derive(Reflect, FromReflect, Clone, PartialEq, Serialize, Deserialize,Debug)]
pub struct SpawnBall(pub u32, pub BallType);

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateGeneric {
    pub remain: Vec<BallType>,
    pub prob: f32,
    pub ball_radius: f32,
}


#[derive(Bundle)]
pub struct GateGenericBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    bbsize: BBSize,
    gate_generic: GateGeneric,
    map_object: MapObject,
}


impl Default for GateGenericBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(DEFAULT_SIZE_X, DEFAULT_SIZE_Y)),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::ONE),
                ..Default::default()
            },
            bbsize: BBSize {x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            gate_generic: GateGeneric {
                remain: vec![],
                prob: 1.0,
                ball_radius: BALL_SIZE,
            },
            map_object: MapObject::GateGeneric,
        }
    }
}

impl From<Vec3> for GateGenericBundle {
    fn from(translation: Vec3) -> Self {
        let mut bundle = GateGenericBundle::default();
        bundle.sprite_bundle.transform = Transform {
                    translation,
                    scale: Vec3::ONE,
                    ..default()
                };
        bundle
    }
}

impl From<(Vec3, Quat, Vec3, GateGeneric)> for GateGenericBundle {
    fn from(tuple: (Vec3, Quat, Vec3, GateGeneric)) -> Self {
        let (translation, rotation, scale, gate_generic) = tuple;

        let mut bundle = GateGenericBundle::default();

        bundle.sprite_bundle.transform = Transform {
                    translation,
                    rotation,
                    scale,
                };
        bundle.gate_generic = gate_generic;

        bundle
    }
}


pub fn handle_user_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut edit_context: ResMut<EditContext>,
    world_position: ResMut<WorldPosition>,
    ) {
    if ! buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let EditContext::Spawn(map_object) = edit_context.clone() {
        if let MapObject::GateGeneric = map_object {

            let mut entity = commands.spawn(
                GateGenericBundle::from(Vec3::from((world_position.translation, 0.0)))
                );

            *edit_context = EditContext::Edit(MapObject::GateGeneric, vec![entity.id()], EditTool::Select);
        }
    }
}

pub fn system_setup(
    mut query: Query<&mut Sprite, With<GateGeneric>>,
    ) {

    for mut sprite in query.iter_mut() {
        let color = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0, };
        sprite.color = color;
    }

}

pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut event: EventReader<SpawnBall>,
    mut query: Query<(Entity, &Transform, &BBSize, &mut GateGeneric)>,
) {
    let game_assets = game_assets.into_inner();

    for e in event.iter() {
        let entity = Entity::from_raw(e.0);
        let balltype = e.1;

        let (_, _, _, mut gate_generic) = query.get_mut(entity).unwrap();
        gate_generic.remain.push(balltype);
    }

    let mut rng = rand::thread_rng();
    for (entity, transform, bbsize, mut gate_generic) in query.iter_mut() {
        if gate_generic.remain.len() > 0 {
            if rng.gen::<f32>() < gate_generic.prob {
                let size = Vec2::new(bbsize.x, bbsize.y) * transform.scale.truncate();
                let pos_max = transform.translation.truncate() + (size / 2.0);
                let pos_min = transform.translation.truncate() - (size / 2.0);

                let x = rng.gen_range(pos_min.x .. pos_max.x);
                let y = rng.gen_range(pos_min.y .. pos_max.y);

                let rad = gate_generic.ball_radius;
                let balltype = gate_generic.remain.pop().unwrap();

                match(balltype) {
                    BallType::Zundamon => {
                        let _ = commands.spawn(
                            cmp_ball_zundamon::BallZundamonBundle::from((Vec2::new(x, y), rad, Vec2::ZERO, game_assets)));
                    },

                    BallType::Zombie => {
                        let _ = commands.spawn(
                            cmp_ball_zombie::BallZombieBundle::from((Vec2::new(x, y), rad, Vec2::ZERO, game_assets)));
                    }

                    BallType::Type1P1 => {
                        let _ = commands.spawn(
                            cmp_ball_type1::BallP1Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    },

                    BallType::Type1P2 => {
                        let _ = commands.spawn(
                            cmp_ball_type1::BallP2Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    }

                    BallType::Type2P1 => {
                        let _ = commands.spawn(
                            cmp_ball_type2::BallType2P1Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    },

                    BallType::Type2P2 => {
                        let _ = commands.spawn(
                            cmp_ball_type2::BallType2P2Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    }

                    BallType::Type3P1 => {
                        let _ = commands.spawn(
                            cmp_ball_type3::BallType3P1Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    },

                    BallType::Type3P2 => {
                        let _ = commands.spawn(
                            cmp_ball_type3::BallType3P2Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    }

                    BallType::Type4P1 => {
                        let _ = commands.spawn(
                            cmp_ball_type4::BallType4P1Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    },

                    BallType::Type4P2 => {
                        let _ = commands.spawn(
                            cmp_ball_type4::BallType4P2Bundle::from((Vec2::new(x, y), Vec2::ZERO, game_assets)));
                    }
                }
            }
        }
    }
}


const FILE_NAME: &str = "/gate_generic.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, GateGeneric)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, gg) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(GateGenericBundle::from((t, r, s, gg)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &GateGeneric)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, GateGeneric)> = vec![];

        for (e, t, gg) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, gg.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

