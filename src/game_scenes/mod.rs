use bevy::prelude::*;
use spectre_loaders::LoadingStatus;
use spectre_state::*;

mod game;
mod loading;

use game::run_game_scene;
use loading::run_loading_scene;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum MyGameScenes {
    Loading,
    Menu,
    Game,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GameState::<MyGameScenes> {
            status: GameStateStatus::Idle,
            current: None,
            next: None,
        })
        .add_system(game_state_transitions.system())
        .add_system(run_loading_scene.system())
        .add_system(run_game_scene.system());
    }
}

fn game_state_transitions(
    loading: Res<LoadingStatus>,
    mut game_state: ResMut<GameState<MyGameScenes>>,
) {
    game_state.update();
    match game_state.status {
        GameStateStatus::Idle => game_state.set_transition(MyGameScenes::Loading),
        GameStateStatus::Running => match game_state.current {
            Some(MyGameScenes::Loading) => {
                if loading.initial_load_done {
                    game_state.set_transition(MyGameScenes::Menu);
                }
            }
            _ => return,
        },
        _ => return,
    };
}
