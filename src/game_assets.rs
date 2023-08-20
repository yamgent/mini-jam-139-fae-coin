use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::ProgressPlugin;

use crate::app_state::AppState;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(AppState::Loading).continue_to(AppState::MainMenu))
            .add_loading_state(LoadingState::new(AppState::Loading))
            .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading)
            .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Regular.ttf")]
    pub font_fira: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/coin.png")]
    pub texture_coin: Handle<Image>,
    #[asset(path = "textures/clouds.png")]
    pub texture_clouds: Handle<Image>,
    #[asset(path = "textures/boost.png")]
    pub texture_boost: Handle<Image>,
    #[asset(path = "textures/fairy.png")]
    pub texture_fairy: Handle<Image>,
    #[asset(path = "textures/glow.png")]
    pub texture_glow: Handle<Image>,
    #[asset(path = "textures/launcher.png")]
    pub texture_launcher: Handle<Image>,

    #[asset(path = "textures/launch_bar.png")]
    pub texture_launch_bar: Handle<Image>,
    #[asset(path = "textures/launch_arrow.png")]
    pub texture_launch_arrow: Handle<Image>,
}
