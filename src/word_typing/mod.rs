use crate::in_game::{Level, LevelResource, ProblemCurrentRomajiText, ProblemHiraganaText, ProblemTitleText, UsedWordIndexes};
use crate::AppState;
use bevy::app::App;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{
    Color, Commands, Component, In, IntoSystem, Query, Res, ResMut, Resource, Text, TextStyle,
    With, World,
};
use bevy::prelude::{State, System};
use bevy::text::TextSection;
use bevy::ui::Style;
use bevy::utils::default;
use bevy::window::CursorIcon::Text as TextCursor;
use std::str::Chars;
use std::vec::IntoIter;
use successive_romaji::{parse_hiragana_with_buf, ParseResult};

use crate::word_typing::choice_word::choice_word;
use crate::word_typing::sharing::{get_sharing, key_char_to_num, Player};
pub use sharing::key_num_to_char;

mod choice_word;
pub(crate) mod sharing;
mod wordbook;

#[derive(Resource)]
pub struct Title(pub String);

#[derive(Resource)]
pub struct Hiragana(pub String);

#[derive(Resource)]
pub struct CurrentRomaji(pub String);

pub fn set_new_problem(
    mut commands: Commands,
    mut level_resource: ResMut<LevelResource>,
    mut used_word_indexes_resource: ResMut<UsedWordIndexes>,
) {
    let word = choice_word(level_resource.0, &used_word_indexes_resource.0);

    let title = word.0;
    let hiragana = word.1;
    commands.insert_resource(Title(title.to_string()));
    commands.insert_resource(Hiragana(hiragana.to_string()));
    commands.insert_resource(CurrentRomaji("".to_string()));
}

fn update_title_ui(In(title): In<String>, mut query: Query<&mut Text, With<ProblemTitleText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = title.to_string();
}

fn update_hiragana_ui(
    In(hiragana): In<String>,
    mut query: Query<&mut Text, With<ProblemHiraganaText>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = hiragana.to_string();
}

fn make_sec(c: char, is_confirmed: bool, is_easy: bool) -> TextSection {
    let sharing = get_sharing();
    let player = sharing
        .get(&key_char_to_num(c.to_uppercase().to_string().chars().next().unwrap()).unwrap())
        .unwrap();
    let color = if is_easy {match player {
        Player::Left => {
            if is_confirmed {
                Color::rgb(0.4, 0.0, 0.0)
            } else {
                Color::rgb(1.0, 0.6, 0.6)
            }
        }
        Player::Right => {
            if is_confirmed {
                Color::rgb(0.0, 0.0, 0.4)
            } else {
                Color::rgb(0.6, 0.6, 1.0)
            }
        }
    } } else {
        if is_confirmed {
                Color::rgb(0.4, 0.4, 0.4)
            } else {
                Color::rgb(0.8, 0.8, 0.8)
            }
    };

    TextSection {
        value: c.to_string(),
        style: TextStyle {
            color,
            font_size: 80.0,
            ..default()
        },
    }
}

fn update_romaji_ui(
    In((hiragana, romaji)): In<(String, String)>,
    mut query: Query<&mut Text, With<ProblemCurrentRomajiText>>,
    level: Res<LevelResource>,
) {
    let mut text = query.single_mut();

    let confirmed_romajis: Vec<char>;
    let rest_romajis: Vec<char>;

    match parse_hiragana_with_buf(&hiragana, &romaji).unwrap() {
        ParseResult::Writing(confirmed, writing, tail) => {
            confirmed_romajis = confirmed
                .iter()
                .map(|(_, r)| r.chars())
                .flatten()
                .chain(writing.cur_buf_string.chars())
                .collect();

            rest_romajis = writing.romaji[writing.cur_buf_string.len()..]
                .chars()
                .chain(tail.iter().map(|(_, r)| r.chars()).flatten())
                .collect();
        }
        ParseResult::Completed(parts) => {
            confirmed_romajis = parts.iter().map(|(_, r)| r.chars()).flatten().collect();
            rest_romajis = "".chars().collect();
        }
    }

    let mut secs = vec![];
    secs.extend(confirmed_romajis.iter().map(|c| make_sec(*c, true, level.0 == Level::Easy)));
    secs.extend(rest_romajis.iter().map(|c| make_sec(*c, false, level.0 == Level::Easy)));
    text.sections = secs;
}

pub fn update_problem_ui(world: &mut World) {
    let title = world.resource::<Title>();
    let hiragana = world.resource::<Hiragana>();
    let current_romaji = world.resource::<CurrentRomaji>();
    let (title, hiragana, current_romaji) = (
        title.0.clone(),
        hiragana.0.clone(),
        current_romaji.0.clone(),
    );

    world.run_system_once_with(title, update_title_ui);
    world.run_system_once_with(hiragana.clone(), update_hiragana_ui);
    world.run_system_once_with((hiragana, current_romaji), update_romaji_ui);
}
