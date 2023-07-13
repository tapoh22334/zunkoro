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

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GearSwirl {
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

pub fn add_swirl(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            gear_swirl: GearSwirl) -> Entity {

    let id = add_gear(commands,
             game_assets,
             image_assets,
             "gear_swirl_512",
             gear_swirl.position,
             gear_swirl.scale,
             gear_swirl.anglevel);

    let id = commands.entity(id).insert(gear_swirl).id();
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


const FILE_NAME_SIMPLE: &str = "/gear_simple.map";
const FILE_NAME_SORTING: &str = "/gear_SORTING.map";
const FILE_NAME_SWIRL: &str = "/gear_SWIRL.map";

use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
    ) {

    for e in load_world_er.iter() {
        {
            let dir = e.0.clone();
            let json_str = std::fs::read_to_string(dir + FILE_NAME_SIMPLE);
            if let Ok(json_str) = json_str {
                let elem_list: Vec<GearSimple> = serde_json::from_str(&json_str).unwrap();

                for e in elem_list {
                    add_simple(&mut commands, &game_assets, &image_assets, e);
                }
            }
        }

        {
            let dir = e.0.clone();
            let json_str = std::fs::read_to_string(dir + FILE_NAME_SORTING);
            if let Ok(json_str) = json_str {
                let elem_list: Vec<GearSorting> = serde_json::from_str(&json_str).unwrap();

                for e in elem_list {
                    add_sorting(&mut commands, &game_assets, &image_assets, e);
                }
            }
        }

        {
            let dir = e.0.clone();
            let json_str = std::fs::read_to_string(dir + FILE_NAME_SWIRL);
            if let Ok(json_str) = json_str {
                let elem_list: Vec<GearSwirl> = serde_json::from_str(&json_str).unwrap();

                for e in elem_list {
                    add_swirl(&mut commands, &game_assets, &image_assets, e);
                }
            }
        }
    }
}



use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              gear_simple_q: Query<(&Velocity, &Transform, &GearSimple)>,
              gear_sorting_q: Query<(&Velocity, &Transform, &GearSorting)>,
              gear_swirl_q: Query<(&Velocity, &Transform, &GearSwirl)>,
              ) {
    for e in save_world_er.iter() {

        {
            // GearSimple
            let dir = e.0.clone();
            let mut elem_list: Vec<GearSimple> = vec![];
            for (v, t, e) in gear_simple_q.iter() {
                let mut e = e.clone();
                e.scale = t.scale.truncate().x;
                e.position = t.translation.truncate();
                e.anglevel = v.angvel;
                elem_list.push(e.clone());
            }
            std::fs::write(dir + FILE_NAME_SIMPLE, serde_json::to_string(&elem_list).unwrap()).unwrap();
        }

        {
            // GearSorting
            let dir = e.0.clone();
            let mut elem_list: Vec<GearSorting> = vec![];
            for (v, t, e) in gear_sorting_q.iter() {
                let mut e = e.clone();
                e.scale = t.scale.truncate().x;
                e.position = t.translation.truncate();
                e.anglevel = v.angvel;
                elem_list.push(e.clone());
            }
            std::fs::write(dir + FILE_NAME_SORTING, serde_json::to_string(&elem_list).unwrap()).unwrap();
        }

        {
            // GearSwirl
            let dir = e.0.clone();
            let mut elem_list: Vec<GearSwirl> = vec![];
            for (v, t, e) in gear_swirl_q.iter() {
                let mut e = e.clone();
                e.scale = t.scale.truncate().x;
                e.position = t.translation.truncate();
                e.anglevel = v.angvel;
                elem_list.push(e.clone());
            }
            std::fs::write(dir + FILE_NAME_SWIRL, serde_json::to_string(&elem_list).unwrap()).unwrap();
        }
    }
}

