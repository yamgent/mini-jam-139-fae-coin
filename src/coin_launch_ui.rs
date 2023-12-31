use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{
    app_state::{AppState, StateOwner},
    game_assets::{FontAssets, TextureAssets},
};

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

pub const SKY_COLOR: Color = Color::rgb(145.0 / 255.0, 142.0 / 255.0, 229.0 / 255.0);

fn setup_coin_launch_ui(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    font_assets: Res<FontAssets>,
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(SKY_COLOR),
                ..Default::default()
            },
            ..Default::default()
        },
        StateOwner(AppState::CoinLaunch),
    ));

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
        UiImage::new(texture_assets.texture_launch_bar.clone()),
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
        UiImage::new(texture_assets.texture_launch_arrow.clone()),
        StateOwner(AppState::CoinLaunch),
    ));

    commands.spawn((
        TextBundle::from_section(
            "[SPACE]: GO!",
            TextStyle {
                font: font_assets.font_fira.clone(),
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(SPEED_BAR_TOP + SPEED_BAR_HEIGHT + 16.0),
            right: Val::Px(32.0),
            ..Default::default()
        }),
        StateOwner(AppState::CoinLaunch),
    ));

    commands.spawn((
        SpriteBundle {
            texture: texture_assets.texture_launcher.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -240.0, 2.0)),
            ..Default::default()
        },
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
