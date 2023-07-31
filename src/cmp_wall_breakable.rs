use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::constants;
use crate::cmp_ball::Ball;
use crate::cmp_ball_zundamon::Zundamon;
use crate::cmp_ball_zombie::Zombie;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_combat::Status;
use crate::cmp_combat::Player1;
use crate::cmp_combat::Player2;
use crate::cmp_ball_zundamon;
use crate::cmp_ball_zombie;

use crate::cmp_wall;
use crate::cmp_wall::WallBundle;

use crate::cmp_rotator::Rotator;
use crate::edit_context::*;

const BALL_SIZE: f32 = 10.0;
pub const DEFAULT_SIZE_X: f32 = 30.0;
pub const DEFAULT_SIZE_Y: f32 = 400.0;

const HP: f32 = 30.0;
const ATTACK: f32 = 100.0;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct WallBreakable;

#[derive(Bundle)]
pub struct WallBreakableP1Bundle {
    wall_breakable: WallBreakable,
    player: Player1,
    status: Status,
    #[bundle]
    wall_bundle: WallBundle,
}


#[derive(Bundle)]
pub struct WallBreakableP2Bundle {
    wall_breakable: WallBreakable,
    player: Player2,
    status: Status,
    #[bundle]
    wall_bundle: WallBundle,
}


impl Default for WallBreakableP1Bundle {
    fn default() -> Self {
        Self {
            wall_breakable: WallBreakable,
            player: Player1,
            status: Status {
                hp: HP,
                hp_max: HP,
                attack: ATTACK,
            },
            wall_bundle: WallBundle::default(),
        }
    }
}

impl From<(Vec3, Quat, Vec3)> for WallBreakableP1Bundle {
    fn from(tuple: (Vec3, Quat, Vec3)) -> Self {
        let (translation, rotation, scale) = tuple;

        let mut bundle = WallBreakableP1Bundle::default();

        bundle.wall_bundle.sprite_bundle.transform = Transform {
                    translation,
                    rotation,
                    scale,
                };

        bundle.wall_bundle.collision_groups = CollisionGroups::new(Group::GROUP_1, Group::GROUP_11);
        bundle
    }
}

impl From<(Vec3, Quat, Vec3, WallBreakable)> for WallBreakableP1Bundle {
    fn from(tuple: (Vec3, Quat, Vec3, WallBreakable)) -> Self {
        let (translation, rotation, scale, _) = tuple;
        WallBreakableP1Bundle::from((translation, rotation, scale))
    }
}

impl Default for WallBreakableP2Bundle {
    fn default() -> Self {
        Self {
            wall_breakable: WallBreakable,
            player: Player2,
            status: Status {
                hp: HP,
                hp_max: HP,
                attack: ATTACK,
            },
            wall_bundle: WallBundle::default(),
        }
    }
}

impl From<(Vec3, Quat, Vec3)> for WallBreakableP2Bundle {
    fn from(tuple: (Vec3, Quat, Vec3)) -> Self {
        let (translation, rotation, scale) = tuple;

        let mut bundle = WallBreakableP2Bundle::default();

        bundle.wall_bundle.sprite_bundle.transform = Transform {
                    translation,
                    rotation,
                    scale,
                };

        bundle.wall_bundle.collision_groups = CollisionGroups::new(Group::GROUP_1, Group::GROUP_10);
        bundle
    }
}

impl From<(Vec3, Quat, Vec3, WallBreakable)> for WallBreakableP2Bundle {
    fn from(tuple: (Vec3, Quat, Vec3, WallBreakable)) -> Self {
        let (translation, rotation, scale, _) = tuple;
        WallBreakableP2Bundle::from((translation, rotation, scale))
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
        if let MapObject::WallBreakableP1 = map_object {

            let mut entity = commands.spawn(
                WallBreakableP1Bundle::from((Vec3::from((world_position.translation, 0.0)),
                                        Quat::from_rotation_z(0.0),
                                        Vec3::ONE))
                );

            *edit_context = EditContext::Edit(MapObject::Wall, vec![entity.id()], EditTool::Select);
        }

        if let MapObject::WallBreakableP2 = map_object {

            let mut entity = commands.spawn(
                WallBreakableP2Bundle::from((Vec3::from((world_position.translation, 0.0)),
                                        Quat::from_rotation_z(0.0),
                                        Vec3::ONE))
                );

            *edit_context = EditContext::Edit(MapObject::Wall, vec![entity.id()], EditTool::Select);
        }
    }

}

pub fn system_damage<T1: Component, T2: Component>(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut wall_q: Query<(Entity, &mut Status, &Transform), (With<WallBreakable>, With<T1>, Without<T2>)>,
    mut ball_q: Query<(Entity, &mut Status, &Transform), (With<Ball>, With<T2>, Without<T1>)>,
) {
    for (wall_e, mut wall_s, wall_t) in wall_q.iter_mut() {
        for (ball_e, mut ball_s, ball_t) in ball_q.iter_mut() {
            if rapier_context.contact_pair(wall_e.clone(), ball_e.clone()).is_some() {
                ball_s.hp = ball_s.hp - wall_s.attack;
                wall_s.hp = wall_s.hp - 1.0;

                if wall_s.hp <= 0.0 {
                    commands.entity(wall_e).despawn();
                }
            }
        }
    }
}


pub fn system_color(
    mut query: Query<(&Status, &mut Sprite), With<WallBreakable>>,
    ) {

    for (status, mut sprite) in query.iter_mut() {
        let a = status.hp / status.hp_max;
        println!("{:?}", a);
        let color = Color::BLACK.with_a(a);
        sprite.color = color;
    }

}

use crate::ev_save_load_world::LoadWorldEvent;
const FILE_NAME1: &str = "/wall_breakable_p1.map";
pub fn load_p1(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME1);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, WallBreakable)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, w) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(WallBreakableP1Bundle::from((t, r, s, w)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save_p1(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &WallBreakable), With<Player1>>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, WallBreakable)> = vec![];

        for (e, t, w) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, w.to_owned()));
        }

        std::fs::write(dir + FILE_NAME1, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

const FILE_NAME2: &str = "/wall_breakable_p2.map";
pub fn load_p2(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME2);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, WallBreakable)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, w) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(WallBreakableP1Bundle::from((t, r, s, w)));
            }
        }
    }
}


pub fn save_p2(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &WallBreakable), With<Player2>>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, WallBreakable)> = vec![];

        for (e, t, w) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, w.to_owned()));
        }

        std::fs::write(dir + FILE_NAME2, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

