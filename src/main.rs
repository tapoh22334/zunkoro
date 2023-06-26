//#![windows_subsystem = "windows"]

use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy::render::texture::{ImageType, CompressedImageFormats};

use rand::prelude::*;

#[derive(Component, Resource, Default, Debug)]
pub struct GameAsset {
    pub image_handles: HashMap<String, Handle<Image>>,
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

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
        .insert_resource(GameAsset::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(cursor_position)
        .add_system(remove_outside_system)
        .run();
}

fn setup_graphics(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>, mut game_assets: ResMut<GameAsset>) {
    // Add a camera so we can see the debug-render.
    commands.spawn((Camera2dBundle::default(), MainCamera));

    //let handle: Handle<Image> = asset_server.load("zun1.png");
    //println!("check1 {:?}", handle);

    //let handle: Handle<Image> = asset_server.load("zun1.png");
    //println!("check2 {:?}", handle);

    let image_bytes = include_bytes!("../assets/zun1.png");
    let image1 = Image::from_buffer(image_bytes, ImageType::MimeType("image/png"), CompressedImageFormats::NONE, true).unwrap();

    let image_bytes = include_bytes!("../assets/zun2.png");
    let image2 = Image::from_buffer(image_bytes, ImageType::MimeType("image/png"), CompressedImageFormats::NONE, true).unwrap();

    let image_bytes = include_bytes!("../assets/zun3.png");
    let image3 = Image::from_buffer(image_bytes, ImageType::MimeType("image/png"), CompressedImageFormats::NONE, true).unwrap();

    game_assets.image_handles = HashMap::from([
        ( "zun1_handle".into(), image_assets.add(image1),),
        ( "zun2_handle".into(), image_assets.add(image2),),
        ( "zun3_handle".into(), image_assets.add(image3),),
    ]);

}

fn add_ball(commands: &mut Commands, game_assets: &Res<GameAsset>, image_assets: &Res<Assets<Image>>, pos: Vec2, vel: Vec2) {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(3.0..50.0);

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

fn setup_physics(mut commands: Commands, game_assets: Res<GameAsset>, image_assets: Res<Assets<Image>>) {

    /* Create the ground. */
    add_white_wall(&mut commands, Vec2::new(400.0, 10.0), Vec2::new(0.0, -400.0));
    add_white_wall(&mut commands, Vec2::new(10.0, 200.0), Vec2::new(200.0, -300.0));
    add_white_wall(&mut commands, Vec2::new(10.0, 200.0), Vec2::new(-200.0, -300.0));

    commands
        .spawn(Player)
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
        add_ball(&mut commands, &game_assets, &image_assets, Vec2::new(x, y), Vec2::new(0.0, 0.0));
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

        // エンティティが画面外に出た場合に削除する
        if window_position_x < 0.0 || window_position_x > window_width || window_position_y < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}


fn cursor_position(
    mut commands: Commands,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
    buttons: Res<Input<MouseButton>>,
    game_assets: Res<GameAsset>,
    image_assets: Res<Assets<Image>>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    //for transform in player.iter() {
    //    println!("Ball altitude: {}", transform.translation.y);
    //}
    let (mut player_position, mut player_velocity) = player.single_mut();

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
        let k_coefficient = 50.0;
        let velocity_x = (world_position.x - player_position.translation.x) * k_coefficient;
        let velocity_y = (world_position.y - player_position.translation.y) * k_coefficient;
        player_velocity.linvel = Vec2::new(velocity_x, velocity_y);

        if buttons.just_pressed(MouseButton::Left) {
            add_ball(&mut commands, &game_assets, &image_assets, world_position, Vec2::new(0.0, 500.0))
        }
    }

    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() < 0.1 {
        let x = rng.gen_range(0.0..1400.0) - 700.0;
        let y = 600.0 + rng.gen_range(0.0..100.0);
        add_ball(&mut commands, &game_assets, &image_assets, Vec2::new(x, y), Vec2::new(0.0, 0.0))
    }

}
