use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Debug)]
pub struct BBSize {
    pub x: f32,
    pub y: f32,
}

