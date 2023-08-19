use bevy::prelude::*;

use crate::coin::Coin;

pub struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ingame_ui)
            .add_systems(Update, update_speed_ui);
    }
}

#[derive(Component)]
pub struct SpeedUi;

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
}

fn update_speed_ui(coin_query: Query<&Coin>, mut query: Query<&mut Text, With<SpeedUi>>) {
    let coin = coin_query.single();
    query.for_each_mut(|mut text| {
        text.sections[0].value = format!("Speed: {:.2}m/s", coin.speed / 10.0);
    });
}
