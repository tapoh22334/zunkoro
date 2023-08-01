use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball::Ball;

use crate::cmp_combat::Player1;
use crate::cmp_combat::Player2;

const DEFAULT_RADIUS: f32 = 512.0 / 2.0;
const DEFAULT_RANGE: f32 = 0.25 * std::f32::consts::PI;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug, Default)]
pub struct ArtilleryAuto {
    pub angvel: f32,
    pub angle: f32,
    pub angle_range: (f32, f32),
    }

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Barrel;

#[derive(Bundle)]
pub struct ArtilleryAutoBarrelBundle {
    barrel: Barrel,
    #[bundle]
    sprite_bundle: SpriteBundle,
}


impl From<(Quat, &GameAsset)> for ArtilleryAutoBarrelBundle {
    fn from(tuple: (Quat, &GameAsset)) -> Self {
        let (rotation, game_assets) = tuple;
        let sprite_handle = game_assets.image_handles.get("artillery_frag2").unwrap();
        Self {
            barrel: Barrel,
            sprite_bundle: SpriteBundle {
                texture: sprite_handle.clone(),
                transform: Transform {
                    rotation,
                    scale: Vec3::ONE,
                    translation: Vec3::new(0.0, 0.0, 2.0),
                },
                ..default()
            },
        }
    }
}


#[derive(Bundle)]
pub struct ArtilleryAutoBaseBundle<T: Component + Default> {
    player: T,
    artillery: ArtilleryAuto,
    bbsize: BBSize,
    collider: Collider,
    collision_groups: CollisionGroups,
    sensor: Sensor,
    #[bundle]
    sprite_bundle: SpriteBundle,
}


impl<T: Component + Default> From<&GameAsset> for ArtilleryAutoBaseBundle<T> {
    fn from(game_assets: &GameAsset) -> Self {
        let sprite_handle = game_assets.image_handles.get("artillery_frag1").unwrap();
        Self {
            player: T::default(),
            artillery: ArtilleryAuto::default(),
            bbsize: BBSize{x: DEFAULT_RADIUS * 2.0, y: DEFAULT_RADIUS * 2.0},
            collider: Collider::ball(DEFAULT_RADIUS),
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),
            sensor: Sensor,
            sprite_bundle: SpriteBundle {
                texture: sprite_handle.clone(),
                transform: Transform {
                    scale: Vec3::ONE,
                    ..Default::default()
                },
                ..default()
            },
        }
    }
}

impl<T: Component + Default> From<(Vec3, Vec3, ArtilleryAuto, &GameAsset)> for ArtilleryAutoBaseBundle<T> {
    fn from(tuple: (Vec3, Vec3, ArtilleryAuto, &GameAsset)) -> Self {
        let (translation, scale, artillery, game_assets) = tuple;

        let mut bundle = ArtilleryAutoBaseBundle::from(game_assets);
        bundle.artillery = artillery;
        bundle.sprite_bundle.transform.translation = translation;
        bundle.sprite_bundle.transform.scale = scale;

        bundle
    }
}


fn quantize_angle(angle: f32, num_steps: u8) -> f32 {
    let step_size = 2.0 * std::f32::consts::PI / num_steps as f32;
    let half_step = step_size / 2.0;
    let normalized_angle = (angle + half_step + 2.0 * std::f32::consts::PI) % (2.0 * std::f32::consts::PI);
    (normalized_angle / step_size).round() * step_size
}

use crate::edit_context;
use crate::edit_context::*;
pub fn handle_user_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    game_assets: Res<GameAsset>,
    mut edit_context: ResMut<EditContext>,
    world_position: ResMut<WorldPosition>,
    mut artillery_frag1: Query<(&Children, &Transform, &mut ArtilleryAuto)>,
    mut artillery_frag2: Query<&mut Transform, (With<Barrel>, Without<ArtilleryAuto>)>,
    ) {
    let game_assets = game_assets.into_inner();

    if buttons.just_pressed(MouseButton::Left) {
        if let EditContext::Spawn(map_object) = edit_context.clone() {
            if let MapObject::ArtilleryAutoP1 = map_object {
                if buttons.just_pressed(MouseButton::Left) {
                    let artillery = ArtilleryAuto {
                        angvel: 0.5,
                        angle: 0.0,
                        angle_range: (-DEFAULT_RANGE, DEFAULT_RANGE)
                    };
                    let mut entity = commands.spawn(ArtilleryAutoBaseBundle::<Player1>::from((
                                Vec3::from((world_position.translation, 2.0)),
                                Vec3::ONE,
                                artillery,
                                game_assets,
                                )));
                    entity.with_children(|children| {
                        children.spawn(ArtilleryAutoBarrelBundle::from((Quat::from_rotation_z(0.0), game_assets)));
                    });
                    entity.insert(MapObject::ArtilleryAutoP1);
                    *edit_context = EditContext::Edit(MapObject::ArtilleryAutoP1, vec![entity.id()], EditTool::Select);
                }
            }

            else if let MapObject::ArtilleryAutoP2 = map_object {
                if buttons.just_pressed(MouseButton::Left) {
                    let artillery = ArtilleryAuto {
                        angvel: 0.5,
                        angle: 0.0,
                        angle_range: (-DEFAULT_RANGE, DEFAULT_RANGE)
                    };
                    let mut entity = commands.spawn(ArtilleryAutoBaseBundle::<Player2>::from((
                                Vec3::from((world_position.translation, 2.0)),
                                Vec3::ONE,
                                artillery,
                                game_assets,
                                )));
                    entity.with_children(|children| {
                        children.spawn(ArtilleryAutoBarrelBundle::from((Quat::from_rotation_z(0.0), game_assets)));
                    });
                    entity.insert(MapObject::ArtilleryAutoP2);
                    *edit_context = EditContext::Edit(MapObject::ArtilleryAutoP2, vec![entity.id()], EditTool::Select);
                }
            }
        }
    }

    match edit_context.clone() {
        EditContext::Edit(MapObject::ArtilleryAutoP1, _, EditTool::Select) |
        EditContext::Edit(MapObject::ArtilleryAutoP2, _, EditTool::Select) => {
            if let EditContext::Edit(map_object, entities, _) = edit_context.clone() {
                if keys.pressed(KeyCode::Key1) {
                    *edit_context = EditContext::Edit(map_object , entities, EditTool::Custom1);
                }
            }
        }
        _ => {}
    }

    match edit_context.clone() {
        EditContext::Edit(MapObject::ArtilleryAutoP1, entities, EditTool::Custom1) |
        EditContext::Edit(MapObject::ArtilleryAutoP2, entities, EditTool::Custom1) => {
            let entity = entities[0];
            if let Ok((children, base_transform, mut artillery)) = artillery_frag1.get_mut(entity) {
                let mut barrel_transform = artillery_frag2.get_mut(*children.iter().next().unwrap()).unwrap();

                let pos = base_transform.translation.truncate();
                let dir = (world_position.translation - pos).normalize();
                let angle = Vec2::new(1.0, 0.0).angle_between(dir);
                let angle = quantize_angle(angle, 8);

                barrel_transform.rotation = Quat::from_rotation_z(angle);
                artillery.angle = angle;
                artillery.angle_range = (angle - DEFAULT_RANGE, angle + DEFAULT_RANGE)
            }
        }
        _ => {}
    }

}

pub fn find_nearest<T: Component>(ball_q: &Query<(Entity, &mut Transform, &mut Velocity, &Ball), (With<T>, Without<Barrel>)>, translation: &Vec3) -> Option<(Entity, f32)> {
    let mut min: Option<(Entity, f32)> = None;
    for (entity, ball_t, _, _) in ball_q {
        let distance = translation.truncate().distance(ball_t.translation.truncate());
        if min.is_none() || min.unwrap().1 >= distance{
            min = Some((entity, distance));
        }
    }
    min
}

fn normalized_angle(angle: f32) -> f32 {
    let pi = std::f32::consts::PI;
    let twopi = 2.0 * std::f32::consts::PI;
    let mut angle =  angle % twopi;
    angle = (angle + twopi) % twopi;
    if angle > pi {
        angle -= twopi;
    }
    angle
}

const DETECTION_RANGE: f32 = 1000.0;
pub fn system<T1: Component, T2: Component>(
    time: Res<Time>,
    mut ball_q: Query<(Entity, &mut Transform, &mut Velocity, &Ball), (With<T2>, Without<Barrel>)>,
    mut artillery_frag1: Query<(Entity, &Children, &Transform, &mut ArtilleryAuto), (With<T1>, Without<Barrel>, Without<Ball>)>,
    mut artillery_frag2: Query<&mut Transform, (With<Barrel>, Without<Ball>)>,
) {
    for (entity, children, transform, mut artillery) in artillery_frag1.iter_mut() {
        let child = children.iter().next().unwrap();
        let mut barrel_transform = artillery_frag2.get_mut(child.to_owned()).unwrap();

        let mut angle_delta: f32 = 0.0;
        if let Some((entity, distance)) = find_nearest(&ball_q, &transform.translation) {
            if distance <= DETECTION_RANGE {
                let (entity, ball_t, _, _) = ball_q.get(entity).unwrap();
                let angle = {
                    let dir = (ball_t.translation.truncate() - transform.translation.truncate()).normalize();
                    let q1 = Quat::from_rotation_z(artillery.angle);
                    let q2 = Quat::from_rotation_z(Vec2::new(1.0, 0.0).angle_between(dir));
                    let q_between = q1.conjugate() * q2;
                    let (axis, mut angle) = q_between.to_axis_angle();
                    println!("axis, angle, {:?}, {:?}", axis.z, angle);
                    if axis.z < 0.0 {
                        angle -= std::f32::consts::PI;
                    }
                    angle
                };

                angle_delta = normalized_angle(angle);
                let clamp = artillery.angvel.abs() * time.delta_seconds();
                angle_delta = angle_delta.clamp(-clamp, clamp);


            } else {
                angle_delta = artillery.angvel * time.delta_seconds();
            }
        } else {
            angle_delta = artillery.angvel * time.delta_seconds();
        }

        let new_angle = artillery.angle + angle_delta;

        let pivot_rotation = Quat::from_rotation_z(new_angle - artillery.angle);
        barrel_transform.rotate_around(Vec3::ZERO, pivot_rotation);

        artillery.angle = new_angle;

        if artillery.angle <= artillery.angle_range.0 {
            artillery.angvel = artillery.angvel.abs();
        } else if artillery.angle >= artillery.angle_range.1 {
            artillery.angvel = -artillery.angvel.abs();
        }
    }
}


pub fn system_fire(
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(Entity, &mut Transform, &mut Velocity, &Ball)>,
    artillery_q: Query<(Entity, &Transform, &BBSize, &ArtilleryAuto), Without<Ball>>,
) {
    for (artillery_e, artillery_transform, bbsize, artillery) in artillery_q.iter() {
        for (ball_e, mut ball_transform, mut ball_velocity, ball) in ball_q.iter_mut() {
            if rapier_context.intersection_pair(artillery_e, ball_e) == Some(true) {
                let dir = Quat::from_rotation_z(artillery.angle).mul_vec3(Vec3::new(1.0, 0.0, 0.0));
                let dist = bbsize.x / 2.0 * artillery_transform.scale.x + ball.radius + 1.0;
                ball_transform.translation = artillery_transform.translation + dir * dist;
                ball_velocity.linvel = dir.truncate() * 400.0;
            }
        }
    }
}


fn file_name_string<T>() -> String {
    const FILE_PREFIX: &str = "/artillery_auto_";
    const FILE_EXT: &str = ".map";

    let type_name = std::any::type_name::<T>();
    let pure_type_name = type_name.split("::").last().unwrap_or(type_name);

    (FILE_PREFIX.to_string() + pure_type_name + FILE_EXT)
}

use std::any::TypeId;
fn get_map_object<T: 'static>() -> MapObject {
    if (TypeId::of::<T>() == TypeId::of::<Player1>()) {
        MapObject::ArtilleryAutoP1
    }
    else {
        MapObject::ArtilleryAutoP2
    } 
}

use crate::ev_save_load_world::LoadWorldEvent;
pub fn load<T: Component + Default>(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    ) {
    let game_assets = game_assets.into_inner();

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + file_name_string::<T>().as_str());
        if let Ok(json_str) = json_str {
            let elem_list: Vec<(u32, u32, Vec3, Quat, Vec3, ArtilleryAuto)> = serde_json::from_str(&json_str).unwrap();

            for (i, i2, t, _, s, a) in elem_list {
                let rotation = Quat::from_rotation_z(a.angle);
                let entity2 = commands.get_or_spawn(Entity::from_raw(i2))
                                    .insert(ArtilleryAutoBarrelBundle::from(( rotation, game_assets ))).id();

                let entity = commands.get_or_spawn(Entity::from_raw(i))
                                    .insert(ArtilleryAutoBaseBundle::<T>::from((t, s, a, game_assets)))
                                    .insert(get_map_object::<T>())
                                    .push_children(&[entity2]);

            }
        }
    }
}

use crate::ev_save_load_world::SaveWorldEvent;
pub fn save<T: Component + Default>(mut save_world_er: EventReader<SaveWorldEvent>,
              artillery_q: Query<(Entity, &Children, &Transform, &ArtilleryAuto), With<T>>,
              barrel_q: Query<Entity, With<Barrel>>,
              ) {
    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut artillery_list: Vec<(u32, u32, Vec3, Quat, Vec3, ArtilleryAuto)> = vec![];

        for (e, c, t, a) in artillery_q.iter() {
            let barrel = barrel_q.get(*c.iter().next().unwrap()).unwrap();
            artillery_list.push((e.index(), barrel.index(), t.translation, t.rotation, t.scale, a.clone()));
        }

        std::fs::write(dir + file_name_string::<T>().as_str(), serde_json::to_string(&artillery_list).unwrap()).unwrap();
    }
}

