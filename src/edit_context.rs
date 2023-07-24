use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::cmp_primitive_shape;

#[derive(Resource, Reflect, FromReflect, Clone, Copy, PartialEq, Debug, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub enum EditTool { #[default] Select,
                Translate,
                Rotate,
                Scale,
                ScaleDistort,
                }

#[derive(Component, Resource, Reflect, FromReflect, Clone, PartialEq, Debug, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub enum MapObject {
    #[default]None,
    Artillery,
    BlockZombie,
    ConverterBody,
    GearSimple,
    GearSorting,
    GearSwirl,
    GateTeleport(Option<(u32, Color)>),
    GateZombie,
    GateZundamon,
    PadVelocity(Option<Vec2>),
    PadAcceleration(Option<Vec2>),
    PrimitiveShape(cmp_primitive_shape::Shape),
    Shredder(Vec<Entity>, Vec<Vec2>),
    VibratingShape(Vec<Entity>),
    RotatingShape(Vec<Entity>),
    RevoluteJoint(Vec<Entity>),
    Zundamon,
}

#[derive(Resource, Reflect, Clone, PartialEq, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub enum EditContext {
    Edit(Vec<Entity>, EditTool),
    Spawn(MapObject)
}

impl Default for EditContext {
    fn default() -> Self {
        EditContext::Edit(vec![], EditTool::default())
    }
}

