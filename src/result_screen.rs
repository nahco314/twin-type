use crate::in_game::{
    Event, EventType, History, InGameUI, Level, LevelResource, ProblemCurrentRomajiText,
    ProblemHiraganaText, ProblemTitleText, UsedWordIndexes,
};
use crate::main_menu::MainMenuUI;
use crate::status_board::{calc_score, GameTime, StartTime};
use crate::AppState;
use bevy::asset::AssetServer;
use bevy::hierarchy::Children;
use bevy::prelude::{
    default, AlignItems, BackgroundColor, BorderColor, BuildChildren, Button, ButtonBundle,
    Changed, ChildBuilder, Color, Commands, Component, DespawnRecursiveExt, Entity, FlexDirection,
    Interaction, JustifyContent, NextState, NodeBundle, Query, Res, ResMut, Style, Text,
    TextAlignment, TextBundle, TextSection, TextStyle, UiRect, Val, With,
};
use std::fmt::format;
use std::time::Instant;

#[derive(Component)]
pub struct ResultScreenUI;

pub fn result_screen_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<ResultScreenUI>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn sign(left: i32, right: i32, neg: bool) -> String {
    let left = if !neg { left } else { -left };
    let right = if !neg { right } else { -right };

    if left > right {
        ">".to_string()
    } else if left == right {
        "=".to_string()
    } else {
        "<".to_string()
    }
}

pub fn start_result_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    history: Res<History>,
) {
    let mut left_type_count = 0;
    let mut left_mistype_count = 0;
    let mut right_type_count = 0;
    let mut right_mistype_count = 0;

    for e in &history.list {
        match e {
            &Event {
                is_left: true,
                type_: EventType::Success { .. },
                ..
            } => {
                left_type_count += 1;
            }
            &Event {
                is_left: true,
                type_: EventType::MisType { .. },
                ..
            } => {
                left_mistype_count += 1;
            }
            &Event {
                is_left: false,
                type_: EventType::Success { .. },
                ..
            } => {
                right_type_count += 1;
            }
            &Event {
                is_left: false,
                type_: EventType::MisType { .. },
                ..
            } => {
                right_mistype_count += 1;
            }
        }
    }

    let lr_result_style = TextStyle {
        font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
        font_size: 60.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    };

    let create_lr_result = |title, left_cnt, right_cnt, neg, parent: &mut ChildBuilder| {
        let mut left_style = lr_result_style.clone();
        let mut right_style = lr_result_style.clone();

        if left_cnt == right_cnt {
        } else if (left_cnt > right_cnt) ^ neg {
            left_style.color = Color::rgb(0.5, 0.9, 0.5);
        } else if (left_cnt < right_cnt) ^ neg {
            right_style.color = Color::rgb(0.5, 0.9, 0.5);
        }

        parent
            .spawn(NodeBundle {
                style: Style {
                    height: Val::Auto,
                    width: Val::Percent(60.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(format!("左側{}数: {}", title, left_cnt), left_style)
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(Style { ..default() }),
                );

                parent.spawn(
                    TextBundle::from_section(
                        sign(left_cnt, right_cnt, neg),
                        lr_result_style.clone(),
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style { ..default() }),
                );

                parent.spawn(
                    TextBundle::from_section(
                        format!("右側{}数: {}", title, right_cnt),
                        right_style,
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style { ..default() }),
                );
            });
    };

    commands
        .spawn(
            (NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ResultScreenUI)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "最終スコア",
                    TextStyle {
                        font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                        font_size: 60.0,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    top: Val::Percent(-10.0),
                    ..default()
                }),
            );
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("{}", calc_score(&history)),
                    TextStyle {
                        font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                        font_size: 120.0,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    top: Val::Percent(-10.0),
                    ..default()
                }),
            );
        })
        .with_children(|p| create_lr_result("タイプ", left_type_count, right_type_count, false, p))
        .with_children(|p| {
            create_lr_result("ミス", left_mistype_count, right_mistype_count, true, p)
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        top: Val::Percent(10.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "戻る",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 72.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(ResultScreenUI);
        });
}

pub fn exit_result_screen(mut commands: Commands, query: Query<Entity, With<ResultScreenUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
