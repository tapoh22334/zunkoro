use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use rand::prelude::*;

use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball;

use crate::edit_context::*;

const BALL_SIZE: f32 = 10.0;
pub const DEFAULT_SIZE_X: f32 = 64.0;
pub const DEFAULT_SIZE_Y: f32 = 64.0;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateGeneric {
    pub remain: i32,
    pub prob: f32,
    pub ball_radius: f32,
}


#[derive(Bundle)]
pub struct GateGenericBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    bbsize: BBSize,
    gate_generic: GateGeneric,
}


impl Default for GateGenericBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(DEFAULT_SIZE_X, DEFAULT_SIZE_Y)),
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::ONE),
                ..Default::default()
            },
            bbsize: BBSize {x: DEFAULT_SIZE_X, y: DEFAULT_SIZE_Y},
            gate_generic: GateGeneric {
                remain: 1,
                prob: 1.0,
                ball_radius: BALL_SIZE,
            }

        }
    }
}


impl From<(Vec3, Quat, Vec3, GateGeneric)> for GateGenericBundle {
    fn from(tuple: (Vec3, Quat, Vec3, GateGeneric)) -> Self {
        let (translation, rotation, scale, gate_generic) = tuple;

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..default()
            },
            gate_generic,
            ..default()
        }
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
        if let MapObject::GateGeneric = map_object {
            let entity = commands.spawn(GateGenericBundle {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::from((world_position.translation, 0.0)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });

            *edit_context = EditContext::Edit(vec![entity.id()], EditTool::Select);
        }
    }
}

pub fn system_setup(
    mut query: Query<&mut Sprite>,
    ) {

    for mut sprite in query.iter_mut() {
        let color = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0, };
        sprite.color = color;
    }

}

pub fn system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    mut query: Query<(Entity, &Transform, &BBSize, &mut GateGeneric)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, transform, bbsize, mut gate_generic) in query.iter_mut() {
        if gate_generic.remain > 0 {
            if rng.gen::<f32>() < gate_generic.prob {
                let size = Vec2::new(bbsize.x, bbsize.y) * transform.scale.truncate();
                let pos_max = transform.translation.truncate() + (size / 2.0);
                let pos_min = transform.translation.truncate() - (size / 2.0);

                let x = rng.gen_range(pos_min.x .. pos_max.x);
                let y = rng.gen_range(pos_min.y .. pos_max.y);

                cmp_ball::add(&mut commands, &game_assets, Vec2::new(x, y), gate_generic.ball_radius, Vec2::new(0.0, 0.0));
                gate_generic.remain -= 1;
            }
        } else {
            commands.get_entity(entity).unwrap().despawn();
        }
    }
}


const FILE_NAME: &str = "/gate_generic.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, Vec3, Quat, Vec3, GateGeneric)> = serde_json::from_str(&json_str).unwrap();

            for (i, t, r, s, gg) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(i));
                entity.insert(GateGenericBundle::from((t, r, s, gg)));
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &Transform, &GateGeneric)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, Vec3, Quat, Vec3, GateGeneric)> = vec![];

        for (e, t, gg) in q.iter() {
            elem_list.push((e.index(), t.translation, t.rotation, t.scale, gg.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

