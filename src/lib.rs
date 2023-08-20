mod app_state;
mod boost_item;
mod cloud;
mod coin;
mod coin_camera;
mod end_ui;
mod fairy;
mod ingame_ui;
mod level;
mod physics;
mod scores;

use app_state::AppStatePlugin;
use bevy::prelude::*;
use boost_item::BoostItemPlugin;
use cloud::CloudPlugin;
use coin::CoinPlugin;
use coin_camera::CoinCameraPlugin;
use end_ui::EndUiPlugin;
use fairy::FairyPlugin;
use ingame_ui::IngameUiPlugin;
use level::LevelPlugin;
use physics::PhysicsPlugin;
use scores::ScoresPlugin;

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugin,
            AppStatePlugin,
            CoinPlugin,
            CoinCameraPlugin,
            CloudPlugin,
            LevelPlugin,
            IngameUiPlugin,
            BoostItemPlugin,
            FairyPlugin,
            EndUiPlugin,
            ScoresPlugin,
        ))
        .run();
}
