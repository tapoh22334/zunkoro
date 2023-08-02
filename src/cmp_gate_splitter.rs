use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_ball::Ball;
use crate::cmp_ball_zundamon::Zundamon;
use crate::cmp_ball_zombie::Zombie;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball_zundamon;
use crate::cmp_ball_zombie;
use crate::cmp_gate_generic;

use crate::cmp_gate_generic::SpawnBall;

use crate::edit_context::*;

pub const DEFAULT_SIZE_X: f32 = 80.0;
pub const DEFAULT_SIZE_Y: f32 = 10.0;


#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateSplitter {
    signals: Vec<SpawnBall>,
    remaining: Option<u32>,
}


#[derive(Bundle)]
pub struct GateSplitterBundle {
    bbsize: BBSize,
    gate_splitter: GateSplitter,
    collider: Collider,
    sensor: Sensor,
    map_object: MapObject,
    #[bundle]
    sprite_bundle: SpriteBundle,
}


impl Default for GateSplitterBundle {
    fn default() -> Self {
        Self {
            bbsize: BBSize {x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            gate_splitter: GateSplitter {
                signals: vec![],
                remaining: None,
            },
            collider: Collider::cuboid(DEFAULT_SIZE_X / 2.0, DEFAULT_SIZE_Y / 2.0),
            sensor: Sensor,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(DEFAULT_SIZE_X, DEFAULT_SIZE_Y)),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::ONE),
                ..Default::default()
            },
            map_object: MapObject::GateSplitter(vec![]),
        }
    }
}

impl From<(Vec3, Quat, Vec3, GateSplitter)> for GateSplitterBundle {
    fn from(tuple: (Vec3, Quat, Vec3, GateSplitter)) -> Self {
        let (translation, rotation, scale, gate_splitter) = tuple;

        let mut bundle = GateSplitterBundle::default();

        bundle.sprite_bundle.transform = Transform {
                    translation,
                    rotation,
                    scale,
                };
        bundle.gate_splitter = gate_splitter;

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
        if let MapObject::GateSplitter(signals) = map_object {

            let mut entity = commands.spawn(
                GateSplitterBundle::from(
                    (Vec3::from((world_position.translation, 0.0)),
                    Quat::from_rotation_z(0.0),
                    Vec3::ONE,
                    GateSplitter {signals, remaining: None},
                    ))
                );

            entity.insert(MapObject::GateSplitter(vec![]));
            *edit_context = EditContext::Edit(MapObject::GateSplitter(vec![]), vec![entity.id()], EditTool::Select);
        }
    }
}

pub fn system_setup(
    mut query: Query<&mut Sprite, With<GateSplitter>>,
    ) {

}

pub fn system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<Entity, With<Ball>>,
    mut splitter_q: Query<(Entity, &mut Sprite, &mut GateSplitter)>,
    mut event: EventWriter<cmp_gate_generic::SpawnBall>,
) {
    for (splitter_e, mut splitter_sprite, mut splitter) in splitter_q.iter_mut() {
        for ball_e in ball_q.iter_mut() {
            if rapier_context.intersection_pair(splitter_e, ball_e) == Some(true) {
                commands.entity(ball_e).despawn();
                let mut signals = splitter.signals.clone();
                if let Some(remain) = splitter.remaining.as_mut() {
                    if *remain > 0 {
                        *remain -= 1;
                    } else {
                        signals = vec![signals.first().unwrap().clone()];
                        splitter_sprite.color.set_a(0.5);
                    }
                }

                for signal in signals {
                    event.send(signal.clone());
                }
            }
        }
    }
}


const FILE_NAME: &str = "/gate_splitter.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, GateSplitter)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, splitter) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(GateSplitterBundle::from((t, r, s, splitter)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &GateSplitter)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, GateSplitter)> = vec![];

        for (e, t, splitter) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, splitter.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

