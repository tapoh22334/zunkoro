use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Debug)]
pub struct FuseTime {
    pub timer: Timer,
}

