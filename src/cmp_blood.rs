use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_fuse_time::FuseTime;

#[derive(Component)]
pub struct Blood;

const SPAWN_NUM: usize = 16;
const LIFE_TIME: f32 = 3.0;
const SIZE: f32 = 4.0;

pub fn add(commands: &mut Commands, pos: Vec2) {
    let mut rng = rand::thread_rng();

    for i in 0..SPAWN_NUM {
        let angle = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
        let speed = rng.gen_range(0.0..100.0);
        let distance = rng.gen_range(0.0..SIZE);

        let dir = Quat::from_rotation_z(angle).mul_vec3(Vec3::new(0.0, 1.0, 0.0));
        let spawn_pos = dir * distance;
        let velocity = dir * speed;

        commands
            .spawn(Blood)
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(SIZE / 2.0))
            .insert(CollisionGroups::new(Group::GROUP_2, Group::ALL))
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::ONE * SIZE),
                    ..Default::default()
                },
                ..Default::default()
            })
        .insert(TransformBundle {
            local: Transform {
                translation: Vec3::new(pos.x + spawn_pos.x, pos.y + spawn_pos.y, 1.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Velocity {
            linvel: velocity.truncate(),
            angvel: 0.0,
        })
        .insert(FuseTime{timer: Timer::from_seconds(LIFE_TIME, TimerMode::Once)} );
        ;

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


