use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::cmp_bbsize::BBSize;

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

