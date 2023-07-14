use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball::Ball;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Vibrator {
    pub direction: Direction,
    pub speed: f32,
    pub range: (f32, f32),
}

pub fn system(
    mut q: Query<(&mut Transform, &mut Velocity, VerticalVibrator)>,
) {
    for (mut t, mut v, vv) in q.iter_mut() {
        match (vv.direction) {
            Vertical => {
                if t.translation.y <= vv.range.0 {
                    v.linvel.y = vv.speed;
                } else if t.translation.y >= vv.range.1 {
                    v.linvel.y = -vv.speed;
                }
            }

            Horizontal => {
                if t.translation.x <= vv.range.0 {
                    v.linvel.x = vv.speed;
                } else if t.translation.x >= vv.range.1 {
                    v.linvel.x = -vv.speed;
                }
            }

        }

    }
}

