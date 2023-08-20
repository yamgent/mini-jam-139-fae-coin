use bevy::prelude::*;
use rand::Rng;

use crate::{
    app_state::{AppState, StateOwner},
    coin::Coin,
    game_assets::TextureAssets,
    physics::RelativeCoinY,
};

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_clouds, check_cloud_coin_collision).run_if(in_state(AppState::Ingame)),
        );
    }
}

#[derive(Component)]
pub struct InitCloud(pub Vec2);

#[derive(Component)]
struct Cloud {
    active: bool,
}

impl Default for Cloud {
    fn default() -> Self {
        Self { active: true }
    }
}

const CLOUD_SIZE: Vec2 = Vec2::new(100.0, 30.0);

impl Cloud {
    pub fn get_bounds(transform: &Transform) -> Rect {
        Rect::from_center_size(
            Vec2::new(transform.translation.x, transform.translation.y),
            CLOUD_SIZE,
        )
    }
}

const CLOUD_SPRITE_TOTAL: i32 = 8;
const CLOUD_SPRITE_PER_ROW_COUNT: i32 = 2;
const CLOUD_SPRITE_SIZE: Vec2 = Vec2::new(128.0, 64.0);

fn init_clouds(
    mut commands: Commands,
    query: Query<(&InitCloud, Entity)>,
    texture_assets: Res<TextureAssets>,
) {
    if query.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    query.for_each(|(init_cloud, init_cloud_entity)| {
        let pos = Vec3::new(init_cloud.0.x, init_cloud.0.y, 0.0);

        let sprite_to_use = rng.gen_range(0..CLOUD_SPRITE_TOTAL);
        let sprite_min = Vec2::new(
            (sprite_to_use % CLOUD_SPRITE_PER_ROW_COUNT) as f32 * CLOUD_SPRITE_SIZE.x,
            (sprite_to_use / CLOUD_SPRITE_PER_ROW_COUNT) as f32 * CLOUD_SPRITE_SIZE.y,
        );
        let sprite_max = Vec2::new(
            sprite_min.x + CLOUD_SPRITE_SIZE.x,
            sprite_min.y + CLOUD_SPRITE_SIZE.y,
        );

        commands.get_entity(init_cloud_entity).unwrap().despawn();
        commands.spawn((
            SpriteBundle {
                texture: texture_assets.texture_clouds.clone(),
                sprite: Sprite {
                    rect: Some(Rect {
                        min: sprite_min,
                        max: sprite_max,
                    }),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            RelativeCoinY,
            Cloud::default(),
            StateOwner(AppState::Ingame),
        ));
    });
}

const CLOUD_SLOW_DOWN_PENALTY: f32 = 200.0;

fn check_cloud_coin_collision(
    mut coin_query: Query<(&mut Coin, &Transform)>,
    mut cloud_query: Query<(&mut Cloud, &Transform, &mut Sprite), Without<Coin>>,
) {
    let (mut coin, coin_transform) = coin_query.single_mut();

    if coin.speed < 0.0 {
        return;
    }

    let coin_rect = Coin::get_bounds(coin_transform);

    cloud_query.for_each_mut(|(mut cloud, cloud_transform, mut cloud_sprite)| {
        if !cloud.active {
            return;
        }

        let cloud_rect = Cloud::get_bounds(cloud_transform);

        if !cloud_rect.intersect(coin_rect).is_empty() {
            cloud.active = false;
            cloud_sprite.color = Color::GRAY;
            coin.speed -= CLOUD_SLOW_DOWN_PENALTY;
            coin.speed = coin.speed.max(0.0);
        }
    });
}
