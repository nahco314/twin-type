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
use std::time::Instant;
use tpg_kb_util::{listen_events, KBEvent, KBEventType};

use crate::in_game::EventType::{MisType, Success};
use crate::status_board::{GameTime, StartTime};
use crate::word_typing::sharing::{get_sharing, Player};
use crate::word_typing::{key_num_to_char, set_new_problem, CurrentRomaji, Hiragana, Title};
use crate::AppState::InGame;
use bevy::ecs::system::RunSystemOnce;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use successive_romaji::{parse_hiragana_with_buf, ParseResult};

pub struct ListenResource {
    thread: JoinHandle<()>,
    rx: Receiver<KBEvent>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Level {
    Easy,
    Medium,
    Hard,
    ExtraHard,
}

#[derive(Resource)]
pub struct LevelResource(pub Level);

#[derive(Resource)]
pub struct UsedWordIndexes(pub Vec<usize>);

#[derive(PartialEq, Clone)]
pub enum MistypeReason {
    Opposite,
    InvalidChar { key_num: u8 },
}

#[derive(PartialEq, Clone)]
pub enum EventType {
    Success { key_num: u8 },
    MisType { reason: MistypeReason },
}

#[derive(PartialEq, Clone)]
pub struct Event {
    pub is_left: bool,
    pub time: Instant,
    pub type_: EventType,
}

#[derive(Resource)]
pub struct History {
    pub list: Vec<Event>,
}

#[derive(Component)]
pub struct Something;

#[derive(Component)]
pub struct InGameUI;

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
        .insert(InGameUI)
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

    commands.insert_resource(StartTime(Instant::now()));
    commands.insert_resource(GameTime(60));

    commands.insert_resource(UsedWordIndexes(vec![]));
    commands.insert_resource(History { list: vec![] });
}

pub fn update_listen_kb(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lr: NonSend<ListenResource>,
    mut level_resource: ResMut<LevelResource>,
    mut used_word_indexes_resource: ResMut<UsedWordIndexes>,
    mut title: Res<Title>,
    mut hiragana: Res<Hiragana>,
    mut romaji: ResMut<CurrentRomaji>,
    mut history: ResMut<History>,
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

                history.list.push(Event {
                    is_left: kb_id == 0,
                    time: Instant::now(),
                    type_: MisType {
                        reason: MistypeReason::Opposite,
                    },
                });

                return;
            }

            let mut cur_r = romaji.0.clone();
            cur_r.push(c);

            match parse_hiragana_with_buf(&hiragana.0, &cur_r) {
                Some(ParseResult::Writing(..)) => {
                    romaji.0 = cur_r;

                    history.list.push(Event {
                        is_left: kb_id == 0,
                        time: Instant::now(),
                        type_: Success { key_num },
                    });
                }
                Some(ParseResult::Completed(..)) => {
                    set_new_problem(commands, level_resource, used_word_indexes_resource);

                    history.list.push(Event {
                        is_left: kb_id == 0,
                        time: Instant::now(),
                        type_: Success { key_num },
                    });
                }
                None => {
                    println!("Invalid input: {}", c);

                    history.list.push(Event {
                        is_left: kb_id == 0,
                        time: Instant::now(),
                        type_: MisType {
                            reason: MistypeReason::InvalidChar { key_num },
                        },
                    });
                }
            }
        }
    }
}

#[derive(Component)]
pub struct MisTypeEffect;

#[derive(Component)]
pub struct EventComponent(pub Event);

fn get_x(is_left: bool, window: &Window) -> f32 {
    let x_w = window.width();
    let num = x_w / 4.0;
    return if is_left { -num } else { num };
}

pub fn create_mistype_effect(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&MisTypeEffect, &EventComponent)>,
    history: ResMut<History>,
    window: Query<&Window>,
) {
    let now = Instant::now();

    for e in &history.list {
        if let MisType { .. } = e.type_ {
        } else {
            continue;
        }

        let mut exist = query.iter().any(|ent| *e == ent.1 .0);
        let elapsed = (now - e.time).as_secs_f32();

        if elapsed > 5.0 {
            exist = true;
        }

        if !exist {
            let x = get_x(e.is_left, window.single());

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "ミス！",
                        TextStyle {
                            font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform {
                        translation: Vec3 {
                            x,
                            y: 200.0,
                            z: 0.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                MisTypeEffect {},
                InGameUI {},
                EventComponent(e.clone()),
            ));
        }
    }
}

pub fn update_mistype_effect(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Transform, &mut Text, &EventComponent), With<(MisTypeEffect)>>,
    history: ResMut<History>,
    window: Query<&Window>,
) {
    let now = Instant::now();

    for (e, mut t, mut text, ec) in query.iter_mut() {
        let elapsed = (now - ec.0.time).as_secs_f32();

        if elapsed > 5.0 {
            commands.entity(e).despawn();
            continue;
        }

        let x = get_x(ec.0.is_left, window.single());

        t.translation.x = x;
        t.translation.y = 200.0 - elapsed * 50.0;

        &text.sections[0].style.color.set_a(1.0 - elapsed / 1.0);
    }
}

pub fn check_time(
    start_time: Res<StartTime>,
    game_time: Res<GameTime>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let elapsed = start_time.0.elapsed();

    let remaining_time = game_time.0 - elapsed.as_secs();

    if remaining_time == 0 {
        next_state.set(AppState::ResultScreen);
    }
}

pub fn exit_in_game(mut commands: Commands, query: Query<Entity, With<InGameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
