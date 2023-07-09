use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_trajectory::Trajectory;
use crate::cmp_trajectory;
use crate::cmp_blood;

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub previous_position: Option<Vec2>,
}

#[derive(Component)]
pub struct Zundamon;

pub fn add(commands: &mut Commands, game_assets: &Res<GameAsset>, pos: Vec2, r: f32, vel: Vec2) {
    let mut rng = rand::thread_rng();
    let image_vec = vec![ "zun1_handle", "zun2_handle", "zun3_handle" ];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    let sprite_handle = game_assets.image_handles.get(random_image).unwrap();
    let collider = Collider::ball(r);

    commands
        .spawn(Ball {radius: r, previous_position: None})
        .insert(Zundamon)
        .insert(RigidBody::Dynamic)
        .insert(Restitution::coefficient(0.1))
        .insert(Friction::coefficient(0.011))
        .insert(collider)
        .insert(CollisionGroups::new(Group::GROUP_1, Group::ALL))
        .insert(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * (r * 2.0)),
                        ..default()
                    },
                    texture: sprite_handle.clone(),
                    ..default()
        })
        .insert(TransformBundle {
                    local: Transform {
                                translation: Vec3::new(pos.x, pos.y, 1.0),
                                //scale: Vec3::ONE / r,
                                ..Default::default()
                            },
                    ..default()
        })
        .insert(Velocity {
            linvel: vel,
            angvel: 0.0,
        })
    ;
}

pub fn kill(commands: &mut Commands,
            audio: &Res<Audio>,
            game_assets: &Res<GameAsset>,
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
