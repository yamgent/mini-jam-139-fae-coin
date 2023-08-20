mod anim;
mod app_state;
mod base;
mod boost_item;
mod cloud;
mod coin;
mod coin_camera;
mod coin_launch_ui;
mod end_ui;
mod fairy;
mod game_assets;
mod ingame_ui;
mod level;
mod loading_ui;
mod main_menu_ui;
mod math;
mod physics;
mod scores;

use anim::AnimPlugin;
use app_state::AppStatePlugin;
use base::CorePlugin;
use bevy::prelude::*;
use boost_item::BoostItemPlugin;
use cloud::CloudPlugin;
use coin::CoinPlugin;
use coin_camera::CoinCameraPlugin;
use coin_launch_ui::CoinLaunchUiPlugin;
use end_ui::EndUiPlugin;
use fairy::FairyPlugin;
use game_assets::GameAssetsPlugin;
use ingame_ui::IngameUiPlugin;
use level::LevelPlugin;
use loading_ui::LoadingUiPlugin;
use main_menu_ui::MainMenuUiPlugin;
use physics::PhysicsPlugin;
use scores::ScoresPlugin;

pub fn run() {
    App::new()
        .add_plugins((
            (
                CorePlugin,
                AppStatePlugin,
                LoadingUiPlugin,
                GameAssetsPlugin,
                MainMenuUiPlugin,
            ),
            (
                CoinLaunchUiPlugin,
                AnimPlugin,
                PhysicsPlugin,
                CoinPlugin,
                CoinCameraPlugin,
                CloudPlugin,
                LevelPlugin,
                IngameUiPlugin,
                BoostItemPlugin,
                FairyPlugin,
                EndUiPlugin,
                ScoresPlugin,
            ),
        ))
        .run();
}
