use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{
    app_state::{AppState, StateOwner},
    game_assets::{FontAssets, TextureAssets},
};

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu_ui)
            .add_systems(Update, start_game.run_if(in_state(AppState::MainMenu)));
    }
}

const BACKGROUND_COLOR: Color = Color::rgb(40.0 / 255.0, 40.0 / 255.0, 63.0 / 255.0);

fn setup_main_menu_ui(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    font_assets: Res<FontAssets>,
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
                ..Default::default()
            },
            ..Default::default()
        },
        StateOwner(AppState::MainMenu),
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            StateOwner(AppState::MainMenu),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(64.0),
                                height: Val::Px(64.0),
                                ..Default::default()
                            },
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        },
                        UiImage::new(texture_assets.texture_coin.clone()),
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Coin in the Sky",
                        TextStyle {
                            font_size: 48.0,
                            color: Color::CYAN,
                            ..Default::default()
                        },
                    ));
                });

            parent.spawn(TextBundle::from_section(
                "Flip your coin into the sky, and maintain it as high as possible!",
                TextStyle {
                    font: font_assets.font_fira.clone(),
                    font_size: 28.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(32.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|mut parent| {
                    fn tip_item(
                        parent: &mut ChildBuilder,
                        font_assets: &Res<FontAssets>,
                        image: Handle<Image>,
                        image_width: f32,
                        image_height: f32,
                        text: String,
                    ) {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(8.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(image_width),
                                            height: Val::Px(image_height),
                                            ..Default::default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        ..Default::default()
                                    },
                                    UiImage::new(image),
                                ));
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    TextStyle {
                                        font: font_assets.font_fira.clone(),
                                        font_size: 16.0,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                ));
                            });
                    }
                    tip_item(
                        &mut parent,
                        &font_assets,
                        texture_assets.texture_single_cloud.clone(),
                        64.0,
                        32.0,
                        "Avoid the clouds using the\n[LEFT] and [RIGHT] arrow keys.".to_string(),
                    );
                    tip_item(
                        &mut parent,
                        &font_assets,
                        texture_assets.texture_fairy.clone(),
                        48.0,
                        48.0,
                        "Touch the fairy to gain\nan automatic boost!".to_string(),
                    );
                    tip_item(
                        &mut parent,
                        &font_assets,
                        texture_assets.texture_boost.clone(),
                        48.0,
                        48.0,
                        "Collect manual boost\n(use [SPACE] to activate them).".to_string(),
                    );
                });

            parent.spawn(TextBundle::from_section(
                "Press [SPACE] to start",
                TextStyle {
                    font: font_assets.font_fira.clone(),
                    font_size: 32.0,
                    color: Color::GREEN,
                    ..Default::default()
                },
            ));
        });

    commands.spawn((
        SpriteBundle {
            texture: texture_assets.texture_launcher.clone(),
            sprite: Sprite {
                color: Color::rgb_u8(60, 60, 60),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -240.0, 2.0)),
            ..Default::default()
        },
        StateOwner(AppState::MainMenu),
    ));
}

fn start_game(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::CoinLaunch);
    }
}
