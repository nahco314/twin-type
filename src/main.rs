mod in_game;
mod main_menu;
mod word_typing;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tpg_kb_util::listen_events;
use tpg_kb_util::KBEvent;

use crate::in_game::{setup_listen_kb, start_ingame, update_listen_kb};
use crate::main_menu::{button_system, exit_main_menu, setup_main_menu, MainMenuUI};
use crate::word_typing::update_problem_ui;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins))
        .add_state::<AppState>()
        .add_systems(Startup, setup_main_menu)
        .add_systems(Startup, setup_listen_kb)
        .add_systems(OnExit(AppState::MainMenu), exit_main_menu)
        .add_systems(OnEnter(AppState::InGame), start_ingame)
        .add_systems(Update, button_system)
        .add_systems(Update, update_listen_kb.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_problem_ui.run_if(in_state(AppState::InGame)))
        .run();
}
