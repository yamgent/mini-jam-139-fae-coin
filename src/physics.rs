use bevy::prelude::*;

use crate::{app_state::AppState, coin::Coin};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_y_pos_relative_to_coin, despawn_out_of_bounds_things)
                .run_if(in_state(AppState::Ingame)),
        );
    }
}

#[derive(Component)]
pub struct RelativeCoinY;

fn update_y_pos_relative_to_coin(
    time: Res<Time>,
    coin_query: Query<&Coin>,
    mut query: Query<&mut Transform, With<RelativeCoinY>>,
) {
    let coin = match coin_query.get_single() {
        Ok(coin) => coin,
        Err(_) => return,
    };

    let adjustments = coin.speed * time.delta_seconds();

    query.for_each_mut(|mut transform| {
        transform.translation.y -= adjustments;
    });
}

// TODO: Related to screen bounds
const OUT_OF_BOUNDS_Y: f32 = -10000.0;

fn despawn_out_of_bounds_things(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<RelativeCoinY>>,
) {
    query.for_each(|(transform, entity)| {
        if transform.translation.y < OUT_OF_BOUNDS_Y {
            commands.get_entity(entity).unwrap().despawn();
        }
    });
}
