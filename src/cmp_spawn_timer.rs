use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_ball_zundamon::Zundamon;
use crate::cmp_ball_zombie::Zombie;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_fuse_time::FuseTime;
use crate::cmp_ball_zundamon;
use crate::cmp_ball_zombie;
use crate::cmp_ball_type1;

use crate::cmp_gate_generic;
use crate::cmp_gate_generic::SpawnBall;

use crate::cmp_rotator::Rotator;
use crate::edit_context::*;

const BALL_SIZE: f32 = 10.0;
pub const DEFAULT_SIZE_X: f32 = 10.0;
pub const DEFAULT_SIZE_Y: f32 = 10.0;


#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct SpawnTimer {
    pub signals: Vec<SpawnBall>,
    pub seconds: f32,
}


#[derive(Bundle)]
pub struct SpawnTimerBundle {
    bbsize: BBSize,
    fuse_time: FuseTime,
    spawn_timer: SpawnTimer,
    map_object: MapObject,
    #[bundle]
    sprite_bundle: SpriteBundle,
}


impl Default for SpawnTimerBundle {
    fn default() -> Self {
        Self {
            bbsize: BBSize {x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            fuse_time: FuseTime {
                timer: Timer::from_seconds(0.0, TimerMode::Once)
            },
            spawn_timer: SpawnTimer {
                signals: vec![],
                seconds: 0.0,
            },
            map_object: MapObject::SpawnTimer(vec![]),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(DEFAULT_SIZE_X, DEFAULT_SIZE_Y)),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::ONE),
                ..Default::default()
            },
        }
    }
}

impl From<(Vec3, Quat, Vec3, SpawnTimer)> for SpawnTimerBundle {
    fn from(tuple: (Vec3, Quat, Vec3, SpawnTimer)) -> Self {
        let (translation, rotation, scale, spawn_timer) = tuple;

        let mut bundle = SpawnTimerBundle::default();

        bundle.sprite_bundle.transform = Transform {
                    translation,
                    rotation,
                    scale,
                };
        bundle.spawn_timer = spawn_timer;

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
        if let MapObject::SpawnTimer(signals) = map_object {

            let mut entity = commands.spawn(
                SpawnTimerBundle::from((Vec3::from((world_position.translation, 0.0)),
                                        Quat::from_rotation_z(0.0),
                                        Vec3::ONE,
                                        SpawnTimer { signals, seconds: 0.0, },
                                        ))
                );

            *edit_context = EditContext::Edit(MapObject::SpawnTimer(vec![]), vec![entity.id()], EditTool::Select);
        }
    }
}

pub fn system_setup(
    mut query: Query<&mut Sprite, With<SpawnTimer>>,
    mut fuse_time_q: Query<(&mut FuseTime, &SpawnTimer)>,
    ) {

    for mut sprite in query.iter_mut() {
        let color = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0, };
        sprite.color = color;
    }

    for (mut fuse_time, spawn_timer) in fuse_time_q.iter_mut() {
        *fuse_time = FuseTime { timer: Timer::from_seconds(spawn_timer.seconds, TimerMode::Once) };
    }
}

pub fn system(
    mut commands: Commands,
    time: Res<Time>,
    mut event: EventWriter<SpawnBall>,
    mut trajectory_q: Query<(Entity, &mut FuseTime, &mut SpawnTimer)>,
) {
    for (entity, mut fuse_time, mut spawn_timer) in trajectory_q.iter_mut() {
        fuse_time.timer.tick(time.delta());
        if fuse_time.timer.finished() { 
            for s in spawn_timer.signals.iter() {
                event.send(s.clone());
                commands.entity(entity).despawn();
            }
        }
    }
}


const FILE_NAME: &str = "/spawn_timer.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, SpawnTimer)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, st) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(SpawnTimerBundle::from((t, r, s, st)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &SpawnTimer)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, SpawnTimer)> = vec![];

        for (e, t, st) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, st.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

