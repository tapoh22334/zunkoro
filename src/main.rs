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

#[derive(Component)]
struct BBSize { x: f32, y: f32 }

#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
enum AppState { #[default] Edit,
                Game }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zunda shower".into(),
                resolution: (1920., 1080.).into(),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FilterQueryInspectorPlugin::<With<Player>>::default())
        .insert_resource(GameAsset::default())
        .register_type::<Player>()
        .add_state::<AppState>()
        .add_system(setup_graphics.on_startup())
        .add_system(setup_physics.in_schedule(OnEnter(AppState::Edit)))
        .add_system(handle_user_input.in_set(OnUpdate(AppState::Edit)))
        .add_system(spawn_entity.in_set(OnUpdate(AppState::Edit)))
        //.add_system(cursor_position.in_set(OnUpdate(AppState::Edit)))
        //.add_system(cursor_position.in_set(OnUpdate(AppState::Game)))
        //.add_system(remove_outside_system.in_set(OnUpdate(AppState::Game)))
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
        .insert(Restitution::coefficient(0.3))
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
                                translation: Vec3::new(pos.x, pos.y, 0.0),
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

fn add_gear(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            name: &str,
            pos: Vec2,
            r: f32,
            anglevel: f32) {
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
                        translation: Vec3::new(pos.x, pos.y, 0.0),
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

}

fn add_map(commands: &mut Commands, game_assets: &Res<GameAsset>, image_assets: &Res<Assets<Image>>) {
    let sprite_handle = game_assets.image_handles.get("map_handle").unwrap();
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

    add_gear(commands, &game_assets, &image_assets, "gear_simple_512", Vec2::new(0.0, 0.0), 1.0, -0.5);

}

fn setup_physics(mut commands: Commands, game_assets: Res<GameAsset>, image_assets: Res<Assets<Image>>) {

    /* Create the ground. */
    //add_white_wall(&mut commands, Vec2::new(400.0, 10.0), Vec2::new(0.0, -400.0));
    //add_white_wall(&mut commands, Vec2::new(10.0, 200.0), Vec2::new(200.0, -300.0));
    //add_white_wall(&mut commands, Vec2::new(10.0, 200.0), Vec2::new(-200.0, -300.0));
    add_map(&mut commands, &game_assets, &image_assets);

    commands
        .spawn(Player{pick: None})
        //.insert(RigidBody::Dynamic)
        //.insert(Collider::ball(10.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    /* Create the bouncing ball. */
    let mut rng = rand::thread_rng();
    for i in 0..20 {
        let x = rng.gen_range(0.0..1200.0) - 600.0;
        let y = 600.0 + rng.gen_range(0.0..100.0);
        let r = 10.0;

        add_ball_random(&mut commands, &game_assets, &image_assets, Vec2::new(x, y), r, Vec2::new(0.0, 0.0));
    }
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


#[derive( Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States )]
enum EditMode { #[default] Select,
                Translate,
                Scale, }
/* Project a point inside of a system. */
fn handle_user_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut player_q: Query<&mut Player>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut transform_q: Query<(Entity, &mut Transform, &mut BBSize)>,
    mut edit_mode: Local<EditMode>,
    ) {

    let mut player_entity = player_q.single_mut();

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
        if buttons.just_pressed(MouseButton::Left) {
            for (entity, transform, size) in transform_q.iter() {
                let sized_width = size.x * transform.scale.x;
                let sized_height = size.y * transform.scale.y;

                if collide(transform.translation,
                           Vec2::new(sized_width, sized_height),
                           Vec3::new(world_position.x, world_position.y, 0.0),
                           Vec2::new(0.0, 0.0)).is_some() {
                    println!("clicked {:?}", entity);

                    player_entity.pick = Some(entity);
                    *edit_mode = EditMode::Translate;
                } 
            }
        }

        if buttons.just_released(MouseButton::Left) {
            if *edit_mode != EditMode::Select {
                *edit_mode = EditMode::Select;
            }
        }

        if player_entity.pick.is_some() {
            if keys.pressed(KeyCode::Escape) || keys.pressed(KeyCode::Q) {
                *edit_mode = EditMode::Select;
            } else if keys.pressed(KeyCode::T) {
                *edit_mode = EditMode::Translate;
            } else if keys.pressed(KeyCode::S) {
                *edit_mode = EditMode::Scale;
            }
        }

        match *edit_mode {
            EditMode::Translate => {
                if let Some(entity) = player_entity.pick {
                    if let Ok((_, mut transform, _)) = transform_q.get_mut(entity) {
                        transform.translation.x = world_position.x;
                        transform.translation.y = world_position.y;
                    }
                }
            }

            EditMode::Scale => {
                if let Some(entity) = player_entity.pick {
                    if let Ok((_, mut transform, bbsize)) = transform_q.get_mut(entity) {
                        let pos = Vec2::new(transform.translation.x, transform.translation.y);
                        let r = pos.distance(world_position);
                        let scale = r / Vec2::new(0.0, 0.0).distance(Vec2::new(bbsize.x / 2.0, bbsize.y / 2.0));
                        transform.scale = Vec3::ONE * scale;
                    }
                }
            }

            _ => {}
        }

        // stop dragging if mouse button was released
        //if buttons.just_released(MouseButton::Left) {
        //    player_entity.pick = None;
        //}
    }
}

fn spawn_entity (
    mut commands: Commands,
    mut egui_contexts: EguiContexts,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
    ){

    egui::Window::new("spawn").show(egui_contexts.ctx_mut(), |ui: &mut egui::Ui| {
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Gear simple");
            if ui.button("Spawn").clicked() {
                info!("Gear simple spawned");
                add_gear(&mut commands, &game_assets, &image_assets, "gear_simple_512", Vec2::new(0.0, 0.0), 1.0, -0.5);
            }
        });

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("Gear sorting");
            if ui.button("Spawn").clicked() {
                info!("Gear sorting spawned");
                add_gear(&mut commands, &game_assets, &image_assets, "gear_sorting_512", Vec2::new(0.0, 0.0), 1.0, -0.5);
            }
        });

    });

}
//fn cursor_position(
//    mut commands: Commands,
//    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
//    buttons: Res<Input<MouseButton>>,
//    game_assets: Res<GameAsset>,
//    image_assets: Res<Assets<Image>>,
//    windows_q: Query<&Window, With<PrimaryWindow>>,
//    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//    //transform_q: Query<(Entity, &Transform)>,
//) {
//    //for transform in player.iter() {
//    //    println!("Ball altitude: {}", transform.translation.y);
//    //}
//    let (mut player_position, mut player_velocity) = player.single_mut();
//
//    // Games typically only have one window (the primary window)
//    let window = windows_q.single();
//
//    // get the camera info and transform
//    // assuming there is exactly one main camera entity, so query::single() is OK
//    let (camera, camera_transform) = camera_q.single();
//
//    // check if the cursor is inside the window and get its position
//    // then, ask bevy to convert into world coordinates, and truncate to discard Z
//    if let Some(world_position) = window.cursor_position()
//        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//        .map(|ray| ray.origin.truncate())
//    {
//        //let k_coefficient = 50.0;
//        //let velocity_x = (world_position.x - player_position.translation.x) * k_coefficient;
//        //let velocity_y = (world_position.y - player_position.translation.y) * k_coefficient;
//        //player_velocity.linvel = Vec2::new(velocity_x, velocity_y);
//
//        //if buttons.just_pressed(MouseButton::Left) {
//        //    add_ball(&mut commands, &game_assets, &image_assets, world_position, Vec2::new(0.0, 500.0))
//        //}
//    }
//
//    //let mut rng = rand::thread_rng();
//    //if rng.gen::<f32>() < 0.01 {
//    //    let x = rng.gen_range(0.0..1400.0) - 700.0;
//    //    let y = 600.0 + rng.gen_range(0.0..100.0);
//    //    add_ball(&mut commands, &game_assets, &image_assets, Vec2::new(x, y), Vec2::new(0.0, 0.0))
//    //}
//
//}
