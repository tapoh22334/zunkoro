//#![windows_subsystem = "windows"]
use serde::{Serialize, Deserialize};
use rand::prelude::*;

use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
use bevy::render::texture::{ImageType, CompressedImageFormats};
use bevy::sprite::collide_aabb::collide;
use bevy::audio::AudioLoader;
use rodio::decoder::Decoder;

use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;

use bevy_inspector_egui::bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use bevy_prototype_lyon::prelude::*;

mod cmp_gate_teleport;
use crate::cmp_gate_teleport::GateTeleportExit;
use crate::cmp_gate_teleport::GateTeleportEntrance;

mod cmp_gate_zundamon;
use crate::cmp_gate_zundamon::GateZundamon;

mod cmp_bbsize;
use crate::cmp_bbsize::BBSize;

mod cmp_fuse_time;

mod cmp_game_asset;
use crate::cmp_game_asset::GameAsset;

mod cmp_artillery;
use crate::cmp_artillery::Artillery;

mod cmp_ball;
use crate::cmp_ball::Ball;

mod cmp_blood;
use crate::cmp_blood::Blood;

mod cmp_gear;
use crate::cmp_gear::GearSimple;
use crate::cmp_gear::GearSorting;
use crate::cmp_gear::GearSwirl;

mod cmp_pad_velocity;
use crate::cmp_pad_velocity::PadVelocity;

mod cmp_shredder;
use crate::cmp_shredder::Shredder;

mod cmp_trajectory;
use crate::cmp_trajectory::Trajectory;

mod cmp_zunda_counter;
use crate::cmp_zunda_counter::Counter;

mod cmp_main_camera;
use crate::cmp_main_camera::MainCamera;
//use crate::cmp_gate_zundamon;

#[derive(Debug, Serialize, Deserialize)]
struct SaveContainer {
    gear_simple: Vec<GearSimple>,
    gear_sorting: Vec<GearSorting>,
    gear_swirl: Vec<GearSwirl>,
    gate_teleport_entrance: Vec<GateTeleportEntrance>,
    gate_teleport_exit: Vec<GateTeleportExit>,
    gate_zundamon: Vec<GateZundamon>,
    pad_velocity: Vec<PadVelocity>,
    shredder: Vec<Shredder>,
}

impl SaveContainer {
    fn new() -> Self {
        Self {
            gear_simple: Vec::new(),
            gear_sorting: Vec::new(),
            gear_swirl: Vec::new(),
            gate_teleport_entrance: Vec::new(),
            gate_teleport_exit: Vec::new(),
            gate_zundamon: Vec::new(),
            pad_velocity: Vec::new(),
            shredder: Vec::new(),
        }
    }
}


#[derive(Resource, Reflect, FromReflect, Clone, Copy, PartialEq, Debug, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
enum EditTool { #[default] Select,
                Translate,
                Scale,
                ScaleDistort,
                }

struct SaveWorldEvent(String);
struct LoadWorldEvent(String);

#[derive(Resource, Reflect, FromReflect, Clone, PartialEq, Debug, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
enum MapObject {
    #[default]None,
    Artillery,
    GearSimple,
    GearSorting,
    GearSwirl,
    GateTeleport(Option<(u32, Color)>),
    GateZundamon,
    PadVelocity(Option<Vec2>),
    Shredder(Vec<Entity>, Vec<Vec2>),
}

#[derive(Resource, Reflect, Clone, PartialEq, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
enum EditContext {
    Edit(Option<Entity>, EditTool),
    Spawn(MapObject)
}

impl Default for EditContext {
    fn default() -> Self {
        EditContext::Edit(None, EditTool::default())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState { #[default] Edit, Game}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zunda shower".into(),
                resolution: (1920., 1080.).into(),
                //mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
        .add_plugin(bevy_framepace::FramepacePlugin)
        .insert_resource(GameAsset::default())
        .insert_resource(EditContext::Edit(None, EditTool::Select))
        //.add_plugin(WorldInspectorPlugin::new())
        //.add_plugin(ResourceInspectorPlugin::<EditContext>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<AppState>()
        .add_system(set_framerate.on_startup())
        .add_system(setup_graphics.on_startup())
        .add_system(setup_sounds.on_startup())
        .add_system(setup_fonts.on_startup())

        .add_system(setup_physics.in_schedule(OnEnter(AppState::Edit)))
        .add_system(game_mode_select.in_set(OnUpdate(AppState::Edit)))
        .add_system(handle_user_input.in_set(OnUpdate(AppState::Edit)))
        .add_system(spawn_map_object.in_set(OnUpdate(AppState::Edit)))

        .add_system(setup_ui.in_schedule(OnEnter(AppState::Game)))

        //.add_system(cmp_ball::system_remove.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_ball::system_trajectory.in_set(OnUpdate(AppState::Game)))

        .add_system(cmp_blood::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<Artillery>()
        .add_system(cmp_artillery::system.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_artillery::system_fire.in_set(OnUpdate(AppState::Game)))

        .register_type::<GateTeleportExit>()
        .register_type::<GateTeleportEntrance>()
        .add_system(cmp_gate_teleport::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<GateZundamon>()
        .add_system(cmp_gate_zundamon::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<PadVelocity>()
        .add_system(cmp_pad_velocity::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<Shredder>()
        .add_system(cmp_shredder::system_move.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_shredder::system_kill.in_set(OnUpdate(AppState::Game)))

        .register_type::<Trajectory>()
        .add_system(cmp_trajectory::system.in_set(OnUpdate(AppState::Game)))

        .add_system(cmp_zunda_counter::system.in_set(OnUpdate(AppState::Game)))

        .add_system(move_camera)
        .add_event::<SaveWorldEvent>()
        .add_system(save_world)
        .add_event::<LoadWorldEvent>()
        .add_system(load_world)
        .run();
}

fn set_framerate(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(60.0);
}

fn save_world(mut save_world_er: EventReader<SaveWorldEvent>,
              gear_simple_q: Query<(&Velocity, &Transform, &GearSimple)>,
              gear_sorting_q: Query<(&Velocity, &Transform, &GearSorting)>,
              gear_swirl_q: Query<(&Velocity, &Transform, &GearSwirl)>,
              gate_teleport_entrance: Query<(&Transform, &GateTeleportEntrance)>,
              gate_teleport_exit: Query<(&Transform, &GateTeleportExit)>,
              gate_zundamon_q: Query<(&Transform, &GateZundamon)>,
              pad_velocity_q: Query<(&Transform, &PadVelocity)>,
              shredder_q: Query<(&Transform, &Shredder)>,
              ) {
    let mut save_container = SaveContainer::new();

    for e in save_world_er.iter() {
        let file = &e.0;
        println!("received event: {}", file);

        for (v, t, e) in gear_simple_q.iter() {
            let mut e = e.clone();
            e.scale = t.scale.truncate().x;
            e.position = t.translation.truncate();
            e.anglevel = v.angvel;
            save_container.gear_simple.push(e.clone());
        }

        for (v, t, e) in gear_sorting_q.iter() {
            let mut e = e.clone();
            e.scale = t.scale.truncate().x;
            e.position = t.translation.truncate();
            e.anglevel = v.angvel;
            save_container.gear_sorting.push(e.clone());
        }

        for (v, t, e) in gear_swirl_q.iter() {
            let mut e = e.clone();
            e.scale = t.scale.truncate().x;
            e.position = t.translation.truncate();
            e.anglevel = v.angvel;
            save_container.gear_swirl.push(e.clone());
        }

        for (t, e) in gate_teleport_entrance.iter() {
            let mut e = e.clone();
            e.size = e.size * t.scale.truncate();
            e.position = t.translation.truncate();
            save_container.gate_teleport_entrance.push(e.clone());
        }

        for (t, e) in gate_teleport_exit.iter() {
            let mut e = e.clone();
            e.size = e.size * t.scale.truncate();
            e.position = t.translation.truncate();
            save_container.gate_teleport_exit.push(e.clone());
        }

        for (t, e) in gate_zundamon_q.iter() {
            let mut e = e.clone();
            e.size = e.size * t.scale.truncate();
            e.position = t.translation.truncate();
            save_container.gate_zundamon.push(e.clone());
        }

        for (t, e) in pad_velocity_q.iter() {
            let mut e = e.clone();
            e.size = e.size * t.scale.truncate();
            e.position = t.translation.truncate();
            save_container.pad_velocity.push(e.clone());
        }

        for (t, e) in shredder_q.iter() {
            let mut e = e.clone();
            e.scale = t.scale.truncate().x;
            save_container.shredder.push(e);
        }

        std::fs::write(file, serde_json::to_string(&save_container).unwrap()).unwrap();
        println!("file saved: {}", file);
    }
}

fn load_world(
    mut load_world_er: EventReader<LoadWorldEvent>,
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
    ) {

    for e in load_world_er.iter() {
        let file = &e.0;
        println!("received event: {}", file);

        let json_str = std::fs::read_to_string(file).unwrap();
        let save_container: SaveContainer = serde_json::from_str(&json_str).unwrap();

        for e in save_container.gear_simple {
            cmp_gear::add_simple(&mut commands, &game_assets, &image_assets, e);
        }

        for e in save_container.gear_sorting {
            cmp_gear::add_sorting(&mut commands, &game_assets, &image_assets, e);
        }

        for e in save_container.gear_swirl {
            cmp_gear::add_swirl(&mut commands, &game_assets, &image_assets, e);
        }

        for e in save_container.gate_teleport_entrance {
            cmp_gate_teleport::add_entrance(&mut commands, e);
        }

        for e in save_container.gate_teleport_exit {
            cmp_gate_teleport::add_exit(&mut commands, e);
        }

        for e in save_container.gate_zundamon {
            cmp_gate_zundamon::add(&mut commands, e);
        }

        for e in save_container.pad_velocity {
            cmp_pad_velocity::add(&mut commands, &game_assets, e);
        }

        for e in save_container.shredder {
            cmp_shredder::add(&mut commands, &game_assets, &image_assets, e);
        }

        //println!("{:?}", save_container);
    }
}

fn load_font(game_assets: &mut GameAsset, font_assets: &mut Assets<Font>, font_bytes: &[u8], name: &str) {
    let source = Font::try_from_bytes(font_bytes.into()).unwrap();
    let handle = font_assets.add(source);
    game_assets.font_handles.insert(name.to_string(), handle);
}

fn setup_fonts(mut commands: Commands, mut game_assets: ResMut<GameAsset>, mut font_assets: ResMut<Assets<Font>>,) {
    let font_mappings = [
        (include_bytes!("../assets/font/FiraMono-Medium.ttf").as_slice(), "font1_handle"),
    ];
    for (path, handle) in font_mappings.iter() {
        load_font(&mut game_assets, &mut font_assets, path, handle);
    }

}

fn load_audio(game_assets: &mut GameAsset, audio_assets: &mut Assets<AudioSource>, audio_bytes: &[u8], name: &str) {
    let source = AudioSource { bytes: audio_bytes.into() };
    let handle = audio_assets.add(source);
    game_assets.audio_handles.insert(name.to_string(), handle);
}

fn setup_sounds(mut commands: Commands, mut game_assets: ResMut<GameAsset>, mut audio_assets: ResMut<Assets<AudioSource>>,) {
    let audio_mappings = [
        (include_bytes!("../assets/audio/zundamon_die1.wav").as_slice(), "zundamon_die1_handle"),
        (include_bytes!("../assets/audio/zundamon_die2.wav").as_slice(), "zundamon_die2_handle"),
        (include_bytes!("../assets/audio/zundamon_die3.wav").as_slice(), "zundamon_die3_handle"),
        (include_bytes!("../assets/audio/zundamon_die4.wav").as_slice(), "zundamon_die4_handle"),
        (include_bytes!("../assets/audio/zundamon_die5.wav").as_slice(), "zundamon_die5_handle"),
        (include_bytes!("../assets/audio/zundamon_die6.wav").as_slice(), "zundamon_die6_handle"),
        (include_bytes!("../assets/audio/zundamon_die7.wav").as_slice(), "zundamon_die7_handle"),
    ];
    for (path, handle) in audio_mappings.iter() {
        load_audio(&mut game_assets, &mut audio_assets, path, handle);
    }

}

fn load_image(game_assets: &mut GameAsset, image_assets: &mut Assets<Image>, image_bytes: &[u8], name: &str) {
    let image = Image::from_buffer(image_bytes, ImageType::MimeType("image/png"), CompressedImageFormats::NONE, true).unwrap();
    let handle = image_assets.add(image);
    game_assets.image_handles.insert(name.to_string(), handle);
}

fn setup_graphics(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>, mut game_assets: ResMut<GameAsset>) {
    // Add a camera so we can see the debug-render.
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let image_mappings = [
        (include_bytes!("../assets/zun1.png").as_slice(), "zun1_handle"),
        (include_bytes!("../assets/zun2.png").as_slice(), "zun2_handle"),
        (include_bytes!("../assets/zun3.png").as_slice(), "zun3_handle"),
        (include_bytes!("../assets/map.png").as_slice(), "map_handle"),
        (include_bytes!("../assets/map2.png").as_slice(), "map2_handle"),
        (include_bytes!("../assets/map3.png").as_slice(), "map3_handle"),
        (include_bytes!("../assets/map4.png").as_slice(), "map4_handle"),
        (include_bytes!("../assets/map_element/artillery_frag1.png").as_slice(), "artillery_frag1"),
        (include_bytes!("../assets/map_element/artillery_frag2.png").as_slice(), "artillery_frag2"),
        (include_bytes!("../assets/map_element/gear_simple_512.png").as_slice(), "gear_simple_512"),
        (include_bytes!("../assets/map_element/gear_sorting_512.png").as_slice(), "gear_sorting_512"),
        (include_bytes!("../assets/map_element/gear_swirl_512.png").as_slice(), "gear_swirl_512"),
        //(include_bytes!("../assets/map_element/shredder_512.png").as_slice(), "shredder_512_handle"),
        (include_bytes!("../assets/map_element/zunda_mochi_512.png").as_slice(), "shredder_512_handle"),
        (include_bytes!("../assets/map_element/pad_velocity.png").as_slice(), "pad_velocity_handle"),
    ];

    for (path, handle) in image_mappings.iter() {
        load_image(&mut game_assets, &mut image_assets, path, handle);
    }
}

fn setup_ui(mut commands: Commands, mut game_assets: Res<GameAsset>) {
    cmp_zunda_counter::add(commands, game_assets);
}

fn add_map(commands: &mut Commands, game_assets: &Res<GameAsset>, image_assets: &Res<Assets<Image>>) {
    //let sprite_handle = game_assets.image_handles.get("map2_handle").unwrap();
    let sprite_handle = game_assets.image_handles.get("map4_handle").unwrap();
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    //let collider = single_convex_polyline_collider_translated(sprite_image).unwrap();
    //let collider = single_polyline_collider_translated(sprite_image);

    //commands
    //    .spawn(SpriteBundle {
    //            texture: sprite_handle.clone(),
    //            ..Default::default()
    //        })
    //    .insert(collider);
    let colliders = multi_polyline_collider_translated(sprite_image);

    let mut entity = commands.spawn((
            SpriteBundle {
                texture: sprite_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.5),
                ..default()
            },
            Interaction::default()
        ));

    for collider in colliders {
        entity.with_children(|children| {
            children.spawn(collider)
                .insert(Restitution::coefficient(0.01))
                .insert(Friction::coefficient(0.001));
        });
    }


}

fn setup_physics(mut commands: Commands, game_assets: Res<GameAsset>, image_assets: Res<Assets<Image>>) {
    /* Create the ground. */
    add_map(&mut commands, &game_assets, &image_assets);
}


/* Project a point inside of a system. */
fn handle_user_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
    mut edit_context: ResMut<EditContext>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut transform_q: Query<(Entity, &mut Transform, &mut BBSize)>,
    ) {

    // Games typically only have one window (the primary window)
    let window = windows_q.single();

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        match edit_context.clone() {
            EditContext::Edit(pick, edit_tool) => {
                if buttons.just_pressed(MouseButton::Left) {
                    for (entity, transform, size) in transform_q.iter() {
                        let sized_width = size.x * transform.scale.x;
                        let sized_height = size.y * transform.scale.y;

                        if collide(transform.translation,
                                   Vec2::new(sized_width, sized_height),
                                   Vec3::new(world_position.x, world_position.y, 0.0),
                                   Vec2::new(0.0, 0.0)).is_some() {
                            println!("clicked {:?}", entity);

                            *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                        } 
                    }
                }

                if buttons.just_released(MouseButton::Left) {
                    if edit_tool != EditTool::Select {
                        *edit_context = EditContext::Edit(pick, EditTool::Select);
                    }
                }

                if pick.is_some() {
                    if keys.pressed(KeyCode::Escape) { 
                        *edit_context = EditContext::Edit(None, EditTool::Select);
                    } else if keys.pressed(KeyCode::Q) {
                        *edit_context = EditContext::Edit(pick, EditTool::Select);
                    } else if keys.pressed(KeyCode::T) {
                        *edit_context = EditContext::Edit(pick, EditTool::Translate);

                    } else if keys.pressed(KeyCode::S) {
                        *edit_context = EditContext::Edit(pick, EditTool::Scale);
                    } else if edit_tool == EditTool::Scale && keys.pressed(KeyCode::D) {
                        *edit_context = EditContext::Edit(pick, EditTool::ScaleDistort);
                    } else if keys.pressed(KeyCode::Delete) {
                        commands.entity(pick.unwrap()).despawn();
                        *edit_context = EditContext::Edit(None, EditTool::Select);
                    }
                }

                match edit_tool {
                    EditTool::Translate => {
                        if let Some(entity) = pick {
                            if let Ok((_, mut transform, _)) = transform_q.get_mut(entity) {
                                transform.translation.x = world_position.x;
                                transform.translation.y = world_position.y;
                            }
                        }
                    }

                    EditTool::Scale => {
                        if let Some(entity) = pick {
                            if let Ok((_, mut transform, bbsize)) = transform_q.get_mut(entity) {
                                let pos = Vec2::new(transform.translation.x, transform.translation.y);
                                let r = pos.distance(world_position);
                                let scale = r / Vec2::ZERO.distance(Vec2::new(bbsize.x / 2.0, bbsize.y / 2.0));
                                println!("scale: {:?}", Vec3::ONE * scale);
                                let scale = (scale * 10.0).round() / 10.0;
                                transform.scale = Vec3::ONE * scale.max(0.1);
                            }
                        }
                    }

                    EditTool::ScaleDistort => {
                        if let Some(entity) = pick {
                            if let Ok((_, mut transform, bbsize)) = transform_q.get_mut(entity) {
                                let pos = transform.translation.truncate();
                                let diff = world_position - pos;
                                let scale = diff / Vec2::ZERO.distance(Vec2::new(bbsize.x / 2.0, bbsize.y / 2.0));
                                let scale = (scale * 10.0).round() / 10.0;
                                transform.scale = Vec3::new(scale.x.abs().max(0.1), scale.y.abs().max(0.1), 1.0);
                            }
                        }
                    }

                    _ => {}
                }
            }

            EditContext::Spawn(map_object) => {
                    match map_object {
                        MapObject::Artillery => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let artillery = Artillery {
                                    scale: 1.0,
                                    position: world_position,
                                    angvel: 0.1,
                                    angle: 0.0,
                                    angle_range: (-0.25 * std::f32::consts::PI, 0.25 * std::f32::consts::PI)
                                };
                                let entity = cmp_artillery::add(&mut commands, &game_assets, &image_assets, artillery);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::GearSimple => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gs = GearSimple {
                                    scale: 1.0, position: world_position, anglevel: -0.5
                                };
                                let entity = cmp_gear::add_simple(&mut commands, &game_assets, &image_assets, gs);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::GearSorting => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gs = GearSorting {
                                    scale: 1.0, position: world_position, anglevel: -0.5
                                };
                                let entity = cmp_gear::add_sorting(&mut commands, &game_assets, &image_assets, gs);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::GearSwirl => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gs = GearSwirl {
                                    scale: 1.0, position: world_position, anglevel: -0.5
                                };
                                let entity = cmp_gear::add_swirl(&mut commands, &game_assets, &image_assets, gs);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::GateTeleport(ctx) => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let mut rng = rand::thread_rng();
                                if ctx.is_none() {
                                    let id = rng.gen_range(0..u32::MAX);
                                    let color = Color::Hsla {
                                        hue: rng.gen_range(0.0..1.0),
                                        saturation: rng.gen_range(0.0..1.0),
                                        lightness: 0.5,
                                        alpha: 1.0 };
                                    let gtent = GateTeleportEntrance {
                                        id,
                                        size: Vec2::new(16.0, 16.0),
                                        position: world_position,
                                        color,
                                    };

                                    println!("GateTeleport entrance added {:?}", gtent);
                                    let entity = cmp_gate_teleport::add_entrance(&mut commands, gtent);
                                    *edit_context = EditContext::Spawn(MapObject::GateTeleport(Some((id, color))));

                                } else {
                                    let (id, color) = ctx.unwrap();
                                    let gtext = GateTeleportExit {
                                        id,
                                        size: Vec2::new(16.0, 16.0),
                                        position: world_position,
                                        color,
                                    };

                                    println!("GateTeleport exit added {:?}", gtext);
                                    let entity = cmp_gate_teleport::add_exit(&mut commands, gtext);
                                    *edit_context = EditContext::Edit(Some(entity), EditTool::Select);

                                }
                            }
                        }

                        MapObject::GateZundamon => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gz = GateZundamon {
                                    size: Vec2::new(128.0, 32.0),
                                    position: world_position,
                                    remain: 100,
                                    prob: 0.5,
                                };
                                let entity = cmp_gate_zundamon::add(&mut commands, gz);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::PadVelocity(origin) => {
                            if buttons.just_pressed(MouseButton::Left) {
                                if origin.is_none() {
                                    *edit_context = EditContext::Spawn(MapObject::PadVelocity(Some(world_position)));
                                } else {
                                    let origin = origin.unwrap();
                                    let dir = (world_position - origin).normalize();
                                    let vel = dir * 200.0;
                                    let pd = PadVelocity {
                                        position: origin,
                                        size: Vec2::new(32.0, 32.0),
                                        velocity: vel
                                    };
                                    let entity = cmp_pad_velocity::add(&mut commands,
                                                                  &game_assets,
                                                                  pd);
                                    *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                                }
                            }
                        }

                        MapObject::Shredder(entities, polyline) => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let mut entities: Vec<Entity> = entities.to_vec();
                                let mut polyline: Vec<Vec2> = polyline.to_vec();

                                let entity = commands
                                    .spawn(SpriteBundle {
                                            sprite: Sprite {
                                                color: Color::BLACK,
                                                custom_size: Some(Vec2::new(8.0, 8.0)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                    .insert(TransformBundle::from(Transform::from_translation(Vec3::from((world_position, 0.0)))))
                                    .insert(BBSize{x: 8.0, y: 8.0})
                                    .id();

                                polyline.push(world_position);
                                entities.push(entity);
                                *edit_context = EditContext::Spawn(MapObject::Shredder(entities, polyline));

                            } else if buttons.just_pressed(MouseButton::Right) {
                                for e in entities {
                                    commands.entity(e).despawn();
                                }

                                if polyline.len() > 0 {
                                    let shredder = Shredder {
                                        scale: 1.0,
                                        polyline,
                                        target_point: 0,
                                        speed: 100.0,
                                        time_offset: 15.0,
                                    };
                                    let entity = cmp_shredder::add(&mut commands,
                                                                  &game_assets,
                                                                  &image_assets,
                                                                  shredder);
                                    *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                                }
                            }

                        }

                        _ => {}
                    }
            }
        }
    }
}

fn move_camera(
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    let (mut projection, mut transform) = q.single_mut();

    {
        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    if ev.y > 0.0 {
                        projection.scale *= 0.95;
                    } else {
                        projection.scale *= 1.05;
                    }
                }
                MouseScrollUnit::Pixel => {}
            }
            println!("{} {}", projection.scale, transform.translation);
        }
    }

    if buttons.pressed(MouseButton::Middle) {
        for ev in motion_evr.iter() {
            transform.translation.x -= ev.delta.x * projection.scale;
            transform.translation.y += ev.delta.y * projection.scale;
        }
    }
    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.scale = projection.scale.clamp(0.1, 10.0);

}

fn spawn_map_object (
    mut egui_contexts: EguiContexts,
    mut edit_mode: ResMut<EditContext>,
    ){

    egui::Window::new("spawn").show(egui_contexts.ctx_mut(), |ui: &mut egui::Ui| {
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Artillery");
            if ui.button("Spawn").clicked() {
                info!("Artillery spawned");
                *edit_mode = EditContext::Spawn(MapObject::Artillery);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Gear simple");
            if ui.button("Spawn").clicked() {
                info!("Gear simple spawned");
                *edit_mode = EditContext::Spawn(MapObject::GearSimple);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Gear sorting");
            if ui.button("Spawn").clicked() {
                info!("Gear sorting spawned");
                *edit_mode = EditContext::Spawn(MapObject::GearSorting);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Gear Swirl");
            if ui.button("Spawn").clicked() {
                info!("Gear swirl spawned");
                *edit_mode = EditContext::Spawn(MapObject::GearSwirl);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Teleport Gate");
            if ui.button("Spawn").clicked() {
                info!("Teleport Gate spawn start");
                *edit_mode = EditContext::Spawn(MapObject::GateTeleport(None));
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Zundamon Gate");
            if ui.button("Spawn").clicked() {
                info!("Zundamon Gate spawned");
                *edit_mode = EditContext::Spawn(MapObject::GateZundamon);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Pad Velocity");
            if ui.button("Spawn").clicked() {
                info!("Pad Velocity spawned");
                *edit_mode = EditContext::Spawn(MapObject::PadVelocity(None));
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Shredder");
            if ui.button("Spawn").clicked() {
                info!("Shredder spawned");
                *edit_mode = EditContext::Spawn(MapObject::Shredder(Vec::new(), Vec::new()));
            }
        });
    });

}

fn game_mode_select (
    mut save_world_ew: EventWriter<SaveWorldEvent>,
    mut load_world_ew: EventWriter<LoadWorldEvent>,
    mut egui_contexts: EguiContexts,
    mut next_app_state: ResMut<NextState<AppState>>,
    ){

    egui::Window::new("GameControl").show(egui_contexts.ctx_mut(), |ui: &mut egui::Ui| {
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Save");
            if ui.button("o").clicked() {
                save_world_ew.send(SaveWorldEvent("assets/map.json".to_string()));
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Load");
            if ui.button("o").clicked() {
                load_world_ew.send(LoadWorldEvent("assets/map.json".to_string()));
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Edit");
            if ui.button("o").clicked() {
                next_app_state.set(AppState::Edit);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Play");
            if ui.button("o").clicked() {
                next_app_state.set(AppState::Game);
            }
        });

    });

}
