use bevy::prelude::*;

pub struct AppStatePlugin;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    CoinLaunch,
    Ingame,
    End,
}

#[derive(Component)]
pub struct StateOwner(pub AppState);

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(OnExit(AppState::Loading), remove_loading)
            .add_systems(OnExit(AppState::MainMenu), remove_main_menu)
            .add_systems(OnExit(AppState::CoinLaunch), remove_coin_launch)
            .add_systems(OnExit(AppState::Ingame), remove_ingame)
            .add_systems(OnExit(AppState::End), remove_end);
    }
}

fn remove_entities(
    commands: &mut Commands,
    query: &Query<(Entity, &StateOwner)>,
    exit_state: AppState,
) {
    query.for_each(|(entity, owner)| {
        if owner.0 == exit_state {
            commands.get_entity(entity).unwrap().despawn_recursive();
        }
    });
}

fn remove_loading(mut commands: Commands, query: Query<(Entity, &StateOwner)>) {
    remove_entities(&mut commands, &query, AppState::Loading);
}

fn remove_main_menu(mut commands: Commands, query: Query<(Entity, &StateOwner)>) {
    remove_entities(&mut commands, &query, AppState::MainMenu);
}

fn remove_coin_launch(mut commands: Commands, query: Query<(Entity, &StateOwner)>) {
    remove_entities(&mut commands, &query, AppState::CoinLaunch);
}

fn remove_ingame(mut commands: Commands, query: Query<(Entity, &StateOwner)>) {
    remove_entities(&mut commands, &query, AppState::Ingame);
}

fn remove_end(mut commands: Commands, query: Query<(Entity, &StateOwner)>) {
    remove_entities(&mut commands, &query, AppState::End);
}
