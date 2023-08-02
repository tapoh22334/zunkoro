use bevy::input::gamepad;
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
use crate::cmp_explosion;
use crate::cmp_explosion::ExplosionBundle;

use crate::cmp_wall;
use crate::cmp_wall::WallBundle;

use crate::cmp_rotator::Rotator;
use crate::edit_context::*;

const BALL_SIZE: f32 = 10.0;
pub const DEFAULT_SIZE_X: f32 = 30.0;
pub const DEFAULT_SIZE_Y: f32 = 400.0;

const EXPLOSION_RADIUS: f32 = 200.0;
const HP: f32 = 100.0;
const ATTACK: f32 = 3.0;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct BreakableSync {
    entities: Vec<u32>,
}

#[derive(Bundle)]
pub struct BreakableSyncP1Bundle {
    breakable_sync: BreakableSync,
    player: Player1,
    status: Status,
    collision_groups: CollisionGroups,
}


#[derive(Bundle)]
pub struct BreakableSyncP2Bundle {
    breakable_sync: BreakableSync,
    player: Player2,
    status: Status,
    collision_groups: CollisionGroups,
}


impl Default for BreakableSyncP1Bundle {
    fn default() -> Self {
        Self {
            breakable_sync: BreakableSync { entities: vec![] },
            player: Player1,
            status: Status {
                hp: HP,
                hp_max: HP,
                attack: ATTACK,
            },
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::GROUP_11),
        }
    }
}

impl From<(Status, BreakableSync)> for BreakableSyncP1Bundle {
    fn from(tuple: (Status, BreakableSync)) -> Self {
        let (status, breakable_sync) = tuple;

        let mut bundle = BreakableSyncP1Bundle::default();
        bundle.status = status;
        bundle.breakable_sync = breakable_sync;

        bundle
    }
}

impl Default for BreakableSyncP2Bundle {
    fn default() -> Self {
        Self {
            breakable_sync: BreakableSync { entities: vec![] },
            player: Player2,
            status: Status {
                hp: HP,
                hp_max: HP,
                attack: ATTACK,
            },
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::GROUP_10),
        }
    }
}

impl From<(Status, BreakableSync)> for BreakableSyncP2Bundle {
    fn from(tuple: (Status, BreakableSync)) -> Self {
        let (status, breakable_sync) = tuple;

        let mut bundle = BreakableSyncP2Bundle::default();
        bundle.status = status;
        bundle.breakable_sync = breakable_sync;

        bundle
    }
}

pub fn handle_user_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut edit_context: ResMut<EditContext>,
    world_position: ResMut<WorldPosition>,
    ) {

    if let EditContext::Spawn(map_object) = edit_context.to_owned() {
        if let MapObject::BreakableSyncP1(ref entities) = map_object {
            let mut iter = entities.iter();
            let base_entity = iter.next().unwrap();

            let mut bundle = BreakableSyncP1Bundle::default();
            bundle.breakable_sync.entities = iter.map(|x| x.index()).collect();

            let _ = commands.entity(*base_entity).insert(bundle);

            *edit_context = EditContext::Edit(MapObject::None, vec![], EditTool::Select);
        }

        if let MapObject::BreakableSyncP2(ref entities) = map_object {
            let mut iter = entities.iter();
            let base_entity = iter.next().unwrap();

            let mut bundle = BreakableSyncP2Bundle::default();
            bundle.breakable_sync.entities = iter.map(|x| x.index()).collect();

            let _ = commands.entity(*base_entity).insert(bundle);

            *edit_context = EditContext::Edit(MapObject::None, vec![], EditTool::Select);
        }
    }

}

use crate::ev_despawn;
pub fn system_damage<T1: Component + Default, T2: Component>(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    game_assets: Res<GameAsset>,
    mut event: EventWriter<ev_despawn::Despawn>,
    mut wall_q: Query<(Entity, &mut Status, &Transform, &BreakableSync), (With<T1>, Without<T2>)>,
    mut ball_q: Query<(Entity, &mut Status, &Transform), (With<Ball>, With<T2>, Without<T1>)>,
) {
    let game_assets = game_assets.into_inner();
    for (wall_e, mut wall_s, wall_t, breakable_sync) in wall_q.iter_mut() {
        for (ball_e, mut ball_s, ball_t) in ball_q.iter_mut() {
            if rapier_context.contact_pair(wall_e.clone(), ball_e.clone()).is_some() {
                ball_s.hp = ball_s.hp - wall_s.attack;
                wall_s.hp = wall_s.hp - 1.0;

                if wall_s.hp <= 0.0 {
                    for entity in breakable_sync.entities.iter() {
                        commands.entity(wall_e).despawn();
                        event.send(ev_despawn::Despawn(*entity));
                        commands.spawn(cmp_explosion::ExplosionBundle::from((wall_t.translation, EXPLOSION_RADIUS, game_assets)))
                                .insert(T1::default());
                    }
                }
            }
        }
    }
}


pub fn system_color(
    mut query: Query<(&Status, &mut Sprite), With<BreakableSync>>,
    ) {

    for (status, mut sprite) in query.iter_mut() {
        let a = status.hp / status.hp_max;
        let color = Color::BLACK.with_a(a / 2.0 + 0.5);
        sprite.color = color;
    }

}

use crate::ev_save_load_world::LoadWorldEvent;
const FILE_NAME1: &str = "/breakable_sync_p1.map";
pub fn load_p1(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME1);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Status, BreakableSync)> = serde_json::from_str(&json_str).unwrap();

            for (i, s, b) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(BreakableSyncP1Bundle::from((s, b)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save_p1(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Status, &BreakableSync), With<Player1>>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Status, BreakableSync)> = vec![];

        for (e, s, b) in q.iter() {
            elem_list.push((e.index(), s.to_owned(), b.to_owned()));
        }

        std::fs::write(dir + FILE_NAME1, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

const FILE_NAME2: &str = "/breakable_sync_p2.map";
pub fn load_p2(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME2);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Status, BreakableSync)> = serde_json::from_str(&json_str).unwrap();

            for (i, s, b) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(BreakableSyncP2Bundle::from((s, b)));
            }
        }
    }
}


pub fn save_p2(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Status, &BreakableSync), With<Player2>>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Status, BreakableSync)> = vec![];

        for (e, s, b) in q.iter() {
            elem_list.push((e.index(), s.to_owned(), b.to_owned()));
        }

        std::fs::write(dir + FILE_NAME2, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

