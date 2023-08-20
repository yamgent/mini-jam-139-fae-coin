use bevy::prelude::*;

use crate::app_state::{AppState, StateOwner};

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu_ui)
            .add_systems(Update, start_game.run_if(in_state(AppState::MainMenu)));
    }
}

fn setup_main_menu_ui(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), StateOwner(AppState::MainMenu)));

    commands.spawn((
        TextBundle::from_section(
            "Coin Flip",
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        ),
        StateOwner(AppState::MainMenu),
    ));

    commands.spawn((
        TextBundle::from_section(
            "Press SPACE to start",
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(256.0),
            left: Val::Px(0.0),
            ..Default::default()
        }),
        StateOwner(AppState::MainMenu),
    ));
}

fn start_game(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::CoinLaunch);
    }
}
