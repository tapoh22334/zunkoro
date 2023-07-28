use bevy::prelude::*;

use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball_zundamon::Zundamon;
use crate::cmp_zundamon_fullbody::ZundamonFullbody;

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
                top: Val::Percent(5.0),
                right: Val::Percent(7.5),
                ..default()
            },
            ..default()
        }),
    ).insert(Counter);
}


#[derive(Default)]
pub struct History {
    pub count_max: usize,
}

pub fn system(
    mut history: Local<History>,
    mut text_q: Query<&mut Text, With<Counter>>,
    ball_q: Query<Entity, With<Zundamon>>,
    zundamon_full_q: Query<Entity, With<ZundamonFullbody>>,
){
    let mut text = text_q.single_mut();
    let len = ball_q.iter().len() + zundamon_full_q.iter().len();

    if history.count_max < len {
        history.count_max = len;
    }

    let message = len.to_string() + "/" + history.count_max.to_string().as_str();
    text.sections[0].value = message;
}
