use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;

use crate::cmp_ball::Zundamon;
use crate::cmp_ball_zombie::Zombie;

#[derive(Component)]
pub struct MainCamera;

pub fn move_camera(
    _keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    let (mut projection, mut transform) = q.single_mut();

    {
        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    if ev.y > 0.0 {
                        projection.scale *= 0.95;
                    } else {
                        projection.scale *= 1.05;
                    }
                }
                MouseScrollUnit::Pixel => {}
            }
            println!("{} {}", projection.scale, transform.translation);
        }
    }

    if buttons.pressed(MouseButton::Middle) {
        for ev in motion_evr.iter() {
            transform.translation.x -= ev.delta.x * projection.scale;
            transform.translation.y += ev.delta.y * projection.scale;
        }
    }
    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.scale = projection.scale.clamp(0.1, 10.0);


}

//const AUTO_CAMERA_K: f32 = 0.02;
const AUTO_CAMERA_K: f32 = 0.06;
const AUTO_CAMERA_D: f32 = 0.6;

const AUTO_CAMERA_VEL_FORWARD: f32 = 1.5;
//const MAX_TRANSLATION_DELTA: f32 = 100.0;
const AUTO_CAMERA_SCALE_K: f32 = 0.06;
const AUTO_CAMERA_DISTANCE_TARGET: f32 = 250.0;

#[derive(Default)]
pub struct History {
    pub prev_error: Vec2,
}

pub fn auto_camera(
    mut history: Local<History>,
    mut q: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    zundamon_q: Query<&Transform, (With<Zundamon>, Without<MainCamera>)>,
    zombie_q: Query<(&Transform, &Velocity), (With<Zombie>, Without<MainCamera>)>,
) {
    if zombie_q.iter().len() == 0 || zundamon_q.iter().len() == 0 {
        return;
    }

    let (mut projection, mut cam_transform) = q.single_mut();

    let mut min_distance = std::f32::MAX;
    let mut t1_translation = Vec2::ZERO;
    let mut t1_velocity = Vec2::ZERO;
    let mut t2_translation = Vec2::ZERO;

    for (zombie_t, zombie_v) in zombie_q.iter() {
        for zundamon_t in zundamon_q.iter() {
            let dist = zombie_t.translation.truncate().distance(zundamon_t.translation.truncate());
            if dist < min_distance {
                min_distance = dist;
                t1_translation = zombie_t.translation.truncate();
                t1_velocity = zombie_v.linvel;
                t2_translation = zundamon_t.translation.truncate();
            }
        }
    }

    let center = (t1_translation + t2_translation) / 2.0;
    let error = center - cam_transform.translation.truncate();
    let delta = error * AUTO_CAMERA_K;
    //let delta = Vec2::new(delta.x.clamp(-MAX_TRANSLATION_DELTA, MAX_TRANSLATION_DELTA),
    //                        delta.y.clamp(-MAX_TRANSLATION_DELTA, MAX_TRANSLATION_DELTA));

    let next_cam_translation = cam_transform.translation.truncate() + delta - delta * AUTO_CAMERA_D;
    cam_transform.translation = Vec3::from((next_cam_translation, cam_transform.translation.z));

    //let distance = t1_translation.distance(t2_translation);
    //let delta = (distance / AUTO_CAMERA_DISTANCE_TARGET) * AUTO_CAMERA_SCALE_K;
    //projection.scale = delta
    //
    history.prev_error = error;
}

