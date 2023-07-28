use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_trajectory::Trajectory;
use crate::cmp_trajectory;
use crate::cmp_blood;

const DEFAULT_BALL_RADIUS: f32 = 19.0 / 2.0;
//const DEFAULT_RESTITUTION: f32 = 0.1;
//const DEFAULT_FRICTION: f32 = 0.011;

const DEFAULT_RESTITUTION: f32 = 0.5;
const DEFAULT_FRICTION: f32 = 0.5;

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub previous_position: Option<Vec2>,
}

#[derive(Component)]
pub struct Zundamon;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    zundamon: Zundamon,
    rigid_body: RigidBody,
    restitution: Restitution,
    friction: Friction,
    collider: Collider,
    collision_groups: CollisionGroups,
    velocity: Velocity,
    #[bundle]
    sprite_bundle: SpriteBundle,
}


impl Default for BallBundle {
    fn default() -> Self {
        Self {
            ball: Ball { radius: DEFAULT_BALL_RADIUS, previous_position: None },
            zundamon: Zundamon,
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(DEFAULT_RESTITUTION),
            friction: Friction::coefficient(DEFAULT_FRICTION),
            collider: Collider::ball(DEFAULT_BALL_RADIUS),
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),
            velocity: Velocity { linvel: Vec2::ZERO, angvel: 0.0 },
            sprite_bundle: SpriteBundle {
                ..default()
            },
        }
    }
}

fn random_sprite_handle(game_assets: &GameAsset) -> &Handle<Image> {
    let mut rng = rand::thread_rng();
    let image_vec = vec![ "zun1_handle", "zun2_handle", "zun3_handle" ];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    game_assets.image_handles.get(random_image).unwrap()
}

impl From<(Vec2, f32, Vec2, &GameAsset)> for BallBundle {
    fn from(tuple: (Vec2, f32, Vec2, &GameAsset)) -> Self {
        let mut bundle = BallBundle::default();
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


pub fn kill(commands: &mut Commands,
            audio: &Res<Audio>,
            game_assets: &GameAsset,
            entity: Entity,
            trans: &Transform,
            ) {
        let mut rng = rand::thread_rng();
        let sv = vec![ "zundamon_die1_handle",
                        "zundamon_die2_handle",
                        "zundamon_die3_handle",
                        "zundamon_die4_handle",
                        "zundamon_die5_handle",
                        "zundamon_die6_handle",
                        "zundamon_die7_handle",
                     ];
        let random_audio = sv[rng.gen_range(0..sv.len())];
        cmp_blood::add(commands, trans.translation.truncate());
        commands.entity(entity).despawn();
        audio.play(game_assets.audio_handles.get(random_audio).unwrap().clone());
}

pub fn system_trajectory(
    mut commands: Commands,
    mut q: Query<(&Transform, &mut Ball)>,
) {
    for (t, mut ball) in q.iter_mut() {
        let curr_pos = t.translation.truncate();

        if ball.previous_position.is_some() {
            let prev_pos = ball.previous_position.unwrap();
            let trajectory = Trajectory { line: (prev_pos, curr_pos), life_time: 0.6 };
            cmp_trajectory::add(&mut commands, trajectory);
        }

        ball.previous_position = Some(curr_pos);
    }
}
