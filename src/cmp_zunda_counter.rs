use bevy::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball::Ball;

#[derive(Component)]
pub struct Counter;

pub fn add(
    mut command: Commands,
    game_assets: Res<GameAsset>,
){
    let font = game_assets.font_handles.get("font1_handle").unwrap();
    let message = "";

    command.spawn(TextBundle::from_section(
        message,
        TextStyle {
            font_size: 60.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            font: font.clone(),
            ..default()
        })
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Percent(5.0),
                right: Val::Percent(50.),
                ..default()
            },
            ..default()
        }),
    ).insert(Counter);
}


pub fn system(
    mut text_q: Query<&mut Text, With<Counter>>,
    ball_q: Query<Entity, With<Ball>>,
){
    let mut text = text_q.single_mut();
    let len = ball_q.iter().len();

    let message = len.to_string() + "/100 Zun";
    text.sections[0].value = message;
}
