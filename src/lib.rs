mod coin;

use bevy::prelude::*;
use coin::CoinPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CoinPlugin)
        .add_systems(Startup, setup)
        .run();
}
