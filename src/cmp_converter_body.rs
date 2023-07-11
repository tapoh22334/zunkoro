use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball::Zundamon;
use crate::cmp_ball::Ball;
use crate::cmp_zundamon_fullbody;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct ConverterBody {
    pub size: Vec2,
    pub position: Vec2,
}

pub fn add(commands: &mut Commands, converter_body: ConverterBody) -> Entity {
    let size = converter_body.size;
    let pos = converter_body.position;
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))
        .insert(Sensor)
        .insert(converter_body);

    return entity.id();
}


pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    rapier_context: Res<RapierContext>,
    ball_q: Query<(Entity, &Transform, &Ball), With<Zundamon>>,
    cb_q: Query<Entity, With<ConverterBody>>,
) {
    for (ball_e, ball_t, ball) in ball_q.iter() {
        for cb_e in cb_q.iter() {
            if rapier_context.intersection_pair(ball_e, cb_e) == Some(true) {
                commands.get_entity(ball_e).unwrap().despawn();
                cmp_zundamon_fullbody::add(&mut commands, &game_assets, ball_t.translation.truncate(), ball.radius * 2.0, Vec2::ZERO);
            }
        }
    }
}

