use bevy::prelude::*;

use crate::{
    app_state::{AppState, StateOwner},
    coin::Coin,
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

fn init_clouds(mut commands: Commands, query: Query<(&InitCloud, Entity)>) {
    query.for_each(|(init_cloud, init_cloud_entity)| {
        let pos = Vec3::new(init_cloud.0.x, init_cloud.0.y, 0.0);

        commands.get_entity(init_cloud_entity).unwrap().despawn();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(CLOUD_SIZE),
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
