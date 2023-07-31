use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_ball::Ball;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_fuse_time::FuseTime;
use crate::cmp_combat::Status;
use crate::cmp_combat::Player1;
use crate::cmp_combat::Player2;

const LIFE_TIME: f32 = 1.0;
const DEFAULT_RESTITUTION: f32 = 0.0;
const DEFAULT_FRICTION: f32 = 0.9;

const RADIUS: f32 = 80.0;
const HP: f32 = 0.0;
const ATTACK: f32 = 50.0;
//const ANGVEL: f32 = -0.5;


#[derive(Component)]
pub struct Explosion;

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    pub status: Status,
    pub collider: Collider,
    pub sensor: Sensor,
    pub collision_groups: CollisionGroups,
    pub fuse_time: FuseTime,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        Self {
            explosion: Explosion,
            status: Status {
                hp: HP,
                hp_max: HP,
                attack: ATTACK,
            },
            collider: Collider::ball(1.0),
            sensor: Sensor,
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),
            fuse_time: FuseTime{timer: Timer::from_seconds(LIFE_TIME, TimerMode::Once)},
            sprite_bundle: SpriteBundle {
                ..default()
            },
        }
    }
}

impl From<(Vec3, &GameAsset)> for ExplosionBundle {
    fn from(tuple: (Vec3, &GameAsset)) -> Self {
        let (translation, game_assets) = tuple;

        let mut bundle = ExplosionBundle::default();

        let handle = game_assets.image_handles.get("explosion_handle").unwrap();
        bundle.sprite_bundle = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * (1.0 * 2.0)),
                ..default()
            },
            texture: handle.clone(),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        };


        bundle
    }
}

pub fn system(
    mut commands: Commands,
    time: Res<Time>,
    mut explosion_q: Query<(Entity, &mut Transform, &mut FuseTime), With<Explosion>>,
) {
    for (entity, mut transform, mut fuse_time) in explosion_q.iter_mut() {
        fuse_time.timer.tick(time.delta());
        if fuse_time.timer.finished() { 
            commands.entity(entity).despawn();
        } else {
            transform.scale = (transform.scale * 1.1).clamp(Vec3::ZERO, Vec3::ONE * 200.0);
        }
    }
}


pub fn system_damage1(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut explosion_q: Query<(Entity, &Status, &Transform), (With<Explosion>, With<Player1>, Without<Player2>)>,
    mut ball_q: Query<(Entity, &mut Status, &Transform), (With<Ball>, With<Player2>, Without<Player1>)>,
) {
    for (sensor_e, sensor_s, sensor_t) in explosion_q.iter_mut() {
        for (ball_e, mut ball_s, ball_t) in ball_q.iter_mut() {
            if rapier_context.intersection_pair(sensor_e.clone(), ball_e.clone()).is_some() {
                ball_s.hp = ball_s.hp - sensor_s.attack;
            }
        }
    }
}


pub fn system_damage2(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut explosion_q: Query<(Entity, &Status, &Transform), (With<Explosion>, With<Player2>, Without<Player1>)>,
    mut ball_q: Query<(Entity, &mut Status, &Transform), (With<Ball>, With<Player1>, Without<Player2>)>,
) {
    for (sensor_e, sensor_s, sensor_t) in explosion_q.iter_mut() {
        for (ball_e, mut ball_s, ball_t) in ball_q.iter_mut() {
            if rapier_context.intersection_pair(sensor_e.clone(), ball_e.clone()).is_some() {
                ball_s.hp = ball_s.hp - sensor_s.attack;
            }
        }
    }
}
