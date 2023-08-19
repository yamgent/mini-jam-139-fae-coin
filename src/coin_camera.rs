use bevy::prelude::*;

use crate::coin::Coin;

pub struct CoinCameraPlugin;

impl Plugin for CoinCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_coin_camera)
            .add_systems(Update, pan_camera_with_coin_speed);
    }
}

#[derive(Component)]
struct CoinCamera;

fn setup_coin_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CoinCamera));
}

// TODO: Related to screen bounds?
const CAMERA_PAN_Y_DIST: f32 = 150.0;

fn pan_camera_with_coin_speed(
    time: Res<Time>,
    coin_query: Query<&Coin>,
    mut camera_query: Query<&mut Transform, With<CoinCamera>>,
) {
    let coin = coin_query.single();
    let mut camera_transform = camera_query.single_mut();

    let target_y = if coin.speed > 20.0 {
        CAMERA_PAN_Y_DIST
    } else if coin.speed > -20.0 {
        0.0
    } else {
        -CAMERA_PAN_Y_DIST
    };

    let dist = target_y - camera_transform.translation.y;

    if dist.abs() < 0.01 {
        return;
    }

    camera_transform.translation.y += dist * time.delta_seconds();
}
