use bevy::prelude::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_coin).add_systems(
            Update,
            (
                handle_coin_gravity,
                do_coin_flip_animation,
                handle_coin_adjustments,
                handle_coin_use_boost,
            ),
        );
    }
}

#[derive(Component)]
pub struct Coin {
    pub speed: f32,
    pub additional_boosts: i32,
}

impl Default for Coin {
    fn default() -> Self {
        Self {
            speed: 0.0,
            additional_boosts: 3,
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
// TODO: Actual sprite and size
const COIN_FULL_SIZE: Vec2 = Vec2::new(50.0, 50.0);

fn setup_coin(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(COIN_FULL_SIZE),
                ..Default::default()
            },
            ..Default::default()
        },
        Coin {
            speed: 1400.0,
            additional_boosts: 3,
        },
        CoinAnimation::default(),
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

const COIN_ADJUSTMENT_X_SPEED: f32 = 150.0;
const COIN_ADJUSTMENT_Y_SPEED_PENALTY: f32 = 200.0;
// TODO: Related to screen bounds?
const COIN_X_BOUND: f32 = 200.0;

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
        transform.translation.x = transform.translation.x.max(-COIN_X_BOUND).min(COIN_X_BOUND);
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
