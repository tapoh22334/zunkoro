//#![windows_subsystem = "windows"]
use rand::prelude::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::texture::{ImageType, CompressedImageFormats};
use bevy::sprite::collide_aabb::collide;

use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::bevy_egui::{egui, EguiContexts, EguiPlugin};

use bevy_prototype_lyon::prelude::*;

mod constants;

mod cmp_artillery;
use crate::cmp_artillery::Artillery;

mod cmp_bbsize;
use crate::cmp_bbsize::BBSize;

mod cmp_ball;
mod cmp_ball_zombie;
mod cmp_blood;

mod cmp_block_zombie;
use crate::cmp_block_zombie::BlockZombie;

mod cmp_converter_body;
use crate::cmp_converter_body::ConverterBody;

mod cmp_fuse_time;
use crate::cmp_fuse_time::FuseTime;

mod cmp_gate_teleport;
use crate::cmp_gate_teleport::GateTeleportExit;
use crate::cmp_gate_teleport::GateTeleportEntrance;

mod cmp_gate_zundamon;
use crate::cmp_gate_zundamon::GateZundamon;

mod cmp_gate_zombie;
use crate::cmp_gate_zombie::GateZombie;

mod cmp_game_asset;
use crate::cmp_game_asset::GameAsset;

mod cmp_gear;
use crate::cmp_gear::GearSimple;
use crate::cmp_gear::GearSorting;
use crate::cmp_gear::GearSwirl;

mod cmp_pad_velocity;
use crate::cmp_pad_velocity::PadVelocity;

mod cmp_pad_acceleration;
use crate::cmp_pad_acceleration::PadAcceleration;

mod cmp_polygonal_shape;
use crate::cmp_polygonal_shape::PolygonalShape;
use crate::cmp_polygonal_shape::PolygonalShapeBundle;

mod cmp_primitive_shape;
use crate::cmp_primitive_shape::PrimitiveShape;
use crate::cmp_primitive_shape::PrimitiveShapeBundle;

mod cmp_revolute_joint;
use crate::cmp_revolute_joint::{RevoluteJoint, delay_load};
use crate::cmp_revolute_joint::DelayLoadRevoluteJoint;

mod cmp_rotator;
use crate::cmp_rotator::Rotator;

mod cmp_shredder;
use crate::cmp_shredder::Shredder;

mod cmp_vibrator;
use crate::cmp_vibrator::Vibrator;

mod cmp_trajectory;
use crate::cmp_trajectory::Trajectory;

mod cmp_zunda_counter;

mod cmp_zundamon_fullbody;

mod cmp_main_camera;
use crate::cmp_main_camera::MainCamera;
//use crate::cmp_gate_zundamon;


mod ev_save_load_world;
use crate::ev_save_load_world::SaveWorldEvent;
use crate::ev_save_load_world::{LoadWorldEvent, LoadWorldEventStage2};

mod edit_context;
use crate::edit_context::*;

#[derive(Component)]
pub struct Map;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState { #[default] Edit, Game}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SystemLabel {
    Load1,
    Load2,
}


fn main() {
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zunda shower".into(),
                resolution: (constants::C_WINDOW_SIZE_X, constants::C_WINDOW_SIZE_Y).into(),
                //mode: WindowMode::Borderless Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(bevy_framepace::FramepacePlugin)
        //.add_system(set_framerate.on_startup())
        .insert_resource(GameAsset::default())
        .insert_resource(edit_context::EditContext::Edit(None, edit_context::EditTool::Select))
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ResourceInspectorPlugin::<edit_context::EditContext>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<AppState>()
        .add_system(setup_graphics.on_startup())
        .add_system(setup_sounds.on_startup())
        .add_system(setup_fonts.on_startup())

        .add_system(setup_physics.in_schedule(OnEnter(AppState::Edit)))
        //.add_system(game_mode_select.in_set(OnUpdate(AppState::Edit)))
        .add_system(game_mode_select)
        .add_system(handle_user_input.in_set(OnUpdate(AppState::Edit)))
        .add_system(spawn_map_object.in_set(OnUpdate(AppState::Edit)))

        .add_system(setup_ui.in_schedule(OnEnter(AppState::Game)))

        .add_event::<SaveWorldEvent>()
        .add_event::<LoadWorldEvent>()
        .add_event::<LoadWorldEventStage2>()
        .add_system(ev_save_load_world::forward_event
                    //Please ensure that the transmitter of LoadWorldEvent, which is game_move_select,
                    //is executed before forward_event to process LoadWorldEventStage2 in the next stage.
                    .after(game_mode_select))

        //.add_system(bdl_rotating_shape::load)
        //.add_system(bdl_rotating_shape::save)

        //.add_system(cmp_ball::system_remove.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_ball::system_trajectory.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_ball_zombie::system_infection.in_set(OnUpdate(AppState::Game)))

        .add_system(cmp_blood::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<Artillery>()
        .add_system(cmp_artillery::load)
        .add_system(cmp_artillery::save)
        .add_system(cmp_artillery::system.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_artillery::system_fire.in_set(OnUpdate(AppState::Game)))

        .register_type::<BlockZombie>()
        .add_system(cmp_block_zombie::load)
        .add_system(cmp_block_zombie::save)

        .register_type::<ConverterBody>()
        .add_system(cmp_converter_body::load)
        .add_system(cmp_converter_body::save)
        .add_system(cmp_converter_body::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<FuseTime>()

        .register_type::<GateTeleportExit>()
        .register_type::<GateTeleportEntrance>()
        .add_system(cmp_gate_teleport::load)
        .add_system(cmp_gate_teleport::save)
        .add_system(cmp_gate_teleport::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<GateZombie>()
        .add_system(cmp_gate_zombie::load)
        .add_system(cmp_gate_zombie::save)
        .add_system(cmp_gate_zombie::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<GateZundamon>()
        .add_system(cmp_gate_zundamon::load)
        .add_system(cmp_gate_zundamon::save)
        .add_system(cmp_gate_zundamon::system.in_set(OnUpdate(AppState::Game)))

        .add_system(cmp_gear::load)
        .add_system(cmp_gear::save)

        .register_type::<PadVelocity>()
        .add_system(cmp_pad_velocity::load)
        .add_system(cmp_pad_velocity::save)
        .add_system(cmp_pad_velocity::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<PadAcceleration>()
        .add_system(cmp_pad_acceleration::load)
        .add_system(cmp_pad_acceleration::save)
        .add_system(cmp_pad_acceleration::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<PolygonalShape>()
        .add_system(cmp_polygonal_shape::load)
        .add_system(cmp_polygonal_shape::save)

        .register_type::<PrimitiveShape>()
        .add_system(cmp_primitive_shape::load)
        .add_system(cmp_primitive_shape::save)

        .register_type::<RevoluteJoint>()
        .add_event::<DelayLoadRevoluteJoint>()
        .add_system(cmp_revolute_joint::handle_user_input)
        .add_system(cmp_revolute_joint::load.before(
                        cmp_revolute_joint::delay_load))
        .add_system(cmp_revolute_joint::delay_load)
        .add_system(cmp_revolute_joint::save)

        .register_type::<Rotator>()
        .add_system(cmp_rotator::system.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_rotator::load)
        .add_system(cmp_rotator::save)

        .register_type::<Shredder>()
        .add_system(cmp_shredder::load)
        .add_system(cmp_shredder::save)
        .add_system(cmp_shredder::system_move.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_shredder::system_kill.in_set(OnUpdate(AppState::Game)))

        .register_type::<Trajectory>()
        .add_system(cmp_trajectory::system.in_set(OnUpdate(AppState::Game)))

        .register_type::<Vibrator>()
        .add_system(cmp_vibrator::system.in_set(OnUpdate(AppState::Game)))
        .add_system(cmp_vibrator::load)
        .add_system(cmp_vibrator::save)

        .add_system(cmp_zunda_counter::system.in_set(OnUpdate(AppState::Game)))

        .add_system(cmp_main_camera::move_camera)
        //.add_system(cmp_main_camera::auto_camera)
        .add_system(cmp_main_camera::auto_camera_vertical)

        .run();
}

fn set_framerate(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(60.0);
}


fn load_font(game_assets: &mut GameAsset, font_assets: &mut Assets<Font>, font_bytes: &[u8], name: &str) {
    let source = Font::try_from_bytes(font_bytes.into()).unwrap();
    let handle = font_assets.add(source);
    game_assets.font_handles.insert(name.to_string(), handle);
}

fn setup_fonts(mut game_assets: ResMut<GameAsset>, mut font_assets: ResMut<Assets<Font>>,) {
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

fn setup_sounds(mut game_assets: ResMut<GameAsset>, mut audio_assets: ResMut<Assets<AudioSource>>,) {
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
        (include_bytes!("../assets/zun1_full.png").as_slice(), "zun1_full_handle"),
        (include_bytes!("../assets/zun2_full.png").as_slice(), "zun2_full_handle"),
        (include_bytes!("../assets/zun3_full.png").as_slice(), "zun3_full_handle"),
        (include_bytes!("../assets/zun4_full.png").as_slice(), "zun4_full_handle"),
        (include_bytes!("../assets/zombie1.png").as_slice(), "zombie1_handle"),
        (include_bytes!("../assets/map.png").as_slice(), "map_handle"),
        (include_bytes!("../assets/map2.png").as_slice(), "map2_handle"),
        (include_bytes!("../assets/map3.png").as_slice(), "map3_handle"),
        //(include_bytes!("../assets/map4.png").as_slice(), "map4_handle"),
        //(include_bytes!("../assets/map5.png").as_slice(), "map5_handle"),
        (include_bytes!("../assets/map_element/artillery_frag1.png").as_slice(), "artillery_frag1"),
        (include_bytes!("../assets/map_element/artillery_frag2.png").as_slice(), "artillery_frag2"),
        (include_bytes!("../assets/map_element/gear_simple_512.png").as_slice(), "gear_simple_512"),
        (include_bytes!("../assets/map_element/gear_sorting_512.png").as_slice(), "gear_sorting_512"),
        (include_bytes!("../assets/map_element/gear_swirl_512.png").as_slice(), "gear_swirl_512"),
        //(include_bytes!("../assets/map_element/shredder_512.png").as_slice(), "shredder_512_handle"),
        (include_bytes!("../assets/map_element/zunda_mochi_512.png").as_slice(), "shredder_512_handle"),
        (include_bytes!("../assets/map_element/pad_velocity.png").as_slice(), "pad_velocity_handle"),
        (include_bytes!("../assets/map_element/pad_acceleration.png").as_slice(), "pad_acceleration_handle"),
    ];

    for (bytes, handle) in image_mappings.iter() {
        println!("{:?}", handle);
        load_image(&mut game_assets, &mut image_assets, bytes, handle);
    }
}


fn setup_ui(commands: Commands, game_assets: Res<GameAsset>) {
    cmp_zunda_counter::add(commands, game_assets);
}


fn load_map_polyline() -> Vec<Vec<Vec2>> {
    let map_file = include_bytes!("../assets/map_mini3.map");
    let file_contents = String::from_utf8_lossy(map_file);
    let map: Vec<Vec<Vec2>> = serde_json::from_str(&file_contents).unwrap();

    return map;
}


fn center(polyline: &Vec<Vec2>) -> Vec2 {
    let mut left_bottom = Vec2::new(std::f32::MAX, std::f32::MAX);
    let mut right_top = Vec2::new(std::f32::MIN, std::f32::MIN);

    for point in polyline {
        if point.x < left_bottom.x {
            left_bottom.x = point.x;
        }

        if point.x > right_top.x {
            right_top.x = point.x;
        }

        if point.y < left_bottom.y {
            left_bottom.y = point.y;
        }

        if point.y > right_top.y {
            right_top.y = point.y;
        }
    }

    let translation = (right_top + left_bottom) / 2.0;

    return translation;
}


fn add_map(commands: &mut Commands) {
    let mut polylines = load_map_polyline();

    for mut polyline in polylines {
        let center = center(&polyline);
        let translation = Vec3::from((center, 0.0));
        let rotation = Quat::from_rotation_z(0.0);
        let scale = Vec3::ONE;

        polyline.iter_mut().for_each(|x: &mut Vec2| *x = *x - center);

        let polygonal_shape = PolygonalShape {
                        polygon: polyline,
                        ..default()
                    };

        commands.spawn(PolygonalShapeBundle::from(
                (
                    translation,
                    rotation,
                    scale,
                    polygonal_shape,
                )));
    }
}


fn setup_physics(mut commands: Commands,
                 mut rapier_configuration: ResMut<RapierConfiguration>) {

    println!("{:?}", rapier_configuration.timestep_mode);
    rapier_configuration.timestep_mode = TimestepMode::Variable {
        max_dt: 1.0 / 60.0,
        time_scale: constants::C_SIMULATION_TIME_SCALE,
        substeps: 1 };

    /* Create the ground. */
    println!("setup map");
    println!("end setup map");
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
    mut transform_q: Query<(Entity, &mut Transform, &mut RigidBody, &mut BBSize)>,
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
                    for (entity, transform, _, size) in transform_q.iter() {
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
                    } else if keys.pressed(KeyCode::R) {
                        *edit_context = EditContext::Edit(pick, EditTool::Rotate);
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
                            if let Ok((_, mut transform, _, _)) = transform_q.get_mut(entity) {
                                transform.translation.x = world_position.x;
                                transform.translation.y = world_position.y;
                            }
                        }
                    }

                    EditTool::Rotate => {
                        if let Some(entity) = pick {
                            if let Ok((_, mut transform, _, _)) = transform_q.get_mut(entity) {
                                let pos = transform.translation.truncate();
                                let dir = (world_position - pos).normalize();
                                let angle = Vec2::new(0.0, 1.0).angle_between(dir);
                                transform.rotation = Quat::from_rotation_z(angle);
                            }
                        }
                    }

                    EditTool::Scale => {
                        if let Some(entity) = pick {
                            if let Ok((_, mut transform, _, bbsize)) = transform_q.get_mut(entity) {
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
                            if let Ok((_, mut transform, _, bbsize)) = transform_q.get_mut(entity) {
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
                                let entity = cmp_artillery::add(&mut commands, &game_assets, artillery);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::BlockZombie => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let block_zombie = BlockZombie {
                                    size: Vec2::new(64.0, 64.0),
                                    position: world_position,
                                };
                                let entity = cmp_block_zombie::add(&mut commands, block_zombie);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::ConverterBody => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let cb = ConverterBody {
                                    size: Vec2::new(256.0, 32.0),
                                    position: world_position,
                                };
                                let entity = cmp_converter_body::add(&mut commands, cb);
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
                                    let _ = cmp_gate_teleport::add_entrance(&mut commands, gtent);
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

                        MapObject::GateZombie => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gz = GateZombie {
                                    size: Vec2::new(128.0, 32.0),
                                    position: world_position,
                                    remain: 5,
                                    prob: 0.5,
                                    spawn_offset_sec: 15.0,
                                };
                                let entity = cmp_gate_zombie::add(&mut commands, gz);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
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
                            const PAD_VELOCITY_SPEED: f32 = 400.0;
                            if buttons.just_pressed(MouseButton::Left) {
                                if origin.is_none() {
                                    *edit_context = EditContext::Spawn(MapObject::PadVelocity(Some(world_position)));
                                } else {
                                    let origin = origin.unwrap();
                                    let dir = (world_position - origin).normalize();
                                    //let vel = dir * 600.0;
                                    let pd = PadVelocity {
                                        position: origin,
                                        size: Vec2::new(32.0, 32.0),
                                        direction: dir,
                                        speed: PAD_VELOCITY_SPEED
                                    };
                                    let entity = cmp_pad_velocity::add(&mut commands,
                                                                  &game_assets,
                                                                  pd);
                                    *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                                }
                            }
                        }

                        MapObject::PadAcceleration(origin) => {
                            const PAD_ACCELERATION_ACCELERATION: f32 = 20.0;
                            if buttons.just_pressed(MouseButton::Left) {
                                if origin.is_none() {
                                    *edit_context = EditContext::Spawn(MapObject::PadAcceleration(Some(world_position)));
                                } else {
                                    let origin = origin.unwrap();
                                    let dir = (world_position - origin).normalize();
                                    let pd = PadAcceleration {
                                        position: origin,
                                        size: Vec2::new(32.0, 32.0),
                                        direction: dir,
                                        speed_delta: PAD_ACCELERATION_ACCELERATION,
                                    };
                                    let entity = cmp_pad_acceleration::add(&mut commands,
                                                                  &game_assets,
                                                                  pd);
                                    *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                               }
                            }
                        }

                        MapObject::PrimitiveShape(shape) => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let t = Vec3::from((world_position, 0.0));
                                let r = Quat::from_rotation_z(0.0);
                                let s = Vec3::ONE;
                                let primitive_shape = PrimitiveShape {
                                    shape,
                                };
                                //let entity = cmp_ball::add(&mut commands, &game_assets, world_position, 40.0, Vec2::new(0.0, 0.0));
                                let entity = commands.spawn(PrimitiveShapeBundle::from((t, r, s, primitive_shape))).id();
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
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
                                                                  shredder);
                                    *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                                }
                            }

                        }

                        MapObject::Zundamon => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let entity = cmp_ball::add(&mut commands, &game_assets, world_position, 40.0, Vec2::new(0.0, 0.0));
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::VibratingShape(entity) => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let (entity, transform, _, bbsize) = transform_q.get(entity).unwrap();

                                let t = transform.translation.x;
                                let distance = cmp_primitive_shape::DEFAULT_SIZE_X;
                                let speed = 100.0;

                                let vibrator = Vibrator {
                                    direction: cmp_vibrator::Direction::Horizontal,
                                    speed,
                                    range: (t - distance, t + distance)
                                };
                                //let entity = cmp_primitive_shape::add(&mut commands, primitive_shape);
                                let entity = commands.entity(entity)
                                    .insert(vibrator).id();
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }


                        MapObject::RotatingShape(entity) => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let rotator = Rotator {
                                    angvel: 1.5,
                                };
                                let entity = commands.entity(entity)
                                    .insert(rotator).id();
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }



                        _ => {}
                    }
            }
        }
    }
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
            ui.label("Block Zombie");
            if ui.button("Spawn").clicked() {
                info!("Block Zombie spawned");
                *edit_mode = EditContext::Spawn(MapObject::BlockZombie);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Converter Body ");
            if ui.button("Spawn").clicked() {
                info!("Converter Body spawned");
                *edit_mode = EditContext::Spawn(MapObject::ConverterBody);
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
            ui.label("Zombie Gate");
            if ui.button("Spawn").clicked() {
                info!("Zombie Gate spawned");
                *edit_mode = EditContext::Spawn(MapObject::GateZombie);
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
            ui.label("Pad Acceleration");
            if ui.button("Spawn").clicked() {
                info!("Pad Acceleration spawned");
                *edit_mode = EditContext::Spawn(MapObject::PadAcceleration(None));
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Box");
            if ui.button("o").clicked() {
                *edit_mode = EditContext::Spawn(MapObject::PrimitiveShape(cmp_primitive_shape::Shape::SBox));
            }

            ui.label("Circle");
            if ui.button("o").clicked() {
                *edit_mode = EditContext::Spawn(MapObject::PrimitiveShape(cmp_primitive_shape::Shape::SCircle));
            }

            ui.label("Dia");
            if ui.button("o").clicked() {
                *edit_mode = EditContext::Spawn(MapObject::PrimitiveShape(cmp_primitive_shape::Shape::SDia));
            }

            ui.label("Star");
            if ui.button("o").clicked() {
                *edit_mode = EditContext::Spawn(MapObject::PrimitiveShape(cmp_primitive_shape::Shape::SStar));
            }

            ui.label("Triangle");
            if ui.button("o").clicked() {
                *edit_mode = EditContext::Spawn(MapObject::PrimitiveShape(cmp_primitive_shape::Shape::STriangle));
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Shredder");
            if ui.button("Spawn").clicked() {
                info!("Shredder spawned");
                *edit_mode = EditContext::Spawn(MapObject::Shredder(Vec::new(), Vec::new()));
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Zundamon");
            if ui.button("o").clicked() {
                 *edit_mode = EditContext::Spawn(MapObject::Zundamon);
            }
        });

        ui.separator();
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Vibrator");
            if ui.button("o").clicked() {
                if let EditContext::Edit(entity_opt, edit_tool) = *edit_mode {
                    if entity_opt.is_some() {
                        *edit_mode = EditContext::Spawn(MapObject::VibratingShape(entity_opt.unwrap()));
                    } else {
                        info!("no entity selected");
                    }
                } else {
                    info!("target not selected");
                }
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Rotator");
            if ui.button("o").clicked() {
                if let EditContext::Edit(entity_opt, edit_tool) = *edit_mode {
                    if entity_opt.is_some() {
                        *edit_mode = EditContext::Spawn(MapObject::RotatingShape(entity_opt.unwrap()));
                    } else {
                        info!("no entity selected");
                    }
                } else {
                    info!("target not selected");
                }
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("RevoluteJoint");
            if ui.button("o").clicked() {
                if let EditContext::Edit(entity_opt, edit_tool) = *edit_mode {
                    if entity_opt.is_some() {
                        *edit_mode = EditContext::Spawn(MapObject::RevoluteJoint(entity_opt.unwrap()));
                    } else {
                        info!("no entity selected");
                    }
                } else {
                    info!("target not selected");
                }
            }
        });

    });

}

fn mkdir_if_not_exist(directory_path: &str) {
    use std::fs;
    if let Err(err) = fs::metadata(&directory_path) {
        if err.kind() == std::io::ErrorKind::NotFound {
            fs::create_dir(&directory_path).unwrap();
        } else {
            panic!("unknown error");
        }
    }
}

fn game_mode_select (
    mut commands: Commands,
    mut save_world_ew: EventWriter<SaveWorldEvent>,
    mut load_world_ew: EventWriter<LoadWorldEvent>,
    mut egui_contexts: EguiContexts,
    mut app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut load_json_path: Local<Option<String>>,
    mut save_json_path: Local<Option<String>>,
    ){

    if load_json_path.is_none() {
        *load_json_path = Some("assets/map".to_string());
    }

    if save_json_path.is_none() {
        *save_json_path = Some("assets/map".to_string());
    }

    match app_state.0 {
        AppState::Edit => {
            egui::Window::new("GameControl").show(egui_contexts.ctx_mut(), |ui: &mut egui::Ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label("Add map");
                    if ui.button("o").clicked() {
                        add_map(&mut commands);
                    }
                });

                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label("Save");
                    ui.text_edit_singleline(save_json_path.as_mut().unwrap());
                    if ui.button("o").clicked() {
                        if let Some(ref directory_path) = *save_json_path {
                            mkdir_if_not_exist(directory_path.as_str());
                            save_world_ew.send(SaveWorldEvent(save_json_path.clone().unwrap()));
                        }
                    }
                });

                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label("Load");
                    ui.text_edit_singleline(load_json_path.as_mut().unwrap());
                    if ui.button("o").clicked() {
                        load_world_ew.send(LoadWorldEvent(load_json_path.clone().unwrap()));
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
        _ => {

        }
    }

}
