//#![windows_subsystem = "windows"]
use serde::{Serialize, Deserialize};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::texture::{ImageType, CompressedImageFormats};
use bevy::sprite::collide_aabb::collide;

use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;

use bevy_inspector_egui::bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::prelude::*;

// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
// use bevy_inspector_egui::quick::ResourceInspectorPlugin;

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

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
struct GearSimple {scale: f32, position: Vec2, anglevel: f32}

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
struct GearSorting {scale: f32, position: Vec2, anglevel: f32}

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
struct GateZundamon {size: Vec2, position: Vec2, remain: i32, prob: f32 }

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
struct PadVelocity {size: Vec2, position: Vec2, velocity: Vec2}

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
struct Shredder {scale: f32, polyline: Vec<Vec2>, target_point: usize, speed: f32}

#[derive(Debug, Serialize, Deserialize)]
struct SaveContainer {
    gear_simple: Vec<GearSimple>,
    gear_sorting: Vec<GearSorting>,
    gate_zundamon: Vec<GateZundamon>,
    pad_velocity: Vec<PadVelocity>,
    shredder: Vec<Shredder>,
}

impl SaveContainer {
    fn new() -> Self {
        Self {
            gear_simple: Vec::new(),
            gear_sorting: Vec::new(),
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
    GearSimple,
    GearSorting,
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(GameAsset::default())
        .insert_resource(EditContext::Edit(None, EditTool::Select))
        //.add_plugin(WorldInspectorPlugin::new())
        //.add_plugin(ResourceInspectorPlugin::<EditContext>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<AppState>()
        .add_system(setup_graphics.on_startup())

        .add_system(setup_physics.in_schedule(OnEnter(AppState::Edit)))
        .add_system(game_mode_select.in_set(OnUpdate(AppState::Edit)))
        .add_system(handle_user_input.in_set(OnUpdate(AppState::Edit)))
        .add_system(spawn_map_object.in_set(OnUpdate(AppState::Edit)))

        .add_system(remove_outside_system.in_set(OnUpdate(AppState::Game)))

        .register_type::<GateZundamon>()
        .add_system(gate_zundamon_system.in_set(OnUpdate(AppState::Game)))

        .register_type::<PadVelocity>()
        .add_system(pad_velocity_system.in_set(OnUpdate(AppState::Game)))

        .register_type::<Shredder>()
        .add_system(shredder_move_system.in_set(OnUpdate(AppState::Game)))
        .add_system(shredder_system.in_set(OnUpdate(AppState::Game)))

        .add_event::<SaveWorldEvent>()
        .add_system(save_world)
        .add_event::<LoadWorldEvent>()
        .add_system(load_world)
        .run();
}

fn save_world(mut save_world_er: EventReader<SaveWorldEvent>,
              gear_simple_q: Query<(&Velocity, &Transform, &GearSimple)>,
              gear_sorting_q: Query<(&Velocity, &Transform, &GearSorting)>,
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
            add_gear_simple(&mut commands, &game_assets, &image_assets, e);
        }

        for e in save_container.gear_sorting {
            add_gear_sorting(&mut commands, &game_assets, &image_assets, e);
        }

        for e in save_container.gate_zundamon {
            add_zundamon_gate(&mut commands, e);
        }

        for e in save_container.pad_velocity {
            add_pad_velocity(&mut commands, &game_assets, e);
        }

        for e in save_container.shredder {
            add_shredder(&mut commands, &game_assets, &image_assets, e);
        }

        //println!("{:?}", save_container);
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
        (include_bytes!("../assets/map_element/gear_simple_512.png").as_slice(), "gear_simple_512"),
        (include_bytes!("../assets/map_element/gear_sorting_512.png").as_slice(), "gear_sorting_512"),
        (include_bytes!("../assets/map_element/shredder_512.png").as_slice(), "shredder_512_handle"),
        (include_bytes!("../assets/map_element/pad_velocity.png").as_slice(), "pad_velocity_handle"),
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

fn add_ball_random(commands: &mut Commands, game_assets: &Res<GameAsset>, pos: Vec2, r: f32, vel: Vec2) {
    let mut rng = rand::thread_rng();
    let image_vec = vec![ "zun1_handle", "zun2_handle", "zun3_handle" ];
    let random_index = rng.gen_range(0..image_vec.len());
    let random_image = image_vec[random_index];

    let sprite_handle = game_assets.image_handles.get(random_image).unwrap();
    //let sprite_image = image_assets.get(sprite_handle).unwrap();
    //let collider = single_convex_hull_collider_translated(sprite_image).unwrap();
    let collider = Collider::ball(r);

    commands
        .spawn(Ball)
        .insert(RigidBody::Dynamic)
        .insert(Restitution::coefficient(0.7))
        .insert(Friction::coefficient(0.05))
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


fn add_zundamon_gate(commands: &mut Commands, gate_zundamon: GateZundamon) -> Entity {
    let size = gate_zundamon.size;
    let pos = gate_zundamon.position;
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
        .insert(gate_zundamon);

    return entity.id();
}


fn add_pad_velocity(commands: &mut Commands,
                    game_assets: &Res<GameAsset>,
                    pad_velocity: PadVelocity) -> Entity {
    let size = pad_velocity.size;
    let pos = pad_velocity.position;
    let vel = pad_velocity.velocity;

    let sprite_handle = game_assets.image_handles.get("pad_velocity_handle").unwrap();
    let mut entity = commands
        .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                texture: sprite_handle.clone(),
                ..Default::default()
            });
        // .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))

    let angle = Vec2::new(0.0, 1.0).angle_between(vel.normalize());
    entity
        .insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.0),
                    rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle),
                    ..Default::default()
                },
                ..default()
                })
        .insert(BBSize{x: size.x, y: size.y})
        .insert(pad_velocity);

    return entity.id();
}

fn add_shredder(commands: &mut Commands,
                    game_assets: &Res<GameAsset>,
                    image_assets: &Res<Assets<Image>>,
                    shredder: Shredder) -> Entity {
    let sprite_handle = game_assets.image_handles.get("shredder_512_handle").unwrap();
    //let sprite_image = image_assets.get(sprite_handle).unwrap();
    //let colliders = multi_polyline_collider_translated(sprite_image);

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
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: -2.0,
        });

    //entity.insert(Collider::ball(512.0 / 2.0));
    //for collider in colliders {
    //    entity.with_children(|children| {
    //        children.spawn(collider)
    //            .insert(TransformBundle {
    //                local: Transform {
    //                    ..Default::default()
    //                },
    //                ..default()
    //            })
    //        ;
    //    });
    //}

    entity.insert(TransformBundle {
                local: Transform {
                    translation: Vec3::new(shredder.polyline[0].x, shredder.polyline[0].y, 0.0),
                    scale: shredder.scale * Vec3::ONE,
                    ..Default::default()
                },
                ..default()
                },
        );

    entity.insert(BBSize{x: 512.0, y: 512.0});
    entity.insert(shredder);

    return entity.id();

}

fn add_gear_simple(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            gear_simple: GearSimple) -> Entity {

    let id = add_gear(commands,
             game_assets,
             image_assets,
             "gear_simple_512",
             gear_simple.position,
             gear_simple.scale,
             gear_simple.anglevel);

    let id = commands.entity(id).insert(gear_simple).id();
    return id;
}

fn add_gear_sorting(commands: &mut Commands,
            game_assets: &Res<GameAsset>,
            image_assets: &Res<Assets<Image>>,
            gear_sorting: GearSorting) -> Entity {

    let id = add_gear(commands,
             game_assets,
             image_assets,
             "gear_sorting_512",
             gear_sorting.position,
             gear_sorting.scale,
             gear_sorting.anglevel);

    let id = commands.entity(id).insert(gear_sorting).id();
    return id;
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
                transform: Transform::from_xyz(0.0, 0.0, 0.5),
                ..default()
            },
            Interaction::default()
        ));

    for collider in colliders {
        entity.with_children(|children| {
            children.spawn(collider)
                .insert(Friction::coefficient(0.01));
        });
    }

    //add_gear(commands, &game_assets, &image_assets, "gear_simple_512", Vec2::new(0.0, 0.0), 1.0, -0.5);

}

fn setup_physics(mut commands: Commands, game_assets: Res<GameAsset>, image_assets: Res<Assets<Image>>) {
    /* Create the ground. */
    add_map(&mut commands, &game_assets, &image_assets);
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
    mut query: Query<(&Transform, &BBSize, &mut GateZundamon)>,
) {
    let mut rng = rand::thread_rng();

    for (transform, bbsize, mut gate_zundamon) in query.iter_mut() {
        if gate_zundamon.remain > 0 {
            if rng.gen::<f32>() < gate_zundamon.prob {
                let size = Vec2::new(bbsize.x, bbsize.y) * transform.scale.truncate();
                let pos_max = transform.translation.truncate() + (size / 2.0);
                let pos_min = transform.translation.truncate() - (size / 2.0);

                let x = rng.gen_range(pos_min.x .. pos_max.x);
                let y = rng.gen_range(pos_min.y .. pos_max.y);

                add_ball_random(&mut commands, &game_assets, Vec2::new(x, y), BALL_SIZE, Vec2::new(0.0, 0.0));
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
        let cuboid_size = Vec2::new(bbsize.x, bbsize.y) / 2.0 * transform.scale.truncate();
        let shape = Collider::cuboid(cuboid_size.x, cuboid_size.y);
        let shape_pos = transform.translation.truncate();
        let (shape_rot, _, _) = transform.rotation.to_euler(EulerRot::ZXY);
        let filter = QueryFilter::only_dynamic();

        rapier_context.intersections_with_shape(
            shape_pos, shape_rot, &shape, filter, |entity| {
                if let Ok(mut vel) = ball_q.get_mut(entity) {
                    vel.0.linvel = pad_velocity.velocity;
                }
            true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        });

    }
}

fn shredder_move_system(
    mut shredder_q: Query<(&mut Transform, &mut Velocity, &mut BBSize, &mut Shredder)>,
) {
    for (mut t, mut v, _, mut shredder) in shredder_q.iter_mut() {
        if shredder.target_point < shredder.polyline.len() - 1 { 
            let target_pos = shredder.polyline[shredder.target_point];
            let distance = t.translation.truncate().distance(target_pos);
            let dir = (target_pos - t.translation.truncate()) / distance;

            let distance_thresh = 10.0;
            if distance < distance_thresh {
                shredder.target_point += 1;
            } else {
                v.linvel = dir * shredder.speed;
            }

        } else {
            v.linvel = Vec2::ZERO;
        }

        println!("{:?}", v.linvel);
    }
}

fn shredder_system(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    ball_q: Query<(Entity, With<Ball>)>,
    shredder_q: Query<(&Transform, &BBSize, &Shredder)>,
) {
    for (transform, bbsize, pad_velocity) in shredder_q.iter(){
        let r = bbsize.x / 2.0 * transform.scale.truncate().x * 0.9;
        let shape = Collider::ball(r);
        let shape_pos = transform.translation.truncate();
        let shape_rot = 0.0;
        let filter = QueryFilter::only_dynamic();

        rapier_context.intersections_with_shape(
            shape_pos, shape_rot, &shape, filter, |entity| {
                commands.entity(entity).despawn();
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
                    if keys.pressed(KeyCode::Escape) || keys.pressed(KeyCode::Q) {
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
                                transform.scale = Vec3::new(scale.x.abs().max(0.1), scale.y.abs().max(0.1), 1.0);
                            }
                        }
                    }

                    _ => {}
                }
            }

            EditContext::Spawn(map_object) => {
                    match map_object {
                        MapObject::GearSimple => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gs = GearSimple {
                                    scale: 1.0, position: world_position, anglevel: -0.5
                                };
                                let entity = add_gear_simple(&mut commands, &game_assets, &image_assets, gs);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::GearSorting => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gs = GearSorting {
                                    scale: 1.0, position: world_position, anglevel: -0.5
                                };
                                let entity = add_gear_sorting(&mut commands, &game_assets, &image_assets, gs);
                                *edit_context = EditContext::Edit(Some(entity), EditTool::Select);
                            }
                        }

                        MapObject::GateZundamon => {
                            if buttons.just_pressed(MouseButton::Left) {
                                let gz = GateZundamon {
                                    size: Vec2::new(128.0, 32.0),
                                    position: world_position,
                                    remain: 100,
                                    prob: 0.1
                                };
                                let entity = add_zundamon_gate(&mut commands, gz);
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
                                    let vel = dir * 300.0;
                                    let pd = PadVelocity {
                                        position: origin,
                                        size: Vec2::new(32.0, 32.0),
                                        velocity: vel
                                    };
                                    let entity = add_pad_velocity(&mut commands,
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
                                    };
                                    let entity = add_shredder(&mut commands,
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

fn spawn_map_object (
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
