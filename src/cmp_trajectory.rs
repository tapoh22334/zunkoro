use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cmp_fuse_time::FuseTime;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Trajectory {
    pub line: (Vec2, Vec2),
    pub life_time: f32,
}

const MAX_ALPHA: f32 = 0.8;

pub fn add(commands: &mut Commands,
            trajectory: Trajectory) -> Entity {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(trajectory.line.0);
    path_builder.line_to(trajectory.line.1);
    let path = path_builder.build();

    let color = Color::Rgba {
                        red: 50.0,
                        green: 205.0,
                        blue: 50.0,
                        alpha: 1.0 * MAX_ALPHA,
                     };

    let mut entity = commands.spawn((
        ShapeBundle {
            path,
            ..default()
        },
        Stroke::new(color, 3.0),
    ));

    entity.insert(FuseTime{timer: Timer::from_seconds(trajectory.life_time, TimerMode::Once)} );
    entity.insert(trajectory);

    return entity.id();
}



pub fn system(
    mut commands: Commands,
    time: Res<Time>,
    mut trajectory_q: Query<(Entity, &mut FuseTime, &mut Stroke)>,
) {
    for (entity, mut fuse_time, mut stroke) in trajectory_q.iter_mut() {
        fuse_time.timer.tick(time.delta());
        if fuse_time.timer.finished() { 
            commands.entity(entity).despawn();
        } else {
            let alpha = fuse_time.timer.percent_left();
            stroke.color = stroke.color.with_a(alpha * MAX_ALPHA);
        }
    }
}

