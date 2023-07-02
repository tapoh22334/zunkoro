use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GearSimple {
    pub scale: f32,
    pub position: Vec2,
    pub anglevel: f32
}

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GearSorting {
    pub scale: f32,
    pub position: Vec2,
    pub anglevel: f32
}


pub fn add_simple(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            gear_simple: GearSimple) -> Entity {

    let id = add_gear(commands,
             game_assets,
             image_assets,
             "gear_simple_512",
             gear_simple.position,
             gear_simple.scale,
             gear_simple.anglevel);

    let id = commands.entity(id).insert(gear_simple).id();
    return id;
}

pub fn add_sorting(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            gear_sorting: GearSorting) -> Entity {

    let id = add_gear(commands,
             game_assets,
             image_assets,
             "gear_sorting_512",
             gear_sorting.position,
             gear_sorting.scale,
             gear_sorting.anglevel);

    let id = commands.entity(id).insert(gear_sorting).id();
    return id;
}

fn add_gear(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            name: &str,
            pos: Vec2,
            r: f32,
            anglevel: f32) -> Entity {
    let sprite_handle = game_assets.image_handles.get(name).unwrap();
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
        .insert(Interaction::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Restitution::coefficient(0.5))
        .insert(Friction::coefficient(0.3))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: anglevel,
        });

    for collider in colliders {
        entity.with_children(|children| {
            children.spawn(collider)
                .insert(TransformBundle {
                    local: Transform {
                        ..Default::default()
                    },
                    ..default()
                })
            ;
        });
    }

    entity.insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.0),
                    scale: r * Vec3::ONE,
                    ..Default::default()
                },
                ..default()
                },
        );

    entity.insert(BBSize{x: 512.0, y: 512.0});

    return entity.id();

}
