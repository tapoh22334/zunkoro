use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Artillery {
    pub scale: f32,
    pub position: Vec2,
    pub angvel: f32,
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
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    let colliders = multi_polyline_collider_translated(sprite_image);

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
        .insert(TransformBundle {
            local: Transform {
                translation: Vec3::new(pos.x, pos.y, 0.0),
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
        .insert(artillery);

    for collider in colliders {
        entity.with_children(|children| {
            children.spawn(collider)
                .insert(TransformBundle {
                    local: Transform {
                        ..Default::default()
                    },
                    ..default()
                }) ;
        });
    }

    // Fragment2
    entity.with_children(|children| {
        let sprite_handle = game_assets.image_handles.get("artillery_frag2").unwrap();
        let sprite_image = image_assets.get(sprite_handle).unwrap();
        let colliders = multi_polyline_collider_translated(sprite_image);
        let mut child = children.spawn(Barrel);
        child.with_children(|child2| {
            let mut child2 = child2.spawn(
                SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: sprite_handle.clone(),
                ..default()
            },);

            for collider in colliders {
                child2.with_children(|child3| {
                    child3.spawn(collider)
                        .insert(TransformBundle {
                            local: Transform {
                                ..Default::default()
                            },
                            ..default()
                        });
                });
            }

            child2.insert(
                TransformBundle::from(Transform::from_xyz(256.0, 0.0, 0.0))
                );
        });
        child.insert(RigidBody::KinematicVelocityBased);
        child.insert(
            TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0))
            )
            .insert(Velocity {
                linvel: Vec2::ZERO,
                angvel,
            });
    });

    return entity.id();
}


pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut artillery_frag1: Query<(&Transform, &mut Artillery)>,
    mut barrels: Query<(&Parent, &Transform, &Velocity, &Barrel)>,
) {
}

