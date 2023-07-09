use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use rand::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_zundamon_fullbody;
use crate::cmp_ball_zombie;

const BALL_SIZE: f32 = 16.0;
const START_TIME_SEC: f32 = 35.0;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct ConverterBody {
    pub size: Vec2,
    pub position: Vec2,
    pub remain: i32,
    pub prob: f32
}

pub fn add(commands: &mut Commands, converter_body: ConverterBody) -> Entity {
    let size = converter_body.size;
    let pos = converter_body.position;
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::None,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(converter_body);

    return entity.id();
}


pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(Entity, &Transform), With<Zundamon>>,
    mut cb_q: Query<(Entity, &Transform), With<ConverterBody>>,
) {
    for (ball_e, ball_t) in ball_q.iter() {
        for (cb_e, mut cb_t) in cb_q.iter_mut() {
            if rapier_context.intersection_pair(ball_e, cb_e) == Some(true) {
                commands.get_entity(ball_e).unwrap().despawn();
                cmp_zundamon_fullbody::add(commands, game_assets, pos, r, Vec2::ZERO);
            }
        }
    }
}

