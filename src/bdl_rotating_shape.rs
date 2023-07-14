use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_primitive_shape::PrimitiveShape;
use crate::cmp_primitive_shape::PrimitiveShapeBundle;
use crate::cmp_rotator::Rotator;
use crate::ev_save_load_world::Derrived;

#[derive(Bundle)]
pub struct RotatingShapeBundle{
    pub rotator: Rotator,
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub derrived: Derrived,
    #[bundle]
    pub primitive_shape_bundle: PrimitiveShapeBundle,
}

impl Default for RotatingShapeBundle {
    fn default() -> Self {
        Self {
            rotator: Rotator::default(),
            velocity: Velocity::default(),
            derrived: Derrived,
            rigid_body: RigidBody::KinematicVelocityBased,
            primitive_shape_bundle: PrimitiveShapeBundle::default(),
        }
    }
}

impl From<(Rotator, PrimitiveShape)> for RotatingShapeBundle {
    fn from(vp: (Rotator, PrimitiveShape)) -> Self {
        let (rotator, primitive_shape) = vp;
        Self {
            rotator,
            primitive_shape_bundle: PrimitiveShapeBundle::from(primitive_shape),
            ..default()
        }
    }
}


const FILE_NAME: &str = "/rotating_shape.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(Rotator, PrimitiveShape)> = serde_json::from_str(&json_str).unwrap();

            for (v, p) in elem_list {
                commands.spawn(RotatingShapeBundle::from((v, p)));
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &Rotator, &PrimitiveShape)>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(Rotator, PrimitiveShape)> = vec![];

        for (t, vi, ps) in q.iter() {
            let mut ps = ps.clone();
            ps.position = t.translation.truncate();
            ps.scale = t.scale.truncate().x;
            ps.rotation = t.rotation;
            elem_list.push((vi.clone(), ps.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

