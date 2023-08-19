use bevy::prelude::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_coin)
            .add_systems(Update, (coin_gravity, coin_flip_animation));
    }
}

#[derive(Component)]
pub struct Coin {
    speed: f32,
}

impl Default for Coin {
    fn default() -> Self {
        Self { speed: 0.0 }
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

const GRAVITY: f32 = 9.8;
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
        Coin { speed: 140.0 },
        CoinAnimation::default(),
    ));
}

fn coin_gravity(time: Res<Time>, mut query: Query<&mut Coin>) {
    query.for_each_mut(|mut coin| {
        coin.speed += -GRAVITY * time.delta_seconds();
    });
}

const COIN_ANIM_MAX_COIN_SPEED_CAP: f32 = 240.0;
const COIN_ANIM_MIN_COIN_SPEED_CAP: f32 = 10.0;
const COIN_ANIM_MAX_SPIN_SPEED: f32 = 15.0;

fn coin_flip_animation(
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
