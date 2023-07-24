use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::edit_context::*;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct RevoluteJoint {
    pub child_entity: Entity,
    pub translation: Vec3,
    pub limits: [Real; 2],
}

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct RevoluteJointBase;

pub fn handle_user_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut edit_context: ResMut<EditContext>,
    mut transform_q: Query<(Entity, &mut Transform, &mut RigidBody)>,
    ) {
    if let EditContext::Spawn(map_object) = edit_context.clone() {
        if let MapObject::RevoluteJoint(entity) = map_object {
            if buttons.just_pressed(MouseButton::Left) {
                let (entity, transform, mut rigid_body) = transform_q.get_mut(entity).unwrap();

                let entity = add(&mut commands,
                                 &mut rigid_body,
                                 RevoluteJoint { child_entity: entity, translation: transform.translation, limits: [0.0, 0.0] });

                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
            }

        }
    }
}


pub fn system(
    //mut rj_q: Query<(&Transform, &mut ImpulseJoint, &RevoluteJoint)>,
    //mut rjb_q: Query<&mut Transform, (With<RevoluteJointBase>, Without<RevoluteJoint>)>,
) {
    //for (t, mut ij, rj) in rj_q.iter_mut() {
    //    let mut parent_t = rjb_q.get_mut(ij.parent).unwrap();

    //    println!("{:?}", parent_t);
    //    println!("{:?}", t);
    //    if *parent_t != *t {
    //        *parent_t = *t;
    //    }
    //}
}


fn add(commands: &mut Commands, rigid_body: &mut RigidBody, revolute_joint: RevoluteJoint) -> Entity {
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
        .insert(RevoluteJointBase)
    .id();

    let mut entity = commands.get_entity(revolute_joint.child_entity).unwrap();
    entity
        .insert(ImpulseJoint::new(base_entity, joint))
        .insert(revolute_joint);

    *rigid_body = RigidBody::Dynamic;

    return entity.id();
}


use crate::ev_save_load_world::LoadWorldEvent;
const FILE_NAME: &str = "/revolute_joint.map";
pub struct DelayLoadRevoluteJoint(pub String);
pub fn delay_load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut load_world_ew: EventWriter<DelayLoadRevoluteJoint>,
    )
{
    for e in load_world_er.iter() {
        let dir = e.0.clone();
        load_world_ew.send(DelayLoadRevoluteJoint(dir));
    }
}

pub fn load(
    mut load_world_er: EventReader<DelayLoadRevoluteJoint>,
    mut commands: Commands,
    mut q: Query<&mut RigidBody>,
    )
{
    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let filename = dir + FILE_NAME;
        let json_str = std::fs::read_to_string(filename);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, RevoluteJoint)> = serde_json::from_str(&json_str).unwrap();

            for (id, v) in elem_list {
                println!("load revolute joint{:?}", id);
                let entity = commands.get_or_spawn(Entity::from_raw(id)).id();
                let mut rigid_body = q.get_mut(entity).unwrap();

                add(&mut commands,
                    &mut rigid_body,
                    RevoluteJoint { child_entity: entity,
                                    translation: v.translation,
                                    limits: v.limits });
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

