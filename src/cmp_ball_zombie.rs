use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_ball;
use crate::cmp_ball::{Ball, Zundamon};
use crate::cmp_game_asset::GameAsset;

const DEFAULT_BALL_RADIUS: f32 = 19.0 / 2.0;
const DEFAULT_RESTITUTION: f32 = 0.1;
const DEFAULT_FRICTION: f32 = 0.010;

#[derive(Component)]
pub struct Zombie;

#[derive(Bundle)]
pub struct BallZombieBundle {
    ball: Ball,
    zombie: Zombie,
    rigid_body: RigidBody,
    restitution: Restitution,
    friction: Friction,
    collider: Collider,
    collision_groups: CollisionGroups,
    velocity: Velocity,
    #[bundle]
    sprite_bundle: SpriteBundle,
}


impl Default for BallZombieBundle {
    fn default() -> Self {
        Self {
            ball: Ball { radius: DEFAULT_BALL_RADIUS, previous_position: None },
            zombie: Zombie,
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(DEFAULT_RESTITUTION),
            friction: Friction::coefficient(DEFAULT_FRICTION),
            collider: Collider::ball(DEFAULT_BALL_RADIUS),
            collision_groups: CollisionGroups::new(Group::GROUP_2, Group::ALL),
            velocity: Velocity { linvel: Vec2::ZERO, angvel: 0.0 },
            sprite_bundle: SpriteBundle {
                ..default()
            },
        }
    }
}

fn random_sprite_handle(game_assets: &GameAsset) -> &Handle<Image> {
    let mut rng = rand::thread_rng();
    let image_vec = vec!["zombie1_handle"];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    game_assets.image_handles.get(random_image).unwrap()
}

impl From<(Vec2, f32, Vec2, &GameAsset)> for BallZombieBundle {
    fn from(tuple: (Vec2, f32, Vec2, &GameAsset)) -> Self {
        let mut bundle = BallZombieBundle::default();
        let (translation, radius, velocity, game_assets) = tuple;

        let sprite_handle = random_sprite_handle(&game_assets);

        bundle.sprite_bundle = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * (radius * 2.0)),
                ..default()
            },
            texture: sprite_handle.clone(),
            transform: Transform {
                translation: Vec3::from((translation, 1.0)),
                ..default()
            },
            ..default()
        };

        bundle
    }
}

pub fn system_infection(
    mut commands: Commands,
    audio: Res<Audio>,
    rapier_context: Res<RapierContext>,
    game_assets: Res<GameAsset>,
    zundamon_q: Query<(Entity, &Transform, &Velocity, &Ball), With<Zundamon>>,
    zombie_q: Query<Entity, (With<Zombie>, Without<Zundamon>)>,
) {
    let game_assets = game_assets.into_inner();

    for (zundamon_e, zundamon_t, zundamon_v, zundamon_ball) in zundamon_q.iter() {
        for zombie_e in zombie_q.iter() {
            if let Some(_) = rapier_context.contact_pair(zundamon_e, zombie_e) {
                cmp_ball::kill(&mut commands, &audio, &game_assets, zundamon_e, &zundamon_t);
                let _ = commands.spawn(
                    BallZombieBundle::from((zundamon_t.translation.truncate(), zundamon_ball.radius, zundamon_v.linvel, game_assets)));
            }
        }
    }
}

