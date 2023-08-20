use bevy::prelude::*;

use crate::app_state::{AppState, StateOwner};

pub struct CoinLaunchUiPlugin;

#[derive(Resource)]
pub struct CoinLaunchSpeedPercentage(pub f32);

impl Plugin for CoinLaunchUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CoinLaunchSpeedPercentage(0.0))
            .add_systems(
                OnEnter(AppState::CoinLaunch),
                (setup_coin_launch_ui, reset_launch_speed),
            )
            .add_systems(
                Update,
                (update_speed, update_speed_ui, launch_coin).run_if(in_state(AppState::CoinLaunch)),
            );
    }
}

fn reset_launch_speed(mut speed: ResMut<CoinLaunchSpeedPercentage>) {
    speed.0 = 0.0;
}

#[derive(Component)]
struct SpeedIndicatorUi;

const SPEED_BAR_HEIGHT: f32 = 200.0;
const SPEED_BAR_TOP: f32 = 32.0;

const SPEED_INDICATOR_HEIGHT: f32 = 16.0;

fn setup_coin_launch_ui(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), StateOwner(AppState::CoinLaunch)));

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Px(SPEED_BAR_HEIGHT),
                position_type: PositionType::Absolute,
                top: Val::Px(SPEED_BAR_TOP),
                right: Val::Px(100.0),
                ..Default::default()
            },
            background_color: Color::WHITE.into(),
            ..Default::default()
        },
        StateOwner(AppState::CoinLaunch),
    ));

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(16.0),
                height: Val::Px(SPEED_INDICATOR_HEIGHT),
                position_type: PositionType::Absolute,
                top: Val::Px(SPEED_BAR_TOP + SPEED_BAR_HEIGHT - SPEED_INDICATOR_HEIGHT * 0.5),
                right: Val::Px(80.0),
                ..Default::default()
            },
            background_color: Color::WHITE.into(),
            ..Default::default()
        },
        SpeedIndicatorUi,
        StateOwner(AppState::CoinLaunch),
    ));
}

fn update_speed(time: Res<Time>, mut speed: ResMut<CoinLaunchSpeedPercentage>) {
    speed.0 += time.delta_seconds();
    if speed.0 > 1.0 {
        speed.0 = speed.0 - speed.0.floor();
    }
}

fn update_speed_ui(
    speed: Res<CoinLaunchSpeedPercentage>,
    mut query: Query<&mut Style, With<SpeedIndicatorUi>>,
) {
    query.for_each_mut(|mut style| {
        style.top = Val::Px(
            SPEED_BAR_TOP + (1.0 - speed.0) * SPEED_BAR_HEIGHT - SPEED_INDICATOR_HEIGHT * 0.5,
        );
    });
}

fn launch_coin(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    next_state.set(AppState::Ingame);
}
