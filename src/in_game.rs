use crate::main_menu::MainMenuUI;
use crate::AppState;
use bevy::asset::{AssetContainer, AssetServer};
use bevy::prelude::{
    default, AlignItems, Commands, Component, Entity, FlexDirection, JustifyContent, NodeBundle,
    NonSend, Query, Res, State, Style, TextAlignment, TextBundle, TextStyle, Val, With, World,
};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use tpg_kb_util::{listen_events, KBEvent, KBEventType};

use crate::word_typing::sharing::{get_sharing, Player};
use crate::word_typing::{key_num_to_char, set_new_problem, CurrentRomaji, Hiragana, Title};
use bevy::ecs::system::RunSystemOnce;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use successive_romaji::parse_hiragana_with_buf;

pub struct ListenResource {
    thread: JoinHandle<()>,
    rx: Receiver<KBEvent>,
}

#[derive(Component)]
pub struct Something;

#[derive(Component)]
pub struct ProblemTitleText;

#[derive(Component)]
pub struct ProblemHiraganaText;

#[derive(Component)]
pub struct ProblemCurrentRomajiText;

pub fn setup_listen_kb(world: &mut World) {
    let (tx, rx) = mpsc::channel();
    let t = thread::spawn(move || {
        listen_events(tx);
    });
    let instance = ListenResource { thread: t, rx };
    world.insert_non_send_resource(instance);
}

pub fn start_ingame(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

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
            parent
                .spawn(
                    TextBundle::from_section(
                        "スケルトン",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 120.0,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        top: Val::Percent(-25.0),
                        ..default()
                    }),
                )
                .insert(ProblemTitleText);
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "すけるとん",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 72.0,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        top: Val::Percent(-25.0),
                        ..default()
                    }),
                )
                .insert(ProblemHiraganaText);
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new(
                            "suker",
                            TextStyle {
                                font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                                font_size: 80.0,
                                color: Color::RED,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "utonn",
                            TextStyle {
                                font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                                font_size: 80.0,
                                color: Color::BLUE,
                                ..default()
                            },
                        ),
                    ])
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        top: Val::Percent(-25.0),
                        ..default()
                    }),
                )
                .insert(ProblemCurrentRomajiText);
        });

    set_new_problem(commands);
}

pub fn update_listen_kb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Title, &Hiragana, &mut CurrentRomaji)>,
    lr: NonSend<ListenResource>,
) {
    if let Ok(res) = lr.rx.try_recv() {
        if let KBEvent {
            keyboard_id: kb_id,
            event_type: KBEventType::KeyPressed { key_num },
        } = res
        {
            let Some(c) = key_num_to_char(key_num) else {
                return;
            };

            let (title, hiragana, mut romaji) = query.single_mut();

            let sharing = get_sharing();
            let player = sharing
                .get(
                    &crate::word_typing::sharing::key_char_to_num(
                        c.to_uppercase().to_string().chars().next().unwrap(),
                    )
                    .unwrap(),
                )
                .unwrap();

            if (player == &Player::Right && kb_id == 0) || (player == &Player::Left && kb_id == 1) {
                println!("Invalid key: {} (by {})", key_num, kb_id);
                return;
            }

            let mut cur_r = romaji.0.clone();
            cur_r.push(c);

            if parse_hiragana_with_buf(&hiragana.0, &cur_r).is_some() {
                romaji.0 = cur_r;
            } else {
                println!("Invalid input: {}", c);
            }
        }
    }
}
