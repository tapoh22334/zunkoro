use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Default, Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Vibrator {
    pub direction: Direction,
    pub speed: f32,
    pub range: (f32, f32),
}

pub fn system(
    mut q: Query<(&mut Transform, &mut Velocity, &Vibrator)>,
) {
    for (t, mut v, vb) in q.iter_mut() {
        match vb.direction {
            Direction::Vertical => {
                if t.translation.y <= vb.range.0 {
                    v.linvel.y = vb.speed;
                } else if t.translation.y >= vb.range.1 {
                    v.linvel.y = -vb.speed;
                }
            }

            Direction::Horizontal => {
                if t.translation.x <= vb.range.0 {
                    v.linvel.x = vb.speed;
                } else if t.translation.x >= vb.range.1 {
                    v.linvel.x = -vb.speed;
                }
            }

        }

    }
}

