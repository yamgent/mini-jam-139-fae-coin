use bevy::prelude::*;

use crate::coin::Coin;

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ingame_ui)
            .add_systems(Update, (update_speed_ui, update_additional_boosts_ui));
    }
}

#[derive(Component)]
pub struct SpeedUi;

#[derive(Component)]
pub struct AdditionalBoostsUi;

fn setup_ingame_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Speed: 0",
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        ),
        SpeedUi,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Additional Boosts: 0",
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(32.0),
            left: Val::Px(0.0),
            ..Default::default()
        }),
        AdditionalBoostsUi,
    ));
}

fn update_speed_ui(coin_query: Query<&Coin>, mut query: Query<&mut Text, With<SpeedUi>>) {
    let coin = coin_query.single();
    query.for_each_mut(|mut text| {
        text.sections[0].value = format!("Speed: {:.2}m/s", coin.speed / 10.0);
    });
}

fn update_additional_boosts_ui(
    coin_query: Query<&Coin>,
    mut query: Query<&mut Text, With<AdditionalBoostsUi>>,
) {
    let coin = coin_query.single();
    query.for_each_mut(|mut text| {
        text.sections[0].value = format!("Boosts Remaining: {}", coin.additional_boosts);
    });
}
