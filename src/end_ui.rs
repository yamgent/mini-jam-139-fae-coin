use bevy::prelude::*;

use crate::{
    app_state::{AppState, StateOwner},
    scores::Scores,
};

pub struct EndUiPlugin;

impl Plugin for EndUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::End), setup_end_ui);
    }
}

fn setup_end_ui(mut commands: Commands, scores: Res<Scores>) {
    commands.spawn((Camera2dBundle::default(), StateOwner(AppState::End)));

    commands.spawn((
        TextBundle::from_section(
            "Game Over",
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        ),
        StateOwner(AppState::End),
    ));

    commands.spawn((
        TextBundle::from_section(
            format!("Final Score: {}m", scores.end_score),
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(32.0),
            left: Val::Px(0.0),
            ..Default::default()
        }),
        StateOwner(AppState::End),
    ));

    commands.spawn((
        TextBundle::from_section(
            format!("Your Best: {}m", scores.best_score),
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(128.0),
            left: Val::Px(0.0),
            ..Default::default()
        }),
        StateOwner(AppState::End),
    ));
}
