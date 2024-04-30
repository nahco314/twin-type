mod force_end;
mod in_game;
mod main_menu;
mod result_screen;
mod show_fps;
mod status_board;
mod word_typing;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tpg_kb_util::listen_events;
use tpg_kb_util::KBEvent;

use crate::force_end::{create_force_end_button, force_end_button_system, remove_force_end_button};
use crate::in_game::{
    check_time, create_mistype_effect, exit_in_game, setup_listen_kb, start_ingame,
    update_listen_kb, update_mistype_effect, History, Level, LevelResource, UsedWordIndexes,
};
use crate::main_menu::{exit_main_menu, main_menu_button_system, setup_main_menu, MainMenuUI};
use crate::result_screen::{exit_result_screen, result_screen_button_system, start_result_screen};
use crate::show_fps::{setup_fps_text, update_fps_text};
use crate::status_board::{create_status_board, update_remaining_time, update_score};
use crate::word_typing::{set_new_problem, update_problem_ui};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    ResultScreen,
}

fn setup_base(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup_base)
        .add_systems(Startup, setup_listen_kb)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), exit_main_menu)
        .add_systems(
            OnEnter(AppState::InGame),
            (start_ingame, apply_deferred, set_new_problem).chain(),
        )
        .add_systems(OnEnter(AppState::InGame), create_status_board)
        .add_systems(OnEnter(AppState::InGame), create_force_end_button)
        .add_systems(OnExit(AppState::InGame), exit_in_game)
        .add_systems(OnExit(AppState::InGame), remove_force_end_button)
        .add_systems(OnEnter(AppState::ResultScreen), start_result_screen)
        .add_systems(OnExit(AppState::ResultScreen), exit_result_screen)
        .add_systems(
            Update,
            main_menu_button_system.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(Update, update_listen_kb.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_problem_ui.run_if(in_state(AppState::InGame)))
        .add_systems(
            Update,
            create_mistype_effect.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            update_mistype_effect.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            update_remaining_time.run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, check_time.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_score.run_if(in_state(AppState::InGame)))
        .add_systems(
            Update,
            force_end_button_system.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            result_screen_button_system.run_if(in_state(AppState::ResultScreen)),
        );

    if cfg!(feature = "show_fps") {
        println!("show_fps feature is enabled");
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps_text)
            .add_systems(Update, update_fps_text);
    }

    app.run();
}
