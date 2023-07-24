use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
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
            artillery: Artillery) -> Entity {

    let s = artillery.scale;
    let pos = artillery.position;

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
        .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2))
        .insert(Sensor)
        ;

    // Fragment2
    entity.with_children(|children| {
        let sprite_handle = game_assets.image_handles.get("artillery_frag2").unwrap();
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
    time: Res<Time>,
    mut artillery_frag1: Query<&mut Artillery>,
    mut artillery_frag2: Query<(&Parent, &mut Transform), With<Barrel>>,
) {
    for (parent, mut barrel_transform) in artillery_frag2.iter_mut() {
        let mut artillery = artillery_frag1.get_mut(parent.get()).unwrap();
        let new_angle = artillery.angle + artillery.angvel * time.delta_seconds();

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
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(Entity, &mut Transform, &mut Velocity, &Ball)>,
    artillery_q: Query<(Entity, &Transform, &BBSize, &Artillery), Without<Ball>>,
) {
    for (artillery_e, artillery_transform, bbsize, artillery) in artillery_q.iter() {
        for (ball_e, mut ball_transform, mut ball_velocity, ball) in ball_q.iter_mut() {
            if rapier_context.intersection_pair(artillery_e, ball_e) == Some(true) {
                let dir = Quat::from_rotation_z(artillery.angle).mul_vec3(Vec3::new(1.0, 0.0, 0.0));
                let dist = bbsize.x / 2.0 * artillery_transform.scale.x + ball.radius + 1.0;
                ball_transform.translation = artillery_transform.translation + dir * dist;
                ball_velocity.linvel = dir.truncate() * 400.0;
            }
        }
    }
}

const FILE_NAME: &str = "/artillery.map";
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
            let elem_list: Vec<Artillery> = serde_json::from_str(&json_str).unwrap();

            for e in elem_list {
                add(&mut commands, &game_assets, e);
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              artillery_q: Query<(&Transform, &Artillery)>) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut artillery_list: Vec<Artillery> = vec![];

        for (t, e) in artillery_q.iter() {
            let mut e = e.clone();
            e.scale = t.scale.truncate().x;
            e.position = t.translation.truncate();
            artillery_list.push(e.clone());
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&artillery_list).unwrap()).unwrap();
    }
}

