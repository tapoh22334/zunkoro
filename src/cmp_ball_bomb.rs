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
use crate::cmp_explosion::ExplosionBundle;
use crate::cmp_rotator::Rotator;

const RADIUS: f32 = 20.0;
const HP: f32 = 1.0;
const ATTACK: f32 = 1.0;
const ANGVEL: f32 = -5.0;
const EXPLOSION_RADIUS: f32 = 100.0;

#[derive(Component)]
pub struct BallBomb;

#[derive(Bundle)]
pub struct BallBombBundle<T: Component + Default> {
    player: T,
    ball_type: BallBomb,
    collision_events: ActiveEvents,
    #[bundle]
    ball_bundle: BallBundle,
}

impl<T: Component + Default> From<(Vec2, Vec2, &GameAsset)> for BallBombBundle<T> {
    fn from(tuple: (Vec2, Vec2, &GameAsset)) -> Self {
        let (translation, velocity, game_assets) = tuple;

        let handle = game_assets.image_handles.get("bomb_handle").unwrap();
        let mut bundle = Self {
            player: T::default(),
            ball_type: BallBomb,
            collision_events: ActiveEvents::COLLISION_EVENTS,
            ball_bundle: BallBundle::from((translation, RADIUS, velocity, handle.clone())),
        };
        bundle.ball_bundle.collision_groups = CollisionGroups::new(Group::GROUP_1, Group::GROUP_1);

        bundle
    }
}

pub fn system<T: Component + Default>(
    mut commands: Commands,
    audio: Res<Audio>,
    game_assets: Res<GameAsset>,
    mut p1_q: Query<(Entity, &Status, &Transform, &BallBomb), With<T>>,
) {
    let game_assets = game_assets.into_inner();

    for (e, s, t, ball) in p1_q.iter() {
        if s.hp <= 0.0 {
            commands.entity(e).despawn();
            //cmp_ball::kill(&mut commands, &audio, &game_assets, e, &t);
            commands.spawn(cmp_explosion::ExplosionBundle::from((t.translation, EXPLOSION_RADIUS, game_assets)))
                    .insert(T::default());
        }
    }
}

pub fn system_ignition<T1: Component + Default>(
    mut commands: Commands,
    audio: Res<Audio>,
    game_assets: Res<GameAsset>,
    rapier_context: Res<RapierContext>,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &Transform), (With<BallBomb>, With<T1>)>,
) {
    let game_assets = game_assets.into_inner();

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, flags) = collision_event {;

            if flags.is_empty() && (query.contains(*e1) || query.contains(*e2)) {
                if let Ok((entity, t)) = query.get(*e1) {
                    commands.entity(entity).despawn();
                    commands.spawn(ExplosionBundle::from((t.translation, EXPLOSION_RADIUS, game_assets)))
                        .insert(T1::default());
                } else if let Ok((entity, t)) = query.get(*e2) {
                    commands.entity(entity).despawn();
                    commands.spawn(ExplosionBundle::from((t.translation, EXPLOSION_RADIUS, game_assets)))
                        .insert(T1::default());
                }
            }
        }

        println!("Received collision event: {:?}", collision_event);
    }
}

