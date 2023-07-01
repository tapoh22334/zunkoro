use bevy::prelude::*;
use bevy::audio::AudioSource;
use std::collections::HashMap;

#[derive(Component, Resource, Default, Debug)]
pub struct GameAsset {
    pub image_handles: HashMap<String, Handle<Image>>,
    pub audio_handles: HashMap<String, Handle<AudioSource>>,
}

