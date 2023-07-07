use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::cmp_main_camera::MainCamera;
use crate::cmp_ball::{Ball, Zundamon};
use crate::cmp_bbsize::BBSize;
use crate::cmp_gate_teleport::{GateTeleportExit, GateTeleportEntrance};
use crate::cmp_pad_velocity::PadVelocity;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_trajectory::Trajectory;
use crate::cmp_trajectory;

#[derive(Component)]
pub struct Zombie;

pub fn add(commands: &mut Commands, game_assets: &Res<GameAsset>, pos: Vec2, r: f32, vel: Vec2) {
    let mut rng = rand::thread_rng();
    let image_vec = vec!["zombie1_handle"];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    let sprite_handle = game_assets.image_handles.get(random_image).unwrap();

    commands
        .spawn(Ball {radius: r, previous_position: None})
        .insert(Zombie)
        .insert(RigidBody::Dynamic)
        .insert(Restitution::coefficient(0.99))
        .insert(Friction::coefficient(0.01))
        .insert(Collider::ball(r))
        .insert(CollisionGroups::new(Group::GROUP_1, Group::ALL))
        .insert(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * (r * 2.0)),
                        ..default()
                    },
                    texture: sprite_handle.clone(),
                    ..default()
        })
        .insert(TransformBundle {
                    local: Transform {
                                translation: Vec3::new(pos.x, pos.y, 1.0),
                                //scale: Vec3::ONE / r,
                                ..Default::default()
                            },
                    ..default()
        })
        .insert(Velocity {
            linvel: vel,
            angvel: 0.0,
        })
    ;
}

pub fn system_infection(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    game_assets: Res<GameAsset>,
    mut zundamon_q: Query<(Entity, &Transform, &Velocity, &Ball), With<Zundamon>>,
    mut zombie_q: Query<(Entity, &Transform, &Velocity), (With<Zombie>, Without<Zundamon>)>,
) {
    for (zundamon_e, zundamon_t, zundamon_v, zundamon_ball) in zundamon_q.iter() {
        for (zombie_e, zombie_t, zombie_v) in zombie_q.iter() {
            if let Some(_) = rapier_context.contact_pair(zundamon_e, zombie_e) {
                commands.entity(zundamon_e).despawn();
                add(&mut commands, &game_assets, zundamon_t.translation.truncate(), zundamon_ball.radius, zundamon_v.linvel);
            }
        }
    }
}

