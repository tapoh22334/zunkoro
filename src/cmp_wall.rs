use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::constants;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;

use crate::cmp_rotator::Rotator;
use crate::edit_context::*;

const BALL_SIZE: f32 = 10.0;
pub const DEFAULT_SIZE_X: f32 = 30.0;
pub const DEFAULT_SIZE_Y: f32 = 400.0;


#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Wall;


#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub bbsize: BBSize,
    pub collider: Collider,
    pub restitution: Restitution,
    pub friction: Friction,
    pub rigid_body: RigidBody,
    pub map_object: MapObject,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}


impl Default for WallBundle {
    fn default() -> Self {
        Self {
            bbsize: BBSize {x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            wall: Wall,
            collider: Collider::cuboid(DEFAULT_SIZE_X / 2.0, DEFAULT_SIZE_Y / 2.0),
            restitution: Restitution::coefficient(constants::C_MAP_RESTITUTION),
            friction: Friction::coefficient(constants::C_MAP_FRICTION),
            rigid_body: RigidBody::KinematicVelocityBased,
            map_object: MapObject::Wall,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(DEFAULT_SIZE_X, DEFAULT_SIZE_Y)),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::ONE),
                ..Default::default()
            },
        }
    }
}

impl From<(Vec3, Quat, Vec3)> for WallBundle {
    fn from(tuple: (Vec3, Quat, Vec3)) -> Self {
        let (translation, rotation, scale) = tuple;

        let mut bundle = WallBundle::default();

        bundle.sprite_bundle.transform = Transform {
                    translation,
                    rotation,
                    scale,
                };

        bundle
    }
}

impl From<(Vec3, Quat, Vec3, Wall)> for WallBundle {
    fn from(tuple: (Vec3, Quat, Vec3, Wall)) -> Self {
        let (translation, rotation, scale, _) = tuple;
        WallBundle::from((translation, rotation, scale))
    }
}

pub fn handle_user_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut edit_context: ResMut<EditContext>,
    world_position: ResMut<WorldPosition>,
    ) {

    if ! buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let EditContext::Spawn(map_object) = edit_context.clone() {
        if let MapObject::Wall = map_object {

            let mut entity = commands.spawn(
                WallBundle::from((Vec3::from((world_position.translation, 0.0)),
                                        Quat::from_rotation_z(0.0),
                                        Vec3::ONE))
                );

            *edit_context = EditContext::Edit(MapObject::Wall, vec![entity.id()], EditTool::Select);
        }
    }

}

//pub fn system_setup(
//    mut query: Query<&mut Sprite, With<Wall>>,
//    ) {
//
//    for mut sprite in query.iter_mut() {
//        let color = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0, };
//        sprite.color = color;
//    }
//
//}

use crate::ev_despawn;
pub fn despawn(
    mut commands: Commands,
    mut event: EventReader<ev_despawn::Despawn>,
    query: Query<&Wall>,
    ) {
    for ev_despawn::Despawn(entity) in event.iter() {
        println!("despawn1");
        if query.contains(Entity::from_raw(entity.to_owned())) {
            println!("despawn2");
            commands.entity(Entity::from_raw(entity.to_owned())).despawn();
        }
    }
}

const FILE_NAME: &str = "/wall.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, Wall)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, w) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(WallBundle::from((t, r, s, w)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &Wall)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, Wall)> = vec![];

        for (e, t, w) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, w.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

