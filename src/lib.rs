mod boost_item;
mod cloud;
mod coin;
mod coin_camera;
mod fairy;
mod ingame_ui;
mod level;
mod physics;

use bevy::prelude::*;
use boost_item::BoostItemPlugin;
use cloud::CloudPlugin;
use coin::CoinPlugin;
use coin_camera::CoinCameraPlugin;
use fairy::FairyPlugin;
use ingame_ui::IngameUiPlugin;
use level::LevelPlugin;
use physics::PhysicsPlugin;

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugin,
            CoinPlugin,
            CoinCameraPlugin,
            CloudPlugin,
            LevelPlugin,
            IngameUiPlugin,
            BoostItemPlugin,
            FairyPlugin,
        ))
        .run();
}
