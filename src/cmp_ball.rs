use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::cmp_game_asset::GameAsset;
use rand::prelude::*;

#[derive(Component)]
pub struct Ball;

pub fn add(commands: &mut Commands, game_assets: &Res<GameAsset>, pos: Vec2, r: f32, vel: Vec2) {
    let mut rng = rand::thread_rng();
    let image_vec = vec![ "zun1_handle", "zun2_handle", "zun3_handle" ];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    let sprite_handle = game_assets.image_handles.get(random_image).unwrap();
    //let sprite_image = image_assets.get(sprite_handle).unwrap();
    //let collider = single_convex_hull_collider_translated(sprite_image).unwrap();
    let collider = Collider::ball(r);

    commands
        .spawn(Ball)
        .insert(RigidBody::Dynamic)
        .insert(Restitution::coefficient(0.7))
        .insert(Friction::coefficient(0.05))
        .insert(collider)
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




