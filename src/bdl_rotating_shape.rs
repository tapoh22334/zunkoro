//use bevy::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_rotator::Rotator;
use crate::ev_save_load_world::Derrived;

#[derive(Bundle)]
pub struct RotatingShapeAttachmentBundle {
    pub rotator: Rotator,
    pub derrived: Derrived,
}

impl Default for RotatingShapeAttachmentBundle {
    fn default() -> Self {
        Self {
            rotator: Rotator::default(),
            derrived: Derrived,
        }
    }
}


impl From<Rotator> for RotatingShapeAttachmentBundle
{
    fn from(rotator: Rotator) -> Self {
        Self {
            rotator,
            ..Default::default()
        }
    }
}


#[derive(Bundle)]
pub struct RotatingShapeBundle<T: Sync + Send + bevy::prelude::Bundle>{
    #[bundle]
    pub vsa_bundle: RotatingShapeAttachmentBundle,
    #[bundle]
    pub shape_bundle: T,
}


impl<T: bevy::prelude::Bundle + Default> Default for RotatingShapeBundle<T> {
    fn default() -> Self {
        Self {
            vsa_bundle: RotatingShapeAttachmentBundle::default(),
            shape_bundle: T::default(),
        }
    }
}


impl<T1: bevy::prelude::Bundle + Default, T2> From<(Vec3, Quat, Vec3, T2, Rotator)> for RotatingShapeBundle<T1>
where
    T1: From<(Vec3, Quat, Vec3, T2)>,
{
    fn from(tuple: (Vec3, Quat, Vec3, T2, Rotator)) -> Self {
        let (t, r, s, shape, rotator) = tuple;
        Self {
            vsa_bundle: RotatingShapeAttachmentBundle::from(rotator),
            shape_bundle: T1::from((t, r, s, shape)),
            ..Default::default()
        }
    }
}


const FILE_NAME: &str = "/rotating_shape.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load<T1: bevy::prelude::Bundle + Default, T2: DeserializeOwned + Sync + Send + bevy::prelude::Component>(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    )
where
    T1: From<(Vec3, Quat, Vec3, T2)>,
{

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(Vec3, Quat, Vec3, T2, Rotator)> = serde_json::from_str(&json_str).unwrap();

            for (t, r, s, p, v) in elem_list {
                commands.spawn(RotatingShapeBundle::<T1>::from((t, r, s, p, v)));
            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save<T: bevy::prelude::Component + Clone + Serialize>(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &Rotator, &T)>
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<(Vec3, Quat, Vec3, T, Rotator)> = vec![];

        for (t, vi, ps) in q.iter() {
            elem_list.push((t.translation, t.rotation, t.scale, ps.clone(), vi.clone()));
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

