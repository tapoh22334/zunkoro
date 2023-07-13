use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball::Ball;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct PadVelocity {
    pub size: Vec2,
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32
}

pub fn add(commands: &mut Commands,
                    game_assets: &Res<GameAsset>,
                    pad_velocity: PadVelocity) -> Entity {
    let size = pad_velocity.size;
    let pos = pad_velocity.position;
    let dir = pad_velocity.direction;

    let sprite_handle = game_assets.image_handles.get("pad_velocity_handle").unwrap();
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                texture: sprite_handle.clone(),
                ..Default::default()
            });

    let angle = Vec2::new(0.0, 1.0).angle_between(dir);
    entity
        .insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.0),
                    rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle),
                    ..Default::default()
                },
                ..default()
                })
        .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))
        .insert(Sensor)
        .insert(BBSize{x: size.x, y: size.y})
        .insert(pad_velocity);

    return entity.id();
}


pub fn system(
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(Entity, &mut Velocity), With<Ball>>,
    pb_q: Query<(Entity, &PadVelocity)>,
) {
    for (pb_e, pb) in pb_q.iter() {
        for (ball_e, mut ball_v) in ball_q.iter_mut() {
            if rapier_context.intersection_pair(pb_e, ball_e) == Some(true) {
                ball_v.linvel = pb.speed * pb.direction;
            }
        }
    }
}


const FILE_NAME: &str = "/pad_velocity.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<PadVelocity> = serde_json::from_str(&json_str).unwrap();

            for e in elem_list {
                add(&mut commands, &game_assets, e);
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &PadVelocity)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<PadVelocity> = vec![];

        for (t, e) in q.iter() {
            let mut e = e.clone();
            e.size = e.size * t.scale.truncate();
            e.position = t.translation.truncate();
            elem_list.push(e.clone());
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

