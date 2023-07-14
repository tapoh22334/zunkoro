use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Rotator {
    pub angvel: f32,
}

pub fn system(
    mut q: Query<(&mut Transform, &mut Velocity, &Rotator)>,
) {
    println!("set angvel");
    for (t, mut v, r) in q.iter_mut() {
        if v.angvel == 0.0 {
            v.angvel = r.angvel;
        }
    }
}

