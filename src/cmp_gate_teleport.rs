use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::cmp_ball::Ball;
use crate::cmp_bbsize::BBSize;


#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateTeleportEntrance{
    pub id: u32,
    pub size: Vec2,
    pub position: Vec2,
    pub color: Color,
    }

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateTeleportExit{
    pub id: u32,
    pub size: Vec2,
    pub position: Vec2,
    pub color: Color,
    }

pub fn add_entrance(commands: &mut Commands, gate_teleport: GateTeleportEntrance) -> Entity {
    let size = gate_teleport.size;
    let pos = gate_teleport.position;
    let color = gate_teleport.color;
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(gate_teleport);

    return entity.id();
}

pub fn add_exit(commands: &mut Commands, gate_teleport: GateTeleportExit) -> Entity {
    let size = gate_teleport.size;
    let pos = gate_teleport.position;
    let color = gate_teleport.color;

    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(gate_teleport);

    return entity.id();
}


pub fn system(
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut entrance_q: Query<(&Transform, &BBSize, &GateTeleportEntrance), Without<Ball>>,
    exit_q: Query<(&Transform, &GateTeleportExit), Without<Ball>>,
) {
    for (transform, bbsize, gate_teleport) in entrance_q.iter_mut() {
        let cuboid_size = Vec2::new(bbsize.x, bbsize.y) / 2.0 * transform.scale.truncate();
        let shape = Collider::cuboid(cuboid_size.x, cuboid_size.y);
        let shape_pos = transform.translation.truncate();
        let (shape_rot, _, _) = transform.rotation.to_euler(EulerRot::ZXY);
        let filter = QueryFilter::only_dynamic()
                        .groups(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));

        rapier_context.intersections_with_shape(
            shape_pos, shape_rot, &shape, filter, |entity| {
                if let Ok((mut ball_transform, mut ball_velocity)) = ball_q.get_mut(entity) {
                    for (exit_transform, exit_gate_teleport) in exit_q.iter() {
                        if exit_gate_teleport.id == gate_teleport.id {
                            ball_transform.translation = exit_transform.translation;
                            ball_velocity.linvel = Vec2::ZERO;
                            break;
                        }
                    }
                }
            true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        });
    }
}

const FILE_NAME_ENTRANCE: &str = "/gate_teleport_entrance.map";
const FILE_NAME_EXIT: &str = "/gate_teleport_exit.map";

use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        {
            let dir = e.0.clone();

            let json_str = std::fs::read_to_string(dir + FILE_NAME_ENTRANCE);
            if let Ok(json_str) = json_str {
                let elem_list: Vec<GateTeleportEntrance> = serde_json::from_str(&json_str).unwrap();

                for e in elem_list {
                    add_entrance(&mut commands, e);
                }
            }
        }
        {
            let dir = e.0.clone();

            let json_str = std::fs::read_to_string(dir + FILE_NAME_EXIT);
            if let Ok(json_str) = json_str {
                let elem_list: Vec<GateTeleportExit> = serde_json::from_str(&json_str).unwrap();

                for e in elem_list {
                    add_exit(&mut commands, e);
                }
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              gate_teleport_entrance: Query<(&Transform, &GateTeleportEntrance)>,
              gate_teleport_exit: Query<(&Transform, &GateTeleportExit)>,
              ) {
    for e in save_world_er.iter() {
        {
            let dir = e.0.clone();
            let mut elem_list: Vec<GateTeleportEntrance> = vec![];

            for (t, e) in gate_teleport_entrance.iter() {
                let mut e = e.clone();
                e.size = e.size * t.scale.truncate();
                e.position = t.translation.truncate();
                elem_list.push(e.clone());
            }

            std::fs::write(dir + FILE_NAME_ENTRANCE, serde_json::to_string(&elem_list).unwrap()).unwrap();
        }

        {
            let dir = e.0.clone();
            let mut elem_list: Vec<GateTeleportExit> = vec![];

            for (t, e) in gate_teleport_exit.iter() {
                let mut e = e.clone();
                e.size = e.size * t.scale.truncate();
                e.position = t.translation.truncate();
                elem_list.push(e.clone());
            }

            std::fs::write(dir + FILE_NAME_EXIT, serde_json::to_string(&elem_list).unwrap()).unwrap();
        }
    }
}

