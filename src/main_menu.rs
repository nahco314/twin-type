use crate::AppState;
use bevy::asset::AssetServer;
use bevy::prelude::{
    default, AlignItems, BorderColor, ButtonBundle, Camera2dBundle, Color, Commands, Component,
    FlexDirection, JustifyContent, NodeBundle, Res, Style, TextAlignment, TextBundle, TextStyle,
    UiRect, Val,
};

use crate::in_game::{Level, LevelResource, Something};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct NormalButton;

#[derive(Component)]
pub struct HardButton;

#[derive(Component)]
pub struct ExtraHardButton;

pub fn main_menu_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<MainMenuUI>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::InGame);
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

pub fn main_menu_normal_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            With<MainMenuUI>,
            With<NormalButton>,
        ),
    >,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(LevelResource(Level::Easy));
            }
            _ => {}
        }
    }
}

pub fn main_menu_hard_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            With<MainMenuUI>,
            With<HardButton>,
        ),
    >,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(LevelResource(Level::Hard));
            }
            _ => {}
        }
    }
}

pub fn main_menu_extra_hard_button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            With<MainMenuUI>,
            With<ExtraHardButton>,
        ),
    >,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(LevelResource(Level::ExtraHard));
            }
            _ => {}
        }
    }
}

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Twin Type",
                    TextStyle {
                        font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                        font_size: 300.0,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    top: Val::Percent(-18.0),
                    ..default()
                }),
            );
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        top: Val::Percent(-15.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "ノーマル",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 72.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(MainMenuUI)
                .insert(NormalButton);
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        top: Val::Percent(-15.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "ハード",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 72.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(MainMenuUI)
                .insert(HardButton);
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        top: Val::Percent(-15.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "超ハード",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 72.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(MainMenuUI)
                .insert(ExtraHardButton);
        })
        .insert(MainMenuUI);
}

pub fn exit_main_menu(mut commands: Commands, query: Query<Entity, With<Something>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
