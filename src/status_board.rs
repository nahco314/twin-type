use crate::in_game::{
    Event, EventType, History, InGameUI, ProblemCurrentRomajiText, ProblemHiraganaText,
    ProblemTitleText,
};
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    default, AlignItems, Color, Commands, Component, FlexDirection, JustifyContent, NodeBundle,
    Query, Res, Resource, Style, TextAlignment, TextBundle, TextSection, TextStyle, Val, With,
};
use bevy::text::Text;
use bevy::utils::Instant;
use std::time::Duration;

#[derive(Resource)]
pub struct StartTime(pub Instant);

#[derive(Resource)]
pub struct GameTime(pub u64);

#[derive(Component)]
pub struct StatusBoardUI;

#[derive(Component)]
pub struct RemainingTimeUI;

#[derive(Component)]
pub struct ScoreUI;

pub fn create_status_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            (NodeBundle {
                style: Style {
                    height: Val::Percent(10.0),
                    width: Val::Percent(100.0),
                    top: Val::Percent(90.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }),
        )
        .insert(InGameUI)
        .with_children(|parent| {
            parent
                .spawn(
                    (TextBundle::from_section(
                        "残り時間: 60",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 60.0,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style { ..default() })),
                )
                .insert(RemainingTimeUI);
            parent
                .spawn(
                    (TextBundle::from_section(
                        "スコア: 0",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 60.0,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style { ..default() })),
                )
                .insert(ScoreUI);
        });
}

pub fn update_remaining_time(
    mut commands: Commands,
    mut query: Query<&mut Text, With<RemainingTimeUI>>,
    start_time: Res<StartTime>,
    game_time: Res<GameTime>,
) {
    let mut text = query.single_mut();

    let elapsed = start_time.0.elapsed();

    let remaining_time = game_time.0 - elapsed.as_secs();

    text.sections[0].value = format!("残り時間: {}", remaining_time);
}

pub fn calc_score(history: &History) -> i32 {
    let mut res = 0;

    for e in &history.list {
        match e {
            Event {
                type_: EventType::Success { .. },
                ..
            } => {
                res += 2;
            }
            Event {
                type_: EventType::MisType { .. },
                ..
            } => {
                res -= 1;
            }
        }
    }

    res
}

pub fn update_score(
    mut commands: Commands,
    mut query: Query<&mut Text, With<ScoreUI>>,
    history: Res<History>,
) {
    let mut text = query.single_mut();

    let score = calc_score(&history);

    text.sections[0].value = format!("スコア: {}", score);
}
