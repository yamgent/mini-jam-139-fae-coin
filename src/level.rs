use bevy::prelude::*;

use crate::cloud::InitCloud;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelMetadata::default())
            .add_systems(Update, spawn_clouds);
    }
}

#[derive(Resource)]
struct LevelMetadata {
    last_cloud_spawn_time: f32,
}

impl Default for LevelMetadata {
    fn default() -> Self {
        Self {
            last_cloud_spawn_time: 0.0,
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
        commands.spawn(InitCloud(Vec2::new(0.0, SPAWN_Y_POS)));
    }
}
