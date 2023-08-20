use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{
    app_state::{AppState, StateOwner},
    coin::Coin,
    coin_launch_ui::SKY_COLOR,
    math::lerp,
};

pub struct CoinCameraPlugin;

impl Plugin for CoinCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Ingame), setup_coin_camera)
            .add_systems(
                Update,
                (pan_camera_with_coin_speed, set_sky_color).run_if(in_state(AppState::Ingame)),
            );
    }
}

#[derive(Component)]
struct CoinCamera;

fn setup_coin_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        CoinCamera,
        StateOwner(AppState::Ingame),
    ));
}

pub const COIN_SCREEN_BOUNDS_X: f32 = 200.0;
pub const COIN_SCREEN_BOUNDS_Y: f32 = 350.0;

const CAMERA_PAN_Y_DIST: f32 = COIN_SCREEN_BOUNDS_Y - 200.0;

fn pan_camera_with_coin_speed(
    time: Res<Time>,
    coin_query: Query<&Coin>,
    mut camera_query: Query<&mut Transform, With<CoinCamera>>,
) {
    let coin = coin_query.single();
    let mut camera_transform = camera_query.single_mut();

    let target_y = CAMERA_PAN_Y_DIST - coin.speed.min(0.0);

    let dist = target_y - camera_transform.translation.y;

    if dist.abs() < 0.01 {
        return;
    }

    camera_transform.translation.y += dist * time.delta_seconds();
}

fn set_sky_color(
    coin_query: Query<&Coin>,
    mut camera_query: Query<&mut Camera2d, With<CoinCamera>>,
) {
    let coin = coin_query.single();
    let mut camera = camera_query.single_mut();
    let color_fade = (coin.altitude / 20000.0).min(1.0);
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(
        lerp(SKY_COLOR.r(), 0.0, color_fade),
        lerp(SKY_COLOR.g(), 0.0, color_fade),
        lerp(SKY_COLOR.b(), 0.0, color_fade),
    ));
}
