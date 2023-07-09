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
const AUTO_CAMERA_VEL_FORWARD: f32 = 1.5;
const MAX_TRANSLATION_DELTA: f32 = 100.0;
pub fn auto_camera(
    mut q: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    zundamon_q: Query<&Transform, (With<Zundamon>, Without<MainCamera>)>,
    zombie_q: Query<(&Transform, &Velocity), (With<Zombie>, Without<MainCamera>)>,
) {
    if zombie_q.iter().len() == 0 {
        return;
    }

    let (mut _projection, mut cam_transform) = q.single_mut();

    let mut min_distance = std::f32::MAX;
    let mut target_translation = Vec2::ZERO;
    let mut target_velocity = Vec2::ZERO;

    for (transform, velocity) in zombie_q.iter() {
        for zundamon_t in zundamon_q.iter() {
            let dist = transform.translation.truncate().distance(zundamon_t.translation.truncate());
            if dist < min_distance {
                min_distance = dist;
                target_translation = transform.translation.truncate();
                target_velocity = velocity.linvel;
            }

        }
    }

    let vel_forward = target_velocity * AUTO_CAMERA_VEL_FORWARD;
    let vel_forward = Vec2::new(vel_forward.x.clamp(-300.0, 300.0), vel_forward.y.clamp(-300.0, 300.0));
    let delta = (target_translation + vel_forward - cam_transform.translation.truncate()) * AUTO_CAMERA_K;
    let delta = Vec2::new(delta.x.clamp(-MAX_TRANSLATION_DELTA, MAX_TRANSLATION_DELTA),
                            delta.y.clamp(-MAX_TRANSLATION_DELTA, MAX_TRANSLATION_DELTA));
    let next_cam_translation = cam_transform.translation.truncate() + delta;

    cam_transform.translation = Vec3::from((next_cam_translation, cam_transform.translation.z));
}

