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

#[derive(Component)]
pub struct Zundamon;

#[derive(Bundle)]
pub struct BallZundamonBundle {
    zundamon: Zundamon,
    #[bundle]
    ball_bundle: BallBundle,
}


fn random_sprite_handle(game_assets: &GameAsset) -> &Handle<Image> {
    let mut rng = rand::thread_rng();
    let image_vec = vec![ "zun1_handle", "zun2_handle", "zun3_handle" ];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    game_assets.image_handles.get(random_image).unwrap()
}


impl From<(Vec2, f32, Vec2, &GameAsset)> for BallZundamonBundle {
    fn from(tuple: (Vec2, f32, Vec2, &GameAsset)) -> Self {
        let (translation, radius, velocity, game_assets) = tuple;

        let handle = random_sprite_handle(&game_assets);
        let bundle = Self {
            zundamon: Zundamon,
            ball_bundle: BallBundle::from((translation, radius, velocity, handle.clone())),
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

