use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Resource, Default, Debug)]
pub struct GameAsset {
    pub image_handles: HashMap<String, Handle<Image>>,
}

