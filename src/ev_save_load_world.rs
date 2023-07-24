use bevy::prelude::*;

#[derive(Component)]
pub struct Derrived;

pub struct SaveWorldEvent(pub String);
pub struct LoadWorldEvent(pub String);

