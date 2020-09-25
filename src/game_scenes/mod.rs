use bevy::prelude::*;
use spectre_loaders::LoadingStatus;
use spectre_state::*;

mod game;
mod loading;
mod main_menu;

use game::*;
use loading::*;
use main_menu::*;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum MyGameScenes {
    Loading,
    Menu,
    Game,
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_resource(GameState::<MyGameScenes> {
                status: GameStatus::Idle,
                current: None,
                next: None,
            })
            .add_system(game_state_transitions.system())
            // loading scene - TODO as plugin?
            .add_system(setup_loading_scene.system())
            .add_system(run_loading_scene.system())
            .add_system(teardown_loading_scene.system())
            // menu scene - TODO as plugin?
            .add_system(setup_menu_scene.system())
            .add_system(run_menu_scene.system())
            .add_system(teardown_menu_scene.system())
            // game scene - TODO as plugin?
            .add_system(setup_game_scene.system())
            .add_system(run_game_scene.system())
            .add_system(teardown_game_scene.system());
    }
}

fn game_state_transitions(
    loading: Res<LoadingStatus>,
    mut game_state: ResMut<GameState<MyGameScenes>>,
) {
    game_state.update();
    match game_state.status {
        GameStatus::Idle => game_state.set_transition(MyGameScenes::Loading),
        GameStatus::Running => match game_state.current {
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
