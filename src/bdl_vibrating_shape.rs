use std::any::type_name;

use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::cmp_vibrator::Vibrator;
use crate::ev_save_load_world::Derrived;

#[derive(Bundle)]
pub struct VibratingShapeAttachmentBundle {
    pub vibrator: Vibrator,
}

impl Default for VibratingShapeAttachmentBundle {
    fn default() -> Self {
        Self {
            vibrator: Vibrator::default(),
        }
    }
}


impl From<Vibrator> for VibratingShapeAttachmentBundle
{
    fn from(vibrator: Vibrator) -> Self {
        Self {
            vibrator,
            ..Default::default()
        }
    }
}


