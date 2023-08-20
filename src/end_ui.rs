use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{
    app_state::{AppState, StateOwner},
    game_assets::FontAssets,
    scores::Scores,
};

pub struct EndUiPlugin;

impl Plugin for EndUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::End), setup_end_ui)
            .add_systems(Update, restart_game.run_if(in_state(AppState::End)));
    }
}

const BACKGROUND_COLOR: Color = Color::rgb(40.0 / 255.0, 40.0 / 255.0, 63.0 / 255.0);

fn setup_end_ui(mut commands: Commands, scores: Res<Scores>, font_assets: Res<FontAssets>) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
                ..Default::default()
            },
            ..Default::default()
        },
        StateOwner(AppState::End),
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            StateOwner(AppState::End),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Good Flight!",
                TextStyle {
                    font_size: 96.0,
                    color: Color::CYAN,
                    ..Default::default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(16.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("Final Score: {}m", scores.end_score),
                        TextStyle {
                            font: font_assets.font_fira.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(4.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("Your Best: {}m", scores.best_score),
                                TextStyle {
                                    font: font_assets.font_fira.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            ));

                            if scores.new_record {
                                parent.spawn(TextBundle::from_section(
                                    "(NEW BEST!)",
                                    TextStyle {
                                        font: font_assets.font_fira.clone(),
                                        font_size: 28.0,
                                        color: Color::YELLOW,
                                        ..Default::default()
                                    },
                                ));
                            }
                        });
                });

            parent.spawn(TextBundle::from_section(
                "Press [SPACE] to try again",
                TextStyle {
                    font: font_assets.font_fira.clone(),
                    font_size: 40.0,
                    color: Color::GREEN,
                    ..Default::default()
                },
            ));
        });
}

fn restart_game(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::CoinLaunch);
    }
}
