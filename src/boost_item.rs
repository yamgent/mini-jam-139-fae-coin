use bevy::prelude::*;

use crate::{coin::Coin, physics::RelativeCoinY};

pub struct BoostItemPlugin;

impl Plugin for BoostItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_boost_items, check_boost_item_coin_collision));
    }
}

#[derive(Component)]
pub struct InitBoostItem(pub Vec2);

#[derive(Component)]
pub struct BoostItem;

const BOOST_ITEM_SIZE: Vec2 = Vec2::new(40.0, 40.0);

impl BoostItem {
    pub fn get_bounds(transform: &Transform) -> Rect {
        Rect::from_center_size(
            Vec2::new(transform.translation.x, transform.translation.y),
            BOOST_ITEM_SIZE,
        )
    }
}

fn init_boost_items(mut commands: Commands, query: Query<(&InitBoostItem, Entity)>) {
    query.for_each(|(init_item, init_item_entity)| {
        let pos = Vec3::new(init_item.0.x, init_item.0.y, 0.0);

        commands.get_entity(init_item_entity).unwrap().despawn();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(BOOST_ITEM_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            RelativeCoinY,
            BoostItem,
        ));
    });
}

fn check_boost_item_coin_collision(
    mut commands: Commands,
    mut coin_query: Query<(&mut Coin, &Transform)>,
    item_query: Query<(&Transform, Entity), (With<BoostItem>, Without<Coin>)>,
) {
    let (mut coin, coin_transform) = coin_query.single_mut();
    let coin_rect = Coin::get_bounds(coin_transform);

    item_query.for_each(|(item_transform, item_entity)| {
        let item_rect = BoostItem::get_bounds(item_transform);

        if !item_rect.intersect(coin_rect).is_empty() {
            coin.additional_boosts += 1;
            commands.get_entity(item_entity).unwrap().despawn();
        }
    });
}
