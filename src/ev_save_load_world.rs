use bevy::prelude::*;

#[derive(Component)]
pub struct Derrived;

pub struct SaveWorldEvent(pub String);
pub struct LoadWorldEvent(pub String);
pub struct LoadWorldEventStage2(pub String);

pub fn forward_event(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut load_world2_ew: EventWriter<LoadWorldEventStage2>,
    )
{
    for e in load_world_er.iter() {
        let dir = e.0.clone();
        load_world2_ew.send(LoadWorldEventStage2(dir));
    }
}

