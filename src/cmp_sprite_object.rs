use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_ball_zundamon::Zundamon;
use crate::cmp_ball_zombie::Zombie;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball_zundamon;
use crate::cmp_ball_zombie;

use crate::cmp_ball_type1;
use crate::cmp_ball_type2;
use crate::cmp_ball_type3;
use crate::cmp_ball_type4;

use crate::cmp_rotator::Rotator;
use crate::edit_context::*;

pub const DEFAULT_SIZE_X: f32 = 256.0;
pub const DEFAULT_SIZE_Y: f32 = 256.0;


#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct SpriteObject {
    pub handle: String,
}


#[derive(Bundle)]
pub struct SpriteObjectBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    bbsize: BBSize,
    sprite_object: SpriteObject,
    map_object: MapObject,
}


impl Default for SpriteObjectBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_scale(Vec3::ONE),
                ..Default::default()
            },
            bbsize: BBSize {x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            sprite_object: SpriteObject { handle: String::from("") },
            map_object: MapObject::SpriteObject(String::from("")),
        }
    }
}

impl From<(Vec3, Quat, Vec3, SpriteObject, &GameAsset)> for SpriteObjectBundle {
    fn from(tuple: (Vec3, Quat, Vec3, SpriteObject, &GameAsset)) -> Self {
        let (translation, rotation, scale, sprite_object, game_assets) = tuple;
        let handle = game_assets.image_handles.get(&sprite_object.handle).unwrap();

        let mut bundle = SpriteObjectBundle::default();

        bundle.sprite_bundle.transform = Transform {
                    translation,
                    rotation,
                    scale,
                };
        bundle.sprite_bundle.texture = handle.to_owned();
        bundle.sprite_object = SpriteObject{ handle: sprite_object.handle };

        bundle
    }
}


pub fn handle_user_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    game_assets: Res<GameAsset>,
    mut edit_context: ResMut<EditContext>,
    world_position: ResMut<WorldPosition>,
    ) {
    let game_assets = game_assets.into_inner();

    if ! buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let EditContext::Spawn(map_object) = edit_context.clone() {
        if let MapObject::SpriteObject(handle) = map_object {

            let mut entity = commands.spawn(
                SpriteObjectBundle::from((Vec3::from((world_position.translation, 0.0)),
                                        Quat::default(),
                                        Vec3::ONE,
                                        SpriteObject{handle},
                                        game_assets
                                        ))
                );

            *edit_context = EditContext::Edit(MapObject::SpriteObject("".to_string()), vec![entity.id()], EditTool::Select);
        }
    }
}

const FILE_NAME: &str = "/sprite_object.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut load_world_er: EventReader<LoadWorldEvent>,
    ) {
    let game_assets = game_assets.into_inner();

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, SpriteObject)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, so) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(SpriteObjectBundle::from((t, r, s, so, game_assets)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &SpriteObject)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, SpriteObject)> = vec![];

        for (e, t, so) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, so.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

