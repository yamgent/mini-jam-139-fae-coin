use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use iyes_progress::ProgressCounter;

use crate::app_state::{AppState, StateOwner};

pub struct LoadingUiPlugin;

impl Plugin for LoadingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup_loading_ui)
            .add_systems(
                Update,
                update_loading_ui.run_if(in_state(AppState::Loading)),
            );
    }
}

#[derive(Component)]
struct LoadingTextUi;

fn setup_loading_ui(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        },
        StateOwner(AppState::Loading),
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            StateOwner(AppState::Loading),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Loading: 0/0",
                    TextStyle {
                        font_size: 48.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                LoadingTextUi,
            ));
        });
}

fn update_loading_ui(
    progress: Option<Res<ProgressCounter>>,
    mut last_done: Local<u32>,
    mut last_total: Local<u32>,
    mut query: Query<&mut Text, With<LoadingTextUi>>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done || progress.total != *last_total {
            *last_done = progress.done;
            *last_total = progress.total;

            query.for_each_mut(|mut text| {
                text.sections[0].value = format!("Loading: {}/{}", progress.done, progress.total);
            });
        }
    }
}
