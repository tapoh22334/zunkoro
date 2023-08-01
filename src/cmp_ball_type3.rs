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

const RADIUS: f32 = 80.0;
const HP: f32 = 1000.0;
const ATTACK: f32 = 30.0;
const ANGVEL: f32 = -1.5;

#[derive(Component)]
pub struct BallType3;

#[derive(Bundle)]
pub struct BallType3P1Bundle {
    player1: Player1,
    ball_type: BallType3,
    status: Status,
    rotator: Rotator,
    #[bundle]
    ball_bundle: BallBundle,
}


impl From<(Vec2, Vec2, &GameAsset)> for BallType3P1Bundle {
    fn from(tuple: (Vec2, Vec2, &GameAsset)) -> Self {
        let (translation, velocity, game_assets) = tuple;

        let handle = game_assets.image_handles.get("zun1_handle").unwrap();
        let mut bundle = Self {
            player1: Player1,
            ball_type: BallType3,
            status: Status {
                hp: HP,
                hp_max: HP,
                attack: ATTACK,
            },
            rotator: Rotator {angvel: ANGVEL},
            ball_bundle: BallBundle::from((translation, RADIUS, velocity, handle.clone())),
        };

        bundle.ball_bundle.collision_groups = CollisionGroups::new(Group::GROUP_10, Group::GROUP_1 | Group::GROUP_11);

        bundle
    }
}


#[derive(Bundle)]
pub struct BallType3P2Bundle {
    player2: Player2,
    ball_type: BallType3,
    status: Status,
    rotator: Rotator,
    #[bundle]
    ball_bundle: BallBundle,
}


impl From<(Vec2, Vec2, &GameAsset)> for BallType3P2Bundle {
    fn from(tuple: (Vec2, Vec2, &GameAsset)) -> Self {
        let (translation, velocity, game_assets) = tuple;

        let handle = game_assets.image_handles.get("zombie1_handle").unwrap();
        let mut bundle = Self {
            player2: Player2,
            ball_type: BallType3,
            status: Status {
                hp: HP,
                hp_max: HP,
                attack: ATTACK,
            },
            rotator: Rotator {angvel: -ANGVEL},
            ball_bundle: BallBundle::from((translation, RADIUS, velocity, handle.clone())),
        };

        bundle.ball_bundle.collision_groups = CollisionGroups::new(Group::GROUP_11, Group::GROUP_1 | Group::GROUP_10);

        bundle
    }
}


pub fn system(
    mut commands: Commands,
    audio: Res<Audio>,
    game_assets: Res<GameAsset>,
    mut query: Query<(Entity, &Status, &Transform, &BallType3), Or<(With<Player1>, With<Player2>)>>,
) {
    for (e, s, t, ball) in query.iter() {
        if s.hp <= 0.0 {
            cmp_ball::kill(&mut commands, &audio, &game_assets, e, &t);
        }
    }
}
