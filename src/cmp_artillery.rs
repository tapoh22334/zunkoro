use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball;
use crate::cmp_ball::Ball;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Artillery {
    pub scale: f32,
    pub position: Vec2,
    pub angvel: f32,
    pub angle: f32,
    pub angle_range: (f32, f32),
    }

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Barrel;

pub fn add(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            artillery: Artillery) -> Entity {

    let s = artillery.scale;
    let pos = artillery.position;
    let angvel = artillery.angvel;

    // Fragment 1
    let sprite_handle = game_assets.image_handles.get("artillery_frag1").unwrap();
    let mut entity = commands.spawn(artillery);

    entity
        .insert((
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: sprite_handle.clone(),
                ..default()
            },
            ));

    entity
        .insert(TransformBundle {
            local: Transform {
                translation: Vec3::new(pos.x, pos.y, 2.0),
                scale: s * Vec3::ONE,
                ..Default::default()
            },
            ..default()
        },)
        .insert(BBSize{x: 512.0, y: 512.0})
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(Collider::ball(256.0))
        .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1))
        .insert(Sensor)
        ;

    // Fragment2
    let parent_id = entity.id();
    entity.with_children(|children| {
        let sprite_handle = game_assets.image_handles.get("artillery_frag2").unwrap();
        let sprite_image = image_assets.get(sprite_handle).unwrap();
        let mut child = children.spawn(Barrel);
        child.insert(SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: sprite_handle.clone(),
                ..default()
            },);

        child
            .insert(
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 2.0))
            );

    });


    return entity.id();
}

pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    time: Res<Time>,
    mut artillery_frag1: Query<&mut Artillery>,
    mut artillery_frag2: Query<(&Parent, &mut Transform, &Barrel)>,
) {
    for (parent, mut barrel_transform, barrel) in artillery_frag2.iter_mut() {
        let mut artillery = artillery_frag1.get_mut(parent.get()).unwrap();
        let mut new_angle = artillery.angle + artillery.angvel * time.delta_seconds();

        let pivot_rotation = Quat::from_rotation_z(new_angle - artillery.angle);
        barrel_transform.rotate_around(Vec3::ZERO, pivot_rotation);

        artillery.angle = new_angle;

        if artillery.angle <= artillery.angle_range.0 {
            artillery.angvel = artillery.angvel.abs();
        } else if artillery.angle >= artillery.angle_range.1 {
            artillery.angvel = -artillery.angvel.abs();
        }
    }
}


pub fn system_fire(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(Entity, &mut Transform, &mut Velocity), With<Ball>>,
    mut artillery_q: Query<(Entity, &Transform, &BBSize, &Artillery), Without<Ball>>,
) {
    for (artillery_e, artillery_transform, bbsize, artillery) in artillery_q.iter() {
        for (ball_e, mut ball_transform, mut ball_velocity) in ball_q.iter_mut() {
            if rapier_context.intersection_pair(artillery_e, ball_e) == Some(true) {
                let dir = Quat::from_rotation_z(artillery.angle).mul_vec3(Vec3::new(1.0, 0.0, 0.0));
                let dist = bbsize.x / 2.0 * artillery_transform.scale.x + 16.0;
                ball_transform.translation = artillery_transform.translation + dir * dist;
                ball_velocity.linvel = dir.truncate() * 400.0;
            }
        }
    }
}

