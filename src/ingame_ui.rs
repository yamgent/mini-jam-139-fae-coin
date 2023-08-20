use bevy::prelude::*;

use crate::{
    app_state::{AppState, StateOwner},
    coin::Coin,
};

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Ingame), setup_ingame_ui)
            .add_systems(
                Update,
                (
                    update_speed_ui,
                    update_additional_boosts_ui,
                    update_highest_altitude_ui,
                    update_altitude_ui,
                )
                    .run_if(in_state(AppState::Ingame)),
            );
    }
}

#[derive(Component)]
pub struct SpeedUi;

#[derive(Component)]
pub struct AdditionalBoostsUi;

#[derive(Component)]
pub struct HighestAltitudeUi;

#[derive(Component)]
pub struct AltitudeUi;

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
        StateOwner(AppState::Ingame),
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
        StateOwner(AppState::Ingame),
    ));

    commands.spawn((
        TextBundle::from_section(
            "Highest Altitude: 0m",
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(128.0),
            left: Val::Px(0.0),
            ..Default::default()
        }),
        HighestAltitudeUi,
        StateOwner(AppState::Ingame),
    ));

    commands.spawn((
        TextBundle::from_section(
            "Altitude: 0.0m",
            TextStyle {
                font_size: 32.0,
                color: Color::GREEN,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(128.0 + 32.0),
            left: Val::Px(0.0),
            ..Default::default()
        }),
        AltitudeUi,
        StateOwner(AppState::Ingame),
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

fn update_highest_altitude_ui(
    coin_query: Query<&Coin>,
    mut query: Query<&mut Text, With<HighestAltitudeUi>>,
) {
    let coin = coin_query.single();
    query.for_each_mut(|mut text| {
        text.sections[0].value = format!(
            "Highest Altitude: {}m",
            (coin.highest_altitude_recorded.floor() as i32) / 10
        );
    });
}

fn update_altitude_ui(coin_query: Query<&Coin>, mut query: Query<&mut Text, With<AltitudeUi>>) {
    let coin = coin_query.single();
    query.for_each_mut(|mut text| {
        text.sections[0].value = format!("Altitude: {:.3}m", coin.altitude / 10.0);
    });
}
