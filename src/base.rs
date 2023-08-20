use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Coin in the Sky".to_string(),
                resolution: (960.0, 720.0).into(),
                // wasm: ID of the element to bind to
                canvas: Some("#bevy".to_owned()),
                // wasm: tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}
