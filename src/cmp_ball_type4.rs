use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_trajectory::Trajectory;
use crate::cmp_trajectory;
use crate::cmp_blood;
use crate::cmp_explosion;
use crate::cmp_ball;
use crate::cmp_ball::BallBundle;
use crate::cmp_ball::Ball;
use crate::cmp_combat::Status;
use crate::cmp_combat::Player1;
use crate::cmp_combat::Player2;
use crate::cmp_rotator::Rotator;

const RADIUS: f32 = 20.0;
const HP: f32 = 200.0;
const ATTACK: f32 = 1.0;
const ANGVEL: f32 = -7.0;
const EXPLOSION_RADIUS: f32 = 200.0;

#[derive(Component)]
pub struct BallType4;

#[derive(Bundle)]
pub struct BallType4P1Bundle {
    player1: Player1,
    ball_type: BallType4,
    status: Status,
    rotator: Rotator,
    #[bundle]
    ball_bundle: BallBundle,
}


impl From<(Vec2, Vec2, &GameAsset)> for BallType4P1Bundle {
    fn from(tuple: (Vec2, Vec2, &GameAsset)) -> Self {
        let (translation, velocity, game_assets) = tuple;

        let handle = game_assets.image_handles.get("bomb_handle").unwrap();
        let mut bundle = Self {
            player1: Player1,
            ball_type: BallType4,
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
pub struct BallType4P2Bundle {
    player2: Player2,
    ball_type: BallType4,
    status: Status,
    rotator: Rotator,
    #[bundle]
    ball_bundle: BallBundle,
}


impl From<(Vec2, Vec2, &GameAsset)> for BallType4P2Bundle {
    fn from(tuple: (Vec2, Vec2, &GameAsset)) -> Self {
        let (translation, velocity, game_assets) = tuple;

        let handle = game_assets.image_handles.get("bomb_handle").unwrap();
        let mut bundle = Self {
            player2: Player2,
            ball_type: BallType4,
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
    mut p1_q: Query<(Entity, &Status, &Transform, &BallType4), With<Player1>>,
    mut p2_q: Query<(Entity, &Status, &Transform, &BallType4), With<Player2>>,
) {
    let game_assets = game_assets.into_inner();

    for (e, s, t, ball) in p1_q.iter() {
        if s.hp <= 0.0 {
            commands.entity(e).despawn();
            //cmp_ball::kill(&mut commands, &audio, &game_assets, e, &t);
            commands.spawn(cmp_explosion::ExplosionBundle::from((t.translation, EXPLOSION_RADIUS, game_assets)))
                    .insert(Player1);
        }
    }

    for (e, s, t, ball) in p2_q.iter() {
        if s.hp <= 0.0 {
            commands.entity(e).despawn();
            commands.spawn(cmp_explosion::ExplosionBundle::from((t.translation, EXPLOSION_RADIUS, game_assets)))
                    .insert(Player2);
            //cmp_ball::kill(&mut commands, &audio, &game_assets, e, &t);
        }
    }
}
