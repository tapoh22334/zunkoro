use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;

use crate::cmp_ball_zundamon::Zundamon;
use crate::cmp_ball_zombie::Zombie;
use crate::cmp_fuse_time::FuseTime;

#[derive(Component)]
pub struct MainCamera;

pub fn move_camera(
    keys: Res<Input<KeyCode>>,
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

    if keys.pressed(KeyCode::Up) {
        projection.scale *= 0.995;
        println!("{} {}", projection.scale, transform.translation);
    }
    if keys.pressed(KeyCode::Down) {
        projection.scale *= 1.005;
        println!("{} {}", projection.scale, transform.translation);
    }

    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.scale = projection.scale.clamp(0.1, 10.0);


}

//const AUTO_CAMERA_K: f32 = 0.02;
const AUTO_CAMERA_K: f32 = 0.08;
const AUTO_CAMERA_D: f32 = 0.3;
const AUTO_CAMERA_VEL_FORWARD: f32 = 1.5;
//const MAX_TRANSLATION_DELTA: f32 = 100.0;
const AUTO_CAMERA_SCALE_K: f32 = 0.06;
const AUTO_CAMERA_DISTANCE_TARGET: f32 = 250.0;

const AUTO_CAMERA_TRANSITION_WAIT_SEC: f32 = 1.5;

#[derive(Default)]
pub struct History {
    pub prev_error: Vec2,
}

fn find_nearest_pair(
    zombie_q: &Query<(Entity, &Transform), (With<Zombie>, Without<MainCamera>)>,
    zundamon_q: &Query<(Entity, &Transform), (With<Zundamon>, Without<MainCamera>)>,
    ) -> (Entity, Entity) {

    let mut min_distance = std::f32::MAX;
    let mut t1_e = None;
    let mut t2_e = None;

    for (zombie_e, zombie_t) in zombie_q.iter() {
        for (zundamon_e, zundamon_t) in zundamon_q.iter() {
            let dist = zombie_t.translation.truncate().distance(zundamon_t.translation.truncate());
            if dist < min_distance {
                min_distance = dist;
                t1_e = Some(zombie_e);
                t2_e = Some(zundamon_e);
            }
        }
    }

    (t1_e.unwrap(), t2_e.unwrap())
}

pub fn auto_camera(
    time: Res<Time>,
    mut history: Local<History>,
    mut curr_interest: Local<Option<(Entity, Entity)>>,
    mut next_interest: Local<Option<((Entity, Entity), FuseTime)>>,
    mut q: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    zombie_q: Query<(Entity, &Transform), (With<Zombie>, Without<MainCamera>)>,
    zundamon_q: Query<(Entity, &Transform), (With<Zundamon>, Without<MainCamera>)>,
) {
    if zombie_q.iter().len() == 0 || zundamon_q.iter().len() == 0 {
        return;
    }

    let (mut _projection, mut cam_transform) = q.single_mut();

    // Transition focus
    let (t1_e, t2_e) = find_nearest_pair(&zombie_q, &zundamon_q);

    if curr_interest.is_none() {
        *curr_interest = Some((t1_e, t2_e));
    } else {
        let (ci1_e, ci2_e) = curr_interest.unwrap();
        if zombie_q.get(ci1_e).is_err() || zundamon_q.get(ci2_e).is_err() {
            *curr_interest = Some((t1_e, t2_e));
        }

        if next_interest.is_none() {
            if t1_e != ci1_e || t2_e != ci2_e {
                *next_interest = Some(((t1_e, t2_e), FuseTime{timer: Timer::from_seconds(AUTO_CAMERA_TRANSITION_WAIT_SEC, TimerMode::Once)}));
            }
        }
        else {
            let ((ni1_e, ni2_e), ref mut fuse_time) = next_interest.as_mut().unwrap();
            if t1_e == *ni1_e && t2_e == *ni2_e {
                fuse_time.timer.tick(time.delta());
                if fuse_time.timer.finished() { 
                    *curr_interest = Some((*ni1_e, *ni2_e));
                    *next_interest = None;
                }
            } else {
                *next_interest = Some(((t1_e, t2_e), FuseTime{timer: Timer::from_seconds(AUTO_CAMERA_TRANSITION_WAIT_SEC, TimerMode::Once)}));
            }
        }
    }

    // Move Camera
    let (ci1_e, ci2_e) = curr_interest.unwrap();

    let (_, t1_transform) = zombie_q.get(ci1_e).unwrap();
    let (_, t2_transform) = zundamon_q.get(ci2_e).unwrap();

    let t1_translation = t1_transform.translation.truncate();
    let t2_translation = t2_transform.translation.truncate();

    let center = (t1_translation + t2_translation) / 2.0;
    let error = center - cam_transform.translation.truncate();
    let delta = error * AUTO_CAMERA_K;

    let next_cam_translation = cam_transform.translation.truncate() + delta - delta * AUTO_CAMERA_D;
    cam_transform.translation = Vec3::from((next_cam_translation, cam_transform.translation.z));

    history.prev_error = error;
}


fn find_bottom(
    zundamon_q: &Query<(Entity, &Transform), (With<Zundamon>, Without<MainCamera>)>,
    ) -> Entity {

    let mut min_y = std::f32::MAX;
    let mut entity = None;

    for (zundamon_e, zundamon_t) in zundamon_q.iter() {
        let y = zundamon_t.translation.truncate().y;
        if y < min_y {
            min_y = y;
            entity = Some(zundamon_e);
        }
    }

    entity.unwrap()
}



use bevy::window::PrimaryWindow;
const AUTO_CAMERA_VERTICAL_K: f32 = 0.08;
const AUTO_CAMERA_VERTICAL_D: f32 = 0.3;
const AUTO_CAMERA_VERTICAL_POS_RATIO: f32 = 0.8;

pub fn auto_camera_vertical(
    windows_q: Query<&Window, With<PrimaryWindow>>,
    mut camera_q: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    zundamon_q: Query<(Entity, &Transform), (With<Zundamon>, Without<MainCamera>)>,
) {
    if zundamon_q.iter().len() == 0 {
        return;
    }

    let (mut _projection, mut cam_transform) = camera_q.single_mut();
    let windows = windows_q.single();
    let window_size_y = windows.resolution.height();
    let y_scroll = window_size_y * AUTO_CAMERA_VERTICAL_POS_RATIO - window_size_y / 2.0;

    let target_e = find_bottom(&zundamon_q);

    let (_, zundamon_t) = zundamon_q.get(target_e).unwrap();

    let error = zundamon_t.translation.y - cam_transform.translation.y + y_scroll;
    let delta = (error * AUTO_CAMERA_VERTICAL_K).clamp(std::f32::MIN, 0.0);

    let next_y = cam_transform.translation.y + delta - delta * AUTO_CAMERA_VERTICAL_D;
    cam_transform.translation = Vec3::from((cam_transform.translation.x, next_y, cam_transform.translation.z));
}
