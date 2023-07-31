use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::cmp_primitive_shape;
use crate::cmp_gate_generic;

#[derive(Resource, Reflect, FromReflect, Clone, Copy, PartialEq, Debug, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub enum EditTool { #[default] Select,
                Translate,
                Rotate,
                Scale,
                ScaleDistort,
                Custom1,
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
    GateGeneric,
    GateSplitter(Vec<cmp_gate_generic::SpawnBall>),
    GateTeleport(Option<(u32, Color)>),
    GateZombie,
    GateZundamon,
    PadVelocity(Option<Vec2>),
    PadAcceleration(Option<Vec2>),
    PolygonalShape,
    PrimitiveShape(cmp_primitive_shape::Shape),
    Shredder(Vec<Entity>, Vec<Vec2>),
    SpawnTimer(Vec<cmp_gate_generic::SpawnBall>),
    Wall,
    BreakableP1(Vec<Entity>),
    BreakableP2(Vec<Entity>),
    BreakableSyncP1(Vec<Entity>),
    BreakableSyncP2(Vec<Entity>),
    VibratingShape(Vec<Entity>),
    RotatingShape(Vec<Entity>),
    RevoluteJoint(Vec<Entity>),
    Zundamon,
}

#[derive(Resource, Reflect, Clone, PartialEq, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub enum EditContext {
    Edit(MapObject, Vec<Entity>, EditTool),
    Spawn(MapObject)
}

impl Default for EditContext {
    fn default() -> Self {
        EditContext::Edit(MapObject::None, vec![], EditTool::default())
    }
}


use bevy::window::PrimaryWindow;
use crate::cmp_main_camera::MainCamera;

#[derive(Resource)]
pub struct WorldPosition {
    pub translation: Vec2,
}

pub fn update_world_position(
    mut world_position: ResMut<WorldPosition>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
    // Games typically only have one window (the primary window)
    let window = windows_q.single();

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
    {
        world_position.translation = position;
    }
}

