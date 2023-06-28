//#![windows_subsystem = "windows"]
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy::render::texture::{ImageType, CompressedImageFormats};
use bevy::sprite::collide_aabb::collide;

use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;

use bevy_inspector_egui::bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use std::collections::HashMap;
use rand::prelude::*;

#[derive(Component, Resource, Default, Debug)]
pub struct GameAsset {
    pub image_handles: HashMap<String, Handle<Image>>,
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

// `InspectorOptions` are completely optional
#[derive(Component, Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Player { pick: Option<Entity> }

#[derive(Component)]
struct Ball;
const BALL_SIZE: f32 = 10.0;

#[derive(Component)]
struct BBSize { x: f32, y: f32 }

#[derive(Component, Reflect)]
struct GateZundamon { remain: i32, prob: f32}

#[derive(Component, Reflect, Clone, Copy)]
struct PadVelocity { vel: Vec2 }

#[derive(Resource, Reflect, FromReflect, Clone, Copy, PartialEq, Debug, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
enum EditTool { #[default] Select,
                Translate,
                Scale,
                ScaleDistort,
                }

#[derive(Resource, Reflect, FromReflect, Clone, Copy, PartialEq, Debug, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
enum MapObject {
    #[default]None,
    GearSimple,
    GearSorting,
    GateZundamon,
    PadVelocity(Option<Vec2>),
}

#[derive(Resource, Reflect, Clone, Copy, PartialEq, Debug, InspectorOptions)]
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

fn squeeze(vec3: Vec3) -> Vec2 {
    Vec2::new(vec3.x, vec3.y)
}

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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(ResourceInspectorPlugin::<EditContext>::default())
        .insert_resource(GameAsset::default())
        .insert_resource(EditContext::Edit(None, EditTool::Select))
        .add_state::<AppState>()
        .add_system(game_mode_select)
        .add_system(setup_graphics.on_startup())
        .add_system(setup_physics.in_schedule(OnEnter(AppState::Edit)))
        .add_system(handle_user_input.in_set(OnUpdate(AppState::Edit)))
        .add_system(spawn_map_object.in_set(OnUpdate(AppState::Edit)))
        .add_system(remove_outside_system.in_set(OnUpdate(AppState::Game)))
        .register_type::<GateZundamon>()
        .add_system(gate_zundamon_system.in_set(OnUpdate(AppState::Game)))
        .register_type::<PadVelocity>()
        .add_system(pad_velocity_system.in_set(OnUpdate(AppState::Game)))
        .run();
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
        (include_bytes!("../assets/map_element/gear1024.png").as_slice(), "gear1024_handle"),
        (include_bytes!("../assets/map_element/gear_simple_512.png").as_slice(), "gear_simple_512"),
        (include_bytes!("../assets/map_element/gear_sorting_512.png").as_slice(), "gear_sorting_512"),
    ];

    for (path, handle) in image_mappings.iter() {
        load_image(&mut game_assets, &mut image_assets, path, handle);
    }
}

//fn inspector_ui(player_q: Query<&mut Player>, world: &mut World) {
//    let mut player_entity = player_q.single_mut();
//
//    let mut egui_context = world
//        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
//        .single(world)
//        .clone();
//
//    if let Some(entity) = player_entity.pick {
//        egui::SidePanel::right("inspector")
//            .default_width(250.0)
//            .show(egui_context.get_mut(), |ui| {
//                egui::ScrollArea::vertical().show(ui, |ui| {
//                    ui.heading("Inspector");
//                    bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
//                    ui.allocate_space(ui.available_size());
//                });
//            });
//    }
//}

fn add_ball_random(commands: &mut Commands, game_assets: &Res<GameAsset>, image_assets: &Res<Assets<Image>>, pos: Vec2, r: f32, vel: Vec2) {
    let mut rng = rand::thread_rng();
    let image_vec = vec![ "zun1_handle", "zun2_handle", "zun3_handle" ];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    let sprite_handle = game_assets.image_handles.get(random_image).unwrap();
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    //let collider = single_convex_hull_collider_translated(sprite_image).unwrap();
    let collider = Collider::ball(r);

    commands
        .spawn(Ball)
        .insert(RigidBody::Dynamic)
        .insert(Restitution::coefficient(0.7))
        .insert(Friction::coefficient(0.3))
        .insert(collider)
        .insert(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * (r * 2.0)),
                        ..default()
                    },
                    texture: sprite_handle.clone(),
                    ..default()
        })
        .insert(TransformBundle {
                    local: Transform {
                                translation: Vec3::new(pos.x, pos.y, 1.0),
                                //scale: Vec3::ONE / r,
                                ..Default::default()
                            },
                    ..default()
        })
        .insert(Velocity {
            linvel: vel,
            angvel: 0.0,
        })
    ;
}

fn add_white_wall(commands: &mut Commands, size: Vec2, pos: Vec2) {
    commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            })
        .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)));
}

fn add_zundamon_gate(commands: &mut Commands, size: Vec2, pos: Vec2) -> Entity {
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(GateZundamon {remain: 100, prob: 0.1});

    return entity.id();
}


fn add_pad_velocity(commands: &mut Commands, size: Vec2, pos: Vec2, vel: Vec2) -> Entity {
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    entity
        .insert(TransformBundle::from(Transform::from_xyz(pos.x, pos.y, 0.0)))
        .insert(BBSize{x: size.x, y: size.y})
        .insert(PadVelocity {vel});

    return entity.id();
}


fn add_gear(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            name: &str,
            pos: Vec2,
            r: f32,
            anglevel: f32) -> Entity {
    let sprite_handle = game_assets.image_handles.get(name).unwrap();
    let sprite_image = image_assets.get(sprite_handle).unwrap();
    let colliders = multi_polyline_collider_translated(sprite_image);

    let mut entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: sprite_handle.clone(),
                ..default()
            },
        ));

    entity
        .insert(Interaction::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Restitution::coefficient(0.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: anglevel,
        });

    for collider in colliders {
        entity.with_children(|children| {
            children.spawn(collider)
                .insert(TransformBundle {
                    local: Transform {
                        ..Default::default()
                    },
                    ..default()
                })
            ;
        });
    }

    entity.insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.0),
                    scale: r * Vec3::ONE,
                    ..Default::default()
                },
                ..default()
                },
        );

    entity.insert(BBSize{x: 512.0, y: 512.0});

    return entity.id();

}

fn add_map(commands: &mut Commands, game_assets: &Res<GameAsset>, image_assets: &Res<Assets<Image>>) {
    let sprite_handle = game_assets.image_handles.get("map2_handle").unwrap();
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
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Interaction::default()
        ));

    for collider in colliders {
        entity.with_children(|children| {
            children.spawn(collider);
        });
    }

    //add_gear(commands, &game_assets, &image_assets, "gear_simple_512", Vec2::new(0.0, 0.0), 1.0, -0.5);

}

fn setup_physics(mut commands: Commands, game_assets: Res<GameAsset>, image_assets: Res<Assets<Image>>) {

    /* Create the ground. */
    //add_white_wall(&mut commands, Vec2::new(400.0, 10.0), Vec2::new(0.0, -400.0));
    //add_white_wall(&mut commands, Vec2::new(10.0, 200.0), Vec2::new(200.0, -300.0));
    //add_white_wall(&mut commands, Vec2::new(10.0, 200.0), Vec2::new(-200.0, -300.0));
    add_map(&mut commands, &game_assets, &image_assets);

    /* Create the bouncing ball. */
    //let mut rng = rand::thread_rng();
    //for i in 0..20 {
    //    let x = rng.gen_range(0.0..1200.0) - 600.0;
    //    let y = 600.0 + rng.gen_range(0.0..100.0);
   //    let r = BALL_SIZE;

    //    add_ball_random(&mut commands, &game_assets, &image_assets, Vec2::new(x, y), r, Vec2::new(0.0, 0.0));
    //}
}


fn remove_outside_system(
    mut commands: Commands,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    query: Query<(Entity, &Transform), With<Ball>>,
) {
    let window = windows_q.single();
    let window_width = window.width();
    let window_height = window.height();

    for (entity, position) in query.iter() {
        let window_position_x = position.translation.x + window_width / 2.0;
        let window_position_y = position.translation.y + window_height / 2.0;

        if window_position_x < 0.0 || window_position_x > window_width || window_position_y < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn gate_zundamon_system(
    mut commands: Commands,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
    mut query: Query<(&Transform, &BBSize, &mut GateZundamon)>,
) {
    let mut rng = rand::thread_rng();

    for (transform, bbsize, mut gate_zundamon) in query.iter_mut() {
        if gate_zundamon.remain > 0 {
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < gate_zundamon.prob {
                let size = Vec2::new(bbsize.x, bbsize.y) * squeeze(transform.scale);
                let pos_max = squeeze(transform.translation) + (size / 2.0);
                let pos_min = squeeze(transform.translation) - (size / 2.0);

                let x = rng.gen_range(pos_min.x .. pos_max.x);
                let y = rng.gen_range(pos_min.y .. pos_max.y);

                add_ball_random(&mut commands, &game_assets, &image_assets, Vec2::new(x, y), BALL_SIZE, Vec2::new(0.0, 0.0));
                gate_zundamon.remain -= 1;
            }
        }
    }
}


fn pad_velocity_system(
    rapier_context: Res<RapierContext>,
    mut ball_q: Query<(&mut Velocity, With<Ball>)>,
    pad_velocity_q: Query<(&Transform, &BBSize, &PadVelocity)>,
) {
    for (transform, bbsize, pad_velocity) in pad_velocity_q.iter(){
        let cuboid_size = Vec2::new(bbsize.x, bbsize.y) / 2.0 * squeeze(transform.scale);
        let shape = Collider::cuboid(cuboid_size.x, cuboid_size.y);
        let shape_pos = squeeze(transform.translation);
        let shape_rot = 0.0;
        let filter = QueryFilter::only_dynamic();

        rapier_context.intersections_with_shape(
            shape_pos, shape_rot, &shape, filter, |entity| {
            println!("The entity {:?} intersects our shape.", entity);
                if let Ok(mut vel) = ball_q.get_mut(entity) {
                    vel.0.linvel = pad_velocity.vel;
                }
            true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        });

    }
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
        match *edit_context {
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
                    if keys.pressed(KeyCode::Escape) || keys.pressed(KeyCode::Q) {
                        *edit_context = EditContext::Edit(pick, EditTool::Select);
                    } else if keys.pressed(KeyCode::T) {
                        *edit_context = EditContext::Edit(pick, EditTool::Translate);
                    } else if keys.pressed(KeyCode::S) {
                        *edit_context = EditContext::Edit(pick, EditTool::Scale);
                    } else if edit_tool == EditTool::Scale && keys.pressed(KeyCode::D) {
                        *edit_context = EditContext::Edit(pick, EditTool::ScaleDistort);
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
                                let scale = r / Vec2::new(0.0, 0.0).distance(Vec2::new(bbsize.x / 2.0, bbsize.y / 2.0));
                                transform.scale = Vec3::ONE * scale;
                            }
                        }
                    }

                    EditTool::ScaleDistort => {
                        if let Some(entity) = pick {
                            if let Ok((_, mut transform, bbsize)) = transform_q.get_mut(entity) {
                                let pos = squeeze(transform.translation);
                                let diff = world_position - pos;
                                let scale = diff / Vec2::new(0.0, 0.0).distance(Vec2::new(bbsize.x / 2.0, bbsize.y / 2.0));
                                transform.scale = Vec3::new(scale.x.abs(), scale.y.abs(), 1.0);
                            }
                        }
                    }

                    _ => {}
                }
            }

            EditContext::Spawn(map_object) => {
                if buttons.just_pressed(MouseButton::Left) {
                    match map_object {
                        MapObject::GearSimple => {
                            let entity = add_gear(&mut commands, &game_assets, &image_assets, "gear_simple_512", world_position, 1.0, -0.5);
                            *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                        }

                        MapObject::GearSorting => {
                            let entity = add_gear(&mut commands, &game_assets, &image_assets, "gear_sorting_512", world_position, 1.0, -0.5);
                            *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                        }

                        MapObject::GateZundamon => {
                            let entity = add_zundamon_gate(&mut commands,
                                                           Vec2::new(128.0, 32.0),
                                                           world_position);
                            *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                        }

                        MapObject::PadVelocity(origin) => {
                            if origin.is_none() {
                                *edit_context = EditContext::Spawn(MapObject::PadVelocity(Some(world_position)));
                            } else {
                                let origin = origin.unwrap();
                                let dir = (world_position - origin).normalize();
                                let vel = dir * 300.0;
                                let entity = add_pad_velocity(&mut commands,
                                                               Vec2::new(32.0, 32.0),
                                                               origin,
                                                               vel);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}

fn spawn_map_object (
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    mut edit_mode: ResMut<EditContext>,
    ){

    egui::Window::new("spawn").show(egui_contexts.ctx_mut(), |ui: &mut egui::Ui| {
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

    });

}

fn game_mode_select (
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    mut next_app_state: ResMut<NextState<AppState>>,
    ){

    egui::Window::new("GameMode").show(egui_contexts.ctx_mut(), |ui: &mut egui::Ui| {
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
