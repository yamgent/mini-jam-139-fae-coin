use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::ProgressPlugin;

use crate::app_state::AppState;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(AppState::Loading).continue_to(AppState::MainMenu))
            .add_loading_state(LoadingState::new(AppState::Loading))
            .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/coin.png")]
    pub texture_coin: Handle<Image>,
}
