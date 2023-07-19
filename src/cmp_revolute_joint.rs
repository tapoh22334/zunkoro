use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct RevoluteJoint {
    pub translation: Vec3,
}

impl From for RevoluteJoint
{
    fn from(revolute_joint: RevoluteJoint) -> Self {
        let joint = RevoluteJointBuilder::new()
            .local_anchor1(Vec2::new(0.0, 0.0))
            .local_anchor2(Vec2::new(0.0, 0.0));

        let base_entity = commands.spawn(RigidBody::Dynamic)
            .insert(TransformBundle {
                local: Transform {
                    translation: revolute_joint.translation,
                    ..Default::default()
                },
                ..default()
            })
        .id();

        Self {
            impulse_joint: ImpulseJoint::new(base_entity, joint),
        }
    }
}


const FILE_NAME: &str = "/revolute_joint.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    )
{

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let filename = dir + FILE_NAME;
        let json_str = std::fs::read_to_string(filename);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, RevoluteJoint)> = serde_json::from_str(&json_str).unwrap();

            for (id, v) in elem_list {
                let mut entity = commands.get_or_spawn(Entity::from_raw(id));
                entity.insert(v);
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(Entity, &RevoluteJoint)>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(u32, RevoluteJoint)> = vec![];

        for (e, rt) in q.iter() {
            elem_list.push((e.index(), rt.clone()));
        }

        let filename = dir + FILE_NAME;
        std::fs::write(filename, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

