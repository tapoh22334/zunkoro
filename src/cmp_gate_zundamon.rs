use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use rand::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball;

const BALL_SIZE: f32 = 20.0;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateZundamon {
    pub size: Vec2,
    pub position: Vec2,
    pub remain: i32,
    pub prob: f32 }

pub fn add(commands: &mut Commands, gate_zundamon: GateZundamon) -> Entity {
    let size = gate_zundamon.size;
    let pos = gate_zundamon.position;
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(gate_zundamon);

    return entity.id();
}


pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut query: Query<(Entity, &Transform, &BBSize, &mut GateZundamon)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, transform, bbsize, mut gate_zundamon) in query.iter_mut() {
        if gate_zundamon.remain > 0 {
            if rng.gen::<f32>() < gate_zundamon.prob {
                let size = Vec2::new(bbsize.x, bbsize.y) * transform.scale.truncate();
                let pos_max = transform.translation.truncate() + (size / 2.0);
                let pos_min = transform.translation.truncate() - (size / 2.0);

                let x = rng.gen_range(pos_min.x .. pos_max.x);
                let y = rng.gen_range(pos_min.y .. pos_max.y);

                cmp_ball::add(&mut commands, &game_assets, Vec2::new(x, y), BALL_SIZE, Vec2::new(0.0, 0.0));
                gate_zundamon.remain -= 1;
            }
        } else {
            commands.get_entity(entity).unwrap().despawn();
        }
    }
}

