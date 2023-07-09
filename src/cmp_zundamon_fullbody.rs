use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_trajectory::Trajectory;
use crate::cmp_trajectory;
use crate::cmp_blood;

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub previous_position: Option<Vec2>,
}

#[derive(Component)]
pub struct ZundamonFullbody;

const ASPECT_RATIO: f32 = 1082.0 / 1650.0;

pub fn add(commands: &mut Commands, game_assets: &Res<GameAsset>, pos: Vec2, r: f32, vel: Vec2) {
    let mut rng = rand::thread_rng();
    let image_vec = vec![ "zun1_full_handle", "zun2_full_handle", "zun3_full_handle" ];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    let sprite_handle = game_assets.image_handles.get(random_image).unwrap();

    commands
        .spawn(ZundamonFullbody)
        //.insert(Ball {radius: r, previous_position: None})
        //.insert(RigidBody::Dynamic)
        //.insert(Restitution::coefficient(0.1))
        //.insert(Friction::coefficient(0.02))
        //.insert(Collider::ball(r))
        //.insert(CollisionGroups::new(Group::GROUP_1, Group::ALL))
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
        //.insert(Velocity {
        //    linvel: vel,
        //    angvel: 0.0,
        //})
    ;
}

