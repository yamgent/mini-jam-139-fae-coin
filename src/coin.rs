use bevy::prelude::*;

use crate::{
    app_state::{AppState, StateOwner},
    coin_camera::COIN_SCREEN_BOUNDS_X,
    coin_launch_ui::CoinLaunchSpeedPercentage,
    game_assets::TextureAssets,
    physics::RelativeCoinY,
    scores::Scores,
};

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Ingame), (setup_coin, setup_launcher))
            .add_systems(
                Update,
                (
                    handle_coin_gravity,
                    do_coin_flip_animation,
                    handle_coin_adjustments,
                    handle_coin_use_boost,
                    calculate_altitude,
                    check_game_over,
                )
                    .run_if(in_state(AppState::Ingame)),
            );
    }
}

#[derive(Component)]
pub struct Coin {
    pub speed: f32,
    pub additional_boosts: i32,
    pub altitude: f32,
    pub highest_altitude_recorded: f32,
}

impl Default for Coin {
    fn default() -> Self {
        Self {
            speed: 0.0,
            additional_boosts: 3,
            altitude: 0.0,
            highest_altitude_recorded: 0.0,
        }
    }
}

impl Coin {
    pub fn get_bounds(transform: &Transform) -> Rect {
        Rect::from_center_size(
            Vec2::new(transform.translation.x, transform.translation.y),
            COIN_FULL_SIZE,
        )
    }
}

#[derive(Component)]
pub struct CoinAnimation {
    orientation: f32,
    direction: f32,
}

impl Default for CoinAnimation {
    fn default() -> Self {
        Self {
            orientation: 1.0,
            direction: -1.0,
        }
    }
}

const GRAVITY: f32 = 98.0;
const COIN_FULL_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const COIN_MIN_START_SPEED: f32 = 400.0;
const COIN_MAX_START_SPEED: f32 = 1400.0;

fn setup_coin(
    mut commands: Commands,
    launch_speed_percentage: Res<CoinLaunchSpeedPercentage>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: texture_assets.texture_coin.clone(),
            ..Default::default()
        },
        Coin {
            speed: COIN_MIN_START_SPEED
                + (COIN_MAX_START_SPEED - COIN_MIN_START_SPEED) * launch_speed_percentage.0,
            additional_boosts: 3,
            altitude: 0.0,
            highest_altitude_recorded: 0.0,
        },
        CoinAnimation::default(),
        StateOwner(AppState::Ingame),
    ));
}

fn setup_launcher(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: texture_assets.texture_launcher.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -240.0, 2.0)),
            ..Default::default()
        },
        RelativeCoinY,
        StateOwner(AppState::Ingame),
    ));
}

fn handle_coin_gravity(time: Res<Time>, mut query: Query<&mut Coin>) {
    query.for_each_mut(|mut coin| {
        coin.speed += -GRAVITY * time.delta_seconds();
    });
}

const COIN_ANIM_MAX_COIN_SPEED_CAP: f32 = 2400.0;
const COIN_ANIM_MIN_COIN_SPEED_CAP: f32 = 100.0;
const COIN_ANIM_MAX_SPIN_SPEED: f32 = 15.0;

fn do_coin_flip_animation(
    time: Res<Time>,
    mut query: Query<(&mut CoinAnimation, &mut Sprite, &Coin)>,
) {
    query.for_each_mut(|(mut anim, mut sprite, coin)| {
        if coin.speed.abs() > 0.01 {
            let spin_speed = COIN_ANIM_MAX_SPIN_SPEED
                * (coin
                    .speed
                    .abs()
                    .min(COIN_ANIM_MAX_COIN_SPEED_CAP)
                    .max(COIN_ANIM_MIN_COIN_SPEED_CAP)
                    / COIN_ANIM_MAX_COIN_SPEED_CAP)
                    .powf(0.5);
            anim.orientation += anim.direction * spin_speed * time.delta_seconds();

            if anim.direction < 0.0 {
                if anim.orientation < -1.0 {
                    anim.orientation += -1.0 - anim.orientation;
                    anim.direction = 1.0;
                }
            } else {
                if anim.orientation > 1.0 {
                    anim.orientation -= anim.orientation - 1.0;
                    anim.direction = -1.0;
                }
            }

            sprite.custom_size = Some(Vec2::new(
                COIN_FULL_SIZE.x,
                COIN_FULL_SIZE.y * anim.orientation,
            ));
        }
    });
}

const COIN_ADJUSTMENT_X_SPEED: f32 = 200.0;
const COIN_ADJUSTMENT_Y_SPEED_PENALTY: f32 = 90.0;

fn handle_coin_adjustments(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Coin)>,
) {
    let left_pressed = keyboard.pressed(KeyCode::Left);
    let right_pressed = keyboard.pressed(KeyCode::Right);

    if left_pressed == right_pressed {
        return;
    }

    let direction = if left_pressed { -1.0 } else { 1.0 };

    query.for_each_mut(|(mut transform, mut coin)| {
        transform.translation.x += direction * COIN_ADJUSTMENT_X_SPEED * time.delta_seconds();
        transform.translation.x = transform
            .translation
            .x
            .max(-COIN_SCREEN_BOUNDS_X)
            .min(COIN_SCREEN_BOUNDS_X);
        coin.speed -= COIN_ADJUSTMENT_Y_SPEED_PENALTY * time.delta_seconds();
    });
}

const COIN_MANUAL_BOOST_SPEED_GAIN: f32 = 200.0;

fn handle_coin_use_boost(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Coin>) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    query.for_each_mut(|mut coin| {
        if coin.additional_boosts <= 0 {
            return;
        }

        coin.additional_boosts -= 1;
        coin.speed += COIN_MANUAL_BOOST_SPEED_GAIN;
    });
}

fn calculate_altitude(time: Res<Time>, mut query: Query<&mut Coin>) {
    query.for_each_mut(|mut coin| {
        coin.altitude += coin.speed * time.delta_seconds();
        coin.highest_altitude_recorded = coin.highest_altitude_recorded.max(coin.altitude);
    });
}

const COIN_LOSE_SPEED: f32 = -400.0;

fn check_game_over(
    query: Query<&Coin>,
    mut next_state: ResMut<NextState<AppState>>,
    mut scores: ResMut<Scores>,
) {
    let coin = query.single();

    if coin.speed < COIN_LOSE_SPEED {
        scores.register_score((coin.highest_altitude_recorded.ceil() as i32) / 10);
        next_state.set(AppState::End);
    }
}
