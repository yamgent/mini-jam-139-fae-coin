use bevy::prelude::*;

use crate::{
    app_state::{AppState, StateOwner},
    boost_item::InitBoostItem,
    cloud::InitCloud,
    coin::Coin,
    fairy::InitFairy,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelMetadata::default()).add_systems(
            Update,
            (spawn_clouds, spawn_boost, spawn_fairy).run_if(in_state(AppState::Ingame)),
        );
    }
}

#[derive(Resource)]
struct LevelMetadata {
    next_cloud_spawn_altitude: f32,
    next_boost_spawn_altitude: f32,
    next_fairy_spawn_altitude: f32,
}

impl Default for LevelMetadata {
    fn default() -> Self {
        Self {
            next_cloud_spawn_altitude: SPAWN_Y_POS * 2.0,
            next_boost_spawn_altitude: SPAWN_Y_POS * 3.0,
            next_fairy_spawn_altitude: SPAWN_Y_POS * 1.5,
        }
    }
}

// TODO: Related to screen bounds
const SPAWN_Y_POS: f32 = 800.0;

fn spawn_clouds(
    mut commands: Commands,
    mut level_metadata: ResMut<LevelMetadata>,
    coin_query: Query<&Coin>,
) {
    let coin = coin_query.single();

    if level_metadata.next_cloud_spawn_altitude < coin.altitude {
        commands.spawn((
            InitCloud(Vec2::new(120.0, SPAWN_Y_POS)),
            StateOwner(AppState::Ingame),
        ));

        level_metadata.next_cloud_spawn_altitude += SPAWN_Y_POS * 2.0;
    }
}

fn spawn_boost(
    mut commands: Commands,
    mut level_metadata: ResMut<LevelMetadata>,
    coin_query: Query<&Coin>,
) {
    let coin = coin_query.single();

    // TODO: Better level design
    if level_metadata.next_boost_spawn_altitude < coin.altitude {
        commands.spawn((
            InitBoostItem(Vec2::new(-60.0, SPAWN_Y_POS)),
            StateOwner(AppState::Ingame),
        ));
        level_metadata.next_boost_spawn_altitude += SPAWN_Y_POS * 3.0;
    }
}

fn spawn_fairy(
    mut commands: Commands,
    mut level_metadata: ResMut<LevelMetadata>,
    coin_query: Query<&Coin>,
) {
    let coin = coin_query.single();

    // TODO: Better level design
    if level_metadata.next_fairy_spawn_altitude < coin.altitude {
        commands.spawn((
            InitFairy(Vec2::new(0.0, SPAWN_Y_POS)),
            StateOwner(AppState::Ingame),
        ));
        level_metadata.next_fairy_spawn_altitude += SPAWN_Y_POS * 3.0;
    }
}
