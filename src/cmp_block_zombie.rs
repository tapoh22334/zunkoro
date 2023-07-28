use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_bbsize::BBSize;
use crate::cmp_ball_zundamon;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct BlockZombie {
    pub size: Vec2,
    pub position: Vec2,
    }

pub fn add(commands: &mut Commands, block_zombie: BlockZombie) -> Entity {
    let size = block_zombie.size;
    let pos = block_zombie.position;
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))
        .insert(CollisionGroups::new(Group::ALL, Group::GROUP_2))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(block_zombie);

    return entity.id();
}

const FILE_NAME: &str = "/block_zombie.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<BlockZombie> = serde_json::from_str(&json_str).unwrap();

            for e in elem_list {
                add(&mut commands, e);
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &BlockZombie)>,) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<BlockZombie> = vec![];

        for (t, e) in q.iter() {
            let mut e = e.clone();
            e.size = e.size * t.scale.truncate();
            e.position = t.translation.truncate();
            elem_list.push(e.clone());
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

