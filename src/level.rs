use bevy::prelude::*;

use crate::{
    app_state::{AppState, StateOwner},
    boost_item::InitBoostItem,
    cloud::InitCloud,
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
    last_cloud_spawn_time: f32,
    last_boost_spawn_time: f32,
    last_fairy_spawn_time: f32,
}

impl Default for LevelMetadata {
    fn default() -> Self {
        Self {
            last_cloud_spawn_time: 0.0,
            last_boost_spawn_time: 0.0,
            last_fairy_spawn_time: 0.0,
        }
    }
}

// TODO: Related to screen bounds
const SPAWN_Y_POS: f32 = 800.0;

fn spawn_clouds(
    time: Res<Time>,
    mut level_metadata: ResMut<LevelMetadata>,
    mut commands: Commands,
) {
    // TODO: Better level design
    if time.elapsed_seconds() - level_metadata.last_cloud_spawn_time > 1.0 {
        level_metadata.last_cloud_spawn_time = time.elapsed_seconds();
        commands.spawn((
            InitCloud(Vec2::new(120.0, SPAWN_Y_POS)),
            StateOwner(AppState::Ingame),
        ));
    }
}

fn spawn_boost(time: Res<Time>, mut level_metadata: ResMut<LevelMetadata>, mut commands: Commands) {
    // TODO: Better level design
    if time.elapsed_seconds() - level_metadata.last_boost_spawn_time > 1.0 {
        level_metadata.last_boost_spawn_time = time.elapsed_seconds();
        commands.spawn((
            InitBoostItem(Vec2::new(-60.0, SPAWN_Y_POS)),
            StateOwner(AppState::Ingame),
        ));
    }
}

fn spawn_fairy(time: Res<Time>, mut level_metadata: ResMut<LevelMetadata>, mut commands: Commands) {
    // TODO: Better level design
    if time.elapsed_seconds() - level_metadata.last_fairy_spawn_time > 1.0 {
        level_metadata.last_fairy_spawn_time = time.elapsed_seconds();
        commands.spawn((
            InitFairy(Vec2::new(0.0, SPAWN_Y_POS)),
            StateOwner(AppState::Ingame),
        ));
    }
}
