use crate::main_menu::MainMenuUI;
use crate::result_screen::ResultScreenUI;
use crate::AppState;
use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::prelude::{
    default, AlignItems, BackgroundColor, BorderColor, Button, ButtonBundle, Changed, Color,
    Commands, Component, DespawnRecursiveExt, Entity, FlexDirection, Interaction, JustifyContent,
    NextState, NodeBundle, PositionType, Query, Res, ResMut, Style, TextAlignment, TextBundle,
    TextStyle, UiRect, Val, With,
};

#[derive(Component)]
pub struct ForceEndButton;

pub fn create_force_end_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                top: Val::Percent(5.0),
                left: Val::Percent(95.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "x",
                TextStyle {
                    font: asset_server.load("fonts/ShinRetroMaruGothic-R.ttf"),
                    font_size: 72.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(ForceEndButton);
}

pub fn force_end_button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<ForceEndButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                next_state.set(AppState::ResultScreen);
            }
            _ => {}
        }
    }
}

pub fn remove_force_end_button(mut commands: Commands, query: Query<Entity, With<ForceEndButton>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
