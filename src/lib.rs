mod coin;
mod coin_camera;

use bevy::prelude::*;
use coin::CoinPlugin;
use coin_camera::CoinCameraPlugin;

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, CoinPlugin, CoinCameraPlugin))
        .run();
}
