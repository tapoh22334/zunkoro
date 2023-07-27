use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_ball;
use crate::cmp_ball::Ball;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_fuse_time::FuseTime;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Shredder {
    pub scale: f32,
    pub polyline: Vec<Vec2>,
    pub target_point: usize,
    pub speed: f32,
    pub time_offset: f32
}

#[derive(Bundle)]
pub struct ShredderBundle {
    interaction: Interaction,
    rigid_body: RigidBody,
    velocity: Velocity,
    bbsize: BBSize,
    fuse_time: FuseTime,
    shredder: Shredder,
    //#[bundle]
    //transform_bundle: TransformBundle,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl From<(&Handle<Image>, &Shredder)> for ShredderBundle {
    fn from(tuple: (&Handle<Image>, &Shredder)) -> Self {
        let (handle, shredder) = tuple;
        Self {
            interaction: Interaction::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: -3.0,
            },
            bbsize: BBSize{x: 512.0, y: 512.0},
            fuse_time: FuseTime{timer: Timer::from_seconds(shredder.time_offset, TimerMode::Once)},
            shredder: shredder.clone(),
            //transform_bundle: TransformBundle {
            //    local: Transform {
            //        translation: Vec3::new(shredder.polyline[0].x, shredder.polyline[0].y, 0.0),
            //        scale: shredder.scale * Vec3::ONE,
            //        ..Default::default()
            //    },
            //    ..default()
            //},
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: handle.clone(),
                transform: Transform {
                    translation: Vec3::new(shredder.polyline[0].x, shredder.polyline[0].y, 0.0),
                    scale: shredder.scale * Vec3::ONE,
                    ..Default::default()
                },
                ..default()
            },
        }
    }
}

pub fn add_index(commands: &mut Commands,
                    index: u32,
                    game_assets: &Res<GameAsset>,
                    shredder: Shredder) -> Entity {
    let sprite_handle = game_assets.image_handles.get("shredder_512_handle").unwrap();

    let mut entity = commands.get_or_spawn(Entity::from_raw(index));
    entity.insert(
            ShredderBundle::from((sprite_handle, &shredder))
        );

    return entity.id();
}

pub fn add(commands: &mut Commands,
                    game_assets: &Res<GameAsset>,
                    shredder: Shredder) -> Entity {
    let sprite_handle = game_assets.image_handles.get("shredder_512_handle").unwrap();

    let mut entity = commands.spawn(
            ShredderBundle::from((sprite_handle, &shredder))
        );

    return entity.id();

}


pub fn system_move(
    time: Res<Time>,
    mut shredder_q: Query<(&Transform, &mut Velocity, &mut BBSize, &mut FuseTime, &mut Shredder)>,
) {
    for (t, mut v, _, mut fuse_time, mut shredder) in shredder_q.iter_mut() {
        fuse_time.timer.tick(time.delta());
        if ! fuse_time.timer.finished() { continue; }

        if shredder.target_point < shredder.polyline.len() - 1 { 
            let target_pos = shredder.polyline[shredder.target_point];
            let distance = t.translation.truncate().distance(target_pos);
            let dir = (target_pos - t.translation.truncate()) / distance;

            let distance_thresh = 10.0;
            if distance < distance_thresh {
                shredder.target_point += 1;
            } else {
                v.linvel = dir * shredder.speed;
            }

        } else {
            v.linvel = Vec2::ZERO;
        }

    }
}

pub fn system_kill(
    mut commands: Commands,
    audio: Res<Audio>,
    game_assets: Res<GameAsset>,
    rapier_context: Res<RapierContext>,
    shredder_q: Query<(&Transform, &BBSize), With<Shredder>>,
    ball_q: Query<&Transform, With<Ball>>,
) {
    let game_assets = game_assets.into_inner();

    for (transform, bbsize) in shredder_q.iter(){
        let r = bbsize.x / 2.0 * transform.scale.truncate().x * 0.9;
        let shape = Collider::ball(r);
        let shape_pos = transform.translation.truncate();
        let shape_rot = 0.0;
        let filter = QueryFilter::only_dynamic()
                        .groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2));

        rapier_context.intersections_with_shape(
            shape_pos, shape_rot, &shape, filter, |entity| {
                let transform = ball_q.get(entity).unwrap();
                cmp_ball::kill(&mut commands, &audio, game_assets, entity, &transform);
                true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        });

    }
}


const FILE_NAME: &str = "/shredder.map";
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
            let elem_list: Vec<(u32, Shredder)> = serde_json::from_str(&json_str).unwrap();

            for (index, e) in elem_list {
                add_index(&mut commands, index, &game_assets, e);
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &Shredder)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Shredder)> = vec![];

        for (entity, t, e) in q.iter() {
            let mut e = e.clone();
            e.scale = t.scale.truncate().x;
            elem_list.push((entity.index(), e.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

