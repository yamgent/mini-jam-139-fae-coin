use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    app_state::{AppState, StateOwner},
    boost_item::InitBoostItem,
    cloud::InitCloud,
    coin::Coin,
    coin_camera::{COIN_SCREEN_BOUNDS_X, COIN_SCREEN_BOUNDS_Y},
    fairy::InitFairy,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelMetadata::default())
            .add_systems(OnEnter(AppState::Ingame), reset_level_metadata)
            .add_systems(
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
            next_fairy_spawn_altitude: SPAWN_Y_POS * 0.5,
            next_cloud_spawn_altitude: SPAWN_Y_POS * 1.0,
            next_boost_spawn_altitude: SPAWN_Y_POS * 1.5,
        }
    }
}

fn reset_level_metadata(mut level_metadata: ResMut<LevelMetadata>) {
    *level_metadata = Default::default();
}

const SPAWN_Y_POS: f32 = COIN_SCREEN_BOUNDS_Y * 2.0;

fn lerp(a: f32, b: f32, val: f32) -> f32 {
    a + (b - a) * val
}

fn lvl(alt: f32, values: [f32; 5]) -> f32 {
    const LEVELS: [f32; 4] = [250.0, 500.0, 1000.0, 2000.0];

    *LEVELS
        .iter()
        .zip(values.iter())
        .find(|(l, _)| alt < **l * 10.0)
        .unwrap_or((&0.0, &values.iter().rev().next().unwrap()))
        .1
}

fn spawn_clouds(
    mut commands: Commands,
    mut level_metadata: ResMut<LevelMetadata>,
    coin_query: Query<&Coin>,
) {
    let coin = coin_query.single();
    let alt = coin.altitude;

    if level_metadata.next_cloud_spawn_altitude < alt {
        let mut rng = rand::thread_rng();

        let low = lvl(alt, [1.0, 2.0, 4.0, 5.0, 6.0]) as i32;
        let high = lvl(alt, [3.0, 6.0, 10.0, 15.0, 20.0]) as i32;

        let num_clouds = rng.gen_range(low..=high);

        (0..num_clouds).into_iter().for_each(|_| {
            commands.spawn((
                InitCloud(Vec2::new(
                    lerp(-COIN_SCREEN_BOUNDS_X, COIN_SCREEN_BOUNDS_X, rng.gen()),
                    lerp(SPAWN_Y_POS, SPAWN_Y_POS * 3.0, rng.gen()),
                )),
                StateOwner(AppState::Ingame),
            ));
        });

        level_metadata.next_cloud_spawn_altitude +=
            COIN_SCREEN_BOUNDS_Y * (2.0 + rng.gen::<f32>() * lvl(alt, [3.0, 3.0, 2.0, 1.0, 0.5]));
    }
}

fn spawn_boost(
    mut commands: Commands,
    mut level_metadata: ResMut<LevelMetadata>,
    coin_query: Query<&Coin>,
) {
    let coin = coin_query.single();
    let alt = coin.altitude;

    if level_metadata.next_boost_spawn_altitude < alt {
        let mut rng = rand::thread_rng();

        let low = 1;
        let high = lvl(alt, [4.0, 4.0, 3.0, 2.0, 1.0]) as i32;

        let num_boosts = rng.gen_range(low..=high);

        (0..num_boosts).into_iter().for_each(|_| {
            commands.spawn((
                InitBoostItem(Vec2::new(
                    lerp(-COIN_SCREEN_BOUNDS_X, COIN_SCREEN_BOUNDS_X, rng.gen()),
                    lerp(SPAWN_Y_POS, SPAWN_Y_POS * 3.0, rng.gen()),
                )),
                StateOwner(AppState::Ingame),
            ));
        });
        level_metadata.next_boost_spawn_altitude += SPAWN_Y_POS * 1.0;
    }
}

fn spawn_fairy(
    mut commands: Commands,
    mut level_metadata: ResMut<LevelMetadata>,
    coin_query: Query<&Coin>,
) {
    let coin = coin_query.single();
    let alt = coin.altitude;

    // TODO: Better level design
    if level_metadata.next_fairy_spawn_altitude < alt {
        let mut rng = rand::thread_rng();

        let low = lvl(alt, [1.0, 1.0, 1.0, 0.0, 0.0]) as i32;
        let high = lvl(alt, [3.0, 3.0, 2.0, 2.0, 1.0]) as i32;

        let num_fairy = rng.gen_range(low..=high);

        if num_fairy > 0 {
            (0..num_fairy).into_iter().for_each(|_| {
                commands.spawn((
                    InitFairy(Vec2::new(
                        lerp(-COIN_SCREEN_BOUNDS_X, COIN_SCREEN_BOUNDS_X, rng.gen()),
                        lerp(SPAWN_Y_POS, SPAWN_Y_POS * 3.0, rng.gen()),
                    )),
                    StateOwner(AppState::Ingame),
                ));
            });
        }

        level_metadata.next_fairy_spawn_altitude += SPAWN_Y_POS * 1.0;
    }
}
