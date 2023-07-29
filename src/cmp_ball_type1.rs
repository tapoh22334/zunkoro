use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_trajectory::Trajectory;
use crate::cmp_trajectory;
use crate::cmp_blood;
use crate::cmp_ball;
use crate::cmp_ball::BallBundle;
use crate::cmp_ball::Ball;
use crate::cmp_combat::Status;
use crate::cmp_combat::Player1;
use crate::cmp_combat::Player2;
use crate::cmp_rotator::Rotator;

const RADIUS: f32 = 20.0;
const HP: f32 = 10.0;
const ATTACK: f32 = 1.0;
const ANGVEL: f32 = -1.5;

#[derive(Bundle)]
pub struct BallP1Bundle {
    player1: Player1,
    status: Status,
    rotator: Rotator,
    #[bundle]
    ball_bundle: BallBundle,
}


impl From<(Vec2, Vec2, &GameAsset)> for BallP1Bundle {
    fn from(tuple: (Vec2, Vec2, &GameAsset)) -> Self {
        let (translation, velocity, game_assets) = tuple;

        let handle = game_assets.image_handles.get("zun1_handle").unwrap();
        let bundle = Self {
            player1: Player1,
            status: Status {
                hp: HP,
                attack: ATTACK,
            },
            rotator: Rotator {angvel: ANGVEL},
            ball_bundle: BallBundle::from((translation, RADIUS, velocity, handle.clone())),
        };

        bundle
    }
}


#[derive(Bundle)]
pub struct BallP2Bundle {
    player2: Player2,
    status: Status,
    rotator: Rotator,
    #[bundle]
    ball_bundle: BallBundle,
}


impl From<(Vec2, Vec2, &GameAsset)> for BallP2Bundle {
    fn from(tuple: (Vec2, Vec2, &GameAsset)) -> Self {
        let (translation, velocity, game_assets) = tuple;

        let handle = game_assets.image_handles.get("zombie1_handle").unwrap();
        let bundle = Self {
            player2: Player2,
            status: Status {
                hp: HP,
                attack: ATTACK,
            },
            rotator: Rotator {angvel: -ANGVEL},
            ball_bundle: BallBundle::from((translation, RADIUS, velocity, handle.clone())),
        };

        bundle
    }
}

