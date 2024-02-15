use crate::in_game::{ProblemCurrentRomajiText, ProblemHiraganaText, ProblemTitleText};
use crate::AppState;
use bevy::app::App;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{
    Color, Commands, Component, In, IntoSystem, Query, Res, Text, TextStyle, With, World,
};
use bevy::prelude::{State, System};
use bevy::text::TextSection;
use bevy::ui::Style;
use bevy::utils::default;
use bevy::window::CursorIcon::Text as TextCursor;
use std::str::Chars;
use std::vec::IntoIter;
use successive_romaji::{parse_hiragana_with_buf, ParseResult};

use crate::word_typing::sharing::{get_sharing, key_char_to_num, Player};
pub use sharing::key_num_to_char;

pub(crate) mod sharing;

#[derive(Component)]
pub struct Title(pub String);

#[derive(Component)]
pub struct Hiragana(pub String);

#[derive(Component)]
pub struct CurrentRomaji(pub String);

pub fn set_new_problem(mut commands: Commands) {
    let title = "ずっと真夜中でいいのに。";
    let hiragana = "ずっとまよなかでいいのに";
    commands.spawn((
        Title(title.to_string()),
        Hiragana(hiragana.to_string()),
        CurrentRomaji("".to_string()),
    ));
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

fn make_sec(c: char, is_confirmed: bool) -> TextSection {
    let sharing = get_sharing();
    let player = sharing
        .get(&key_char_to_num(c.to_uppercase().to_string().chars().next().unwrap()).unwrap())
        .unwrap();
    let color = match player {
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
    secs.extend(confirmed_romajis.iter().map(|c| make_sec(*c, true)));
    secs.extend(rest_romajis.iter().map(|c| make_sec(*c, false)));
    text.sections = secs;
}

pub fn update_problem_ui(world: &mut World) {
    let mut query = world.query::<(&Title, &Hiragana, &CurrentRomaji)>();
    let (title, hiragana, current_romaji) = query.single(world);
    let (title, hiragana, current_romaji) = (
        title.0.clone(),
        hiragana.0.clone(),
        current_romaji.0.clone(),
    );

    world.run_system_once_with(title, update_title_ui);
    world.run_system_once_with(hiragana.clone(), update_hiragana_ui);
    world.run_system_once_with((hiragana, current_romaji), update_romaji_ui);
}
