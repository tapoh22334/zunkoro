use serde::{Serialize, Deserialize};
use rand::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_ball::Ball;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_fuse_time::FuseTime;
use crate::cmp_blood;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Shredder {
    pub scale: f32,
    pub polyline: Vec<Vec2>,
    pub target_point: usize,
    pub speed: f32,
    pub time_offset: f32
}

pub fn add(commands: &mut Commands,
                    game_assets: &Res<GameAsset>,
                    image_assets: &Res<Assets<Image>>,
                    shredder: Shredder) -> Entity {
    let sprite_handle = game_assets.image_handles.get("shredder_512_handle").unwrap();

    let mut entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: sprite_handle.clone(),
                ..default()
            },
        ));

    entity
        .insert(Interaction::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: -3.0,
        });

    entity.insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(shredder.polyline[0].x, shredder.polyline[0].y, 0.0),
                    scale: shredder.scale * Vec3::ONE,
                    ..Default::default()
                },
                ..default()
                },
        );

    entity.insert(BBSize{x: 512.0, y: 512.0});
    entity.insert(FuseTime{timer: Timer::from_seconds(shredder.time_offset, TimerMode::Once)} );
    entity.insert(shredder);

    return entity.id();

}


pub fn system_move(
    time: Res<Time>,
    mut shredder_q: Query<(&mut Transform, &mut Velocity, &mut BBSize, &mut FuseTime, &mut Shredder)>,
) {
    for (mut t, mut v, _, mut fuse_time, mut shredder) in shredder_q.iter_mut() {
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
    shredder_q: Query<(&Transform, &BBSize, &Shredder)>,
    ball_q: Query<&Transform, With<Ball>>,
) {
    for (transform, bbsize, shredder) in shredder_q.iter(){
        let r = bbsize.x / 2.0 * transform.scale.truncate().x * 0.9;
        let shape = Collider::ball(r);
        let shape_pos = transform.translation.truncate();
        let shape_rot = 0.0;
        let filter = QueryFilter::only_dynamic()
                        .groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));
        let mut rng = rand::thread_rng();
        let sv = vec![ "zundamon_die1_handle",
                        "zundamon_die2_handle",
                        "zundamon_die3_handle",
                        "zundamon_die4_handle",
                        "zundamon_die5_handle",
                        "zundamon_die6_handle",
                        "zundamon_die7_handle",
                     ];
        let random_audio = sv[rng.gen_range(0..sv.len())];

        rapier_context.intersections_with_shape(
            shape_pos, shape_rot, &shape, filter, |entity| {
                let blood_pos = ball_q.get(entity).unwrap().translation.truncate();
                cmp_blood::add(&mut commands, blood_pos);
                commands.entity(entity).despawn();
                audio.play(game_assets.audio_handles.get(random_audio).unwrap().clone());
                true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        });

    }
}


