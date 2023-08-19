use bevy::prelude::*;

use crate::{coin::Coin, physics::RelativeCoinY};

pub struct FairyPlugin;

impl Plugin for FairyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_fairies, check_fairy_coin_collision));
    }
}

#[derive(Component)]
pub struct InitFairy(pub Vec2);

#[derive(Component)]
pub struct Fairy;

const FAIRY_SIZE: Vec2 = Vec2::new(60.0, 60.0);

impl Fairy {
    pub fn get_bounds(transform: &Transform) -> Rect {
        Rect::from_center_size(
            Vec2::new(transform.translation.x, transform.translation.y),
            FAIRY_SIZE,
        )
    }
}

fn init_fairies(mut commands: Commands, query: Query<(&InitFairy, Entity)>) {
    query.for_each(|(init_fairy, init_fairy_entity)| {
        commands.get_entity(init_fairy_entity).unwrap().despawn();

        let pos = Vec3::new(init_fairy.0.x, init_fairy.0.y, 0.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::PINK,
                    custom_size: Some(FAIRY_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            RelativeCoinY,
            Fairy,
        ));
    });
}

// should be more powerful than manual boost
const FAIRY_SPEED_BOOST: f32 = 400.0;

fn check_fairy_coin_collision(
    mut commands: Commands,
    mut coin_query: Query<(&mut Coin, &Transform)>,
    fairy_query: Query<(&Transform, Entity), (With<Fairy>, Without<Coin>)>,
) {
    let (mut coin, coin_transform) = coin_query.single_mut();
    let coin_rect = Coin::get_bounds(coin_transform);

    fairy_query.for_each(|(fairy_transform, fairy_entity)| {
        let fairy_rect = Fairy::get_bounds(fairy_transform);

        if !fairy_rect.intersect(coin_rect).is_empty() {
            coin.speed += FAIRY_SPEED_BOOST;
            commands.get_entity(fairy_entity).unwrap().despawn();
        }
    });
}
