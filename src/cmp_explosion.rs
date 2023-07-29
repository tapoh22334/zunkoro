use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_fuse_time::FuseTime;
use crate::cmp_combat::Status;

const LIFE_TIME: f32 = 1.0;
const DEFAULT_RESTITUTION: f32 = 0.0;
const DEFAULT_FRICTION: f32 = 0.9;

const RADIUS: f32 = 80.0;
const HP: f32 = 50.0;
const ATTACK: f32 = 1.0;
const ANGVEL: f32 = -0.5;


#[derive(Component)]
pub struct Explosion;

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    pub status: Status,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub friction: Friction,
    pub collider: Collider,
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
                attack: ATTACK,
            },
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(DEFAULT_RESTITUTION),
            friction: Friction::coefficient(DEFAULT_FRICTION),
            collider: Collider::ball(1.0),
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

        let handle = game_assets.image_handles.get("bomb_handle").unwrap();
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
            transform.scale = (transform.scale * 1.1).clamp(Vec3::ZERO, Vec3::ONE * 100.0);
        }
    }
}


