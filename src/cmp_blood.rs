use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_fuse_time::FuseTime;

const SPAWN_NUM: usize = 8;
const LIFE_TIME: f32 = 3.0;
const BALL_RADIUS: f32 = 2.0;
const DEFAULT_RESTITUTION: f32 = 0.0;
const DEFAULT_FRICTION: f32 = 0.9;


#[derive(Component)]
pub struct Blood;

#[derive(Bundle)]
pub struct BloodBundle {
    pub blood: Blood,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub friction: Friction,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub velocity: Velocity,
    pub fuse_time: FuseTime,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

impl Default for BloodBundle {
    fn default() -> Self {
        Self {
            blood: Blood,
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(DEFAULT_RESTITUTION),
            friction: Friction::coefficient(DEFAULT_FRICTION),
            collider: Collider::ball(BALL_RADIUS),
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),
            velocity: Velocity { linvel: Vec2::ZERO, angvel: 0.0 },
            fuse_time: FuseTime{timer: Timer::from_seconds(LIFE_TIME, TimerMode::Once)},
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::ONE * BALL_RADIUS * 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

impl From<(Vec3, Vec2)> for BloodBundle {
    fn from(tuple: (Vec3, Vec2)) -> Self {
        let (translation, velocity) = tuple;

        let mut bundle = BloodBundle::default();
        bundle.sprite_bundle.transform.translation = translation;
        bundle.velocity.linvel = velocity;

        bundle
    }
}


pub fn add(commands: &mut Commands, pos: Vec2, num: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let angle = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
        let speed = rng.gen_range(0.0..1000.0);
        let distance = rng.gen_range(0.0..BALL_RADIUS);

        let dir = Quat::from_rotation_z(angle).mul_vec3(Vec3::new(0.0, 1.0, 0.0));
        let spawn_pos = dir * distance;
        let velocity = (dir * speed).truncate();

        let translation = Vec3::new(pos.x + spawn_pos.x, pos.y + spawn_pos.y, 1.0);
        commands.spawn(BloodBundle::from((translation, velocity)));

    }
}


pub fn system(
    mut commands: Commands,
    time: Res<Time>,
    mut blood_q: Query<(Entity, &mut FuseTime), With<Blood>>,
) {
    for (entity, mut fuse_time) in blood_q.iter_mut() {
        fuse_time.timer.tick(time.delta());
        if fuse_time.timer.finished() { 
            commands.entity(entity).despawn();
        }
    }
}


