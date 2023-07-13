use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use rand::prelude::*;
use crate::cmp_bbsize::BBSize;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball_zombie;
use crate::cmp_fuse_time::FuseTime;

const BALL_SIZE: f32 = 30.0;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct GateZombie {
    pub size: Vec2,
    pub position: Vec2,
    pub remain: i32,
    pub prob: f32,
    pub spawn_offset_sec: f32,
}

pub fn add(commands: &mut Commands, gate_zombie: GateZombie) -> Entity {
    let size = gate_zombie.size;
    let pos = gate_zombie.position;
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(FuseTime{timer: Timer::from_seconds(gate_zombie.spawn_offset_sec, TimerMode::Once)} )
        .insert(gate_zombie);

    return entity.id();
}


pub fn system(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<GameAsset>,
    mut query: Query<(Entity, &Transform, &BBSize, &mut FuseTime, &mut GateZombie)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, transform, bbsize, mut fuse_time, mut gate_zombie) in query.iter_mut() {
        fuse_time.timer.tick(time.delta());
        if ! fuse_time.timer.finished() { continue; }

        if gate_zombie.remain > 0 {
            if rng.gen::<f32>() < gate_zombie.prob {
                let size = Vec2::new(bbsize.x, bbsize.y) * transform.scale.truncate();
                let pos_max = transform.translation.truncate() + (size / 2.0);
                let pos_min = transform.translation.truncate() - (size / 2.0);

                let x = rng.gen_range(pos_min.x .. pos_max.x);
                let y = rng.gen_range(pos_min.y .. pos_max.y);

                cmp_ball_zombie::add(&mut commands, &game_assets, Vec2::new(x, y), BALL_SIZE, Vec2::new(0.0, 0.0));
                gate_zombie.remain -= 1;
            }
        } else {
            commands.get_entity(entity).unwrap().despawn();
        }

    }
}


const FILE_NAME: &str = "/gate_zombie.map";
use crate::ev_save_load_world::LoadWorldEvent;
pub fn load(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    ) {

    for e in load_world_er.iter() {
        let dir = e.0.clone();

        let json_str = std::fs::read_to_string(dir + FILE_NAME);
        if let Ok(json_str) = json_str {
            let elem_list: Vec<GateZombie> = serde_json::from_str(&json_str).unwrap();

            for e in elem_list {
                add(&mut commands, e);
            }
        }
    }
}


use crate::ev_save_load_world::SaveWorldEvent;
pub fn save(mut save_world_er: EventReader<SaveWorldEvent>,
              q: Query<(&Transform, &GateZombie)>,
              ) {

    for e in save_world_er.iter() {
        let dir = e.0.clone();
        let mut elem_list: Vec<GateZombie> = vec![];

        for (t, e) in q.iter() {
            let mut e = e.clone();
            e.size = e.size * t.scale.truncate();
            e.position = t.translation.truncate();
            elem_list.push(e.clone());
        }

        std::fs::write(dir + FILE_NAME, serde_json::to_string(&elem_list).unwrap()).unwrap();
    }
}

