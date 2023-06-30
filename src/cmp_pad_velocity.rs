use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball::Ball;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct PadVelocity {
    pub size: Vec2,
    pub position: Vec2,
    pub velocity: Vec2
}

pub fn add(commands: &mut Commands,
                    game_assets: &Res<GameAsset>,
                    pad_velocity: PadVelocity) -> Entity {
    let size = pad_velocity.size;
    let pos = pad_velocity.position;
    let vel = pad_velocity.velocity;

    let sprite_handle = game_assets.image_handles.get("pad_velocity_handle").unwrap();
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                texture: sprite_handle.clone(),
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    let angle = Vec2::new(0.0, 1.0).angle_between(vel.normalize());
    entity
        .insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.0),
                    rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle),
                    ..Default::default()
                },
                ..default()
                })
        .insert(BBSize{x: size.x, y: size.y})
        .insert(pad_velocity);

    return entity.id();
}

pub fn system(
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(&mut Velocity, With<Ball>)>,
    pad_velocity_q: Query<(&Transform, &BBSize, &PadVelocity)>,
) {
    for (transform, bbsize, pad_velocity) in pad_velocity_q.iter(){
        let cuboid_size = Vec2::new(bbsize.x, bbsize.y) / 2.0 * transform.scale.truncate();
        let shape = Collider::cuboid(cuboid_size.x, cuboid_size.y);
        let shape_pos = transform.translation.truncate();
        let (shape_rot, _, _) = transform.rotation.to_euler(EulerRot::ZXY);
        let filter = QueryFilter::only_dynamic();

        rapier_context.intersections_with_shape(
            shape_pos, shape_rot, &shape, filter, |entity| {
                if let Ok(mut vel) = ball_q.get_mut(entity) {
                    vel.0.linvel = pad_velocity.velocity;
                }
            true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        });

    }
}

