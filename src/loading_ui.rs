use bevy::prelude::*;

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

fn setup_loading_ui(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), StateOwner(AppState::Loading)));

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
            parent.spawn(TextBundle::from_section(
                "Loading: 0/0",
                TextStyle {
                    font_size: 32.0,
                    color: Color::GREEN,
                    ..Default::default()
                },
            ));
        });
}

fn update_loading_ui(mut next_state: ResMut<NextState<AppState>>) {
    // TODO: Actual code
    next_state.set(AppState::MainMenu);
}
