use bevy::prelude::*;
use spectre_loaders::LoadingStatus;
use spectre_state::*;

use super::MyGameScenes;

pub struct LoadingSceneItem;

pub fn run_loading_scene(
    commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    loading_state: Res<LoadingStatus>,
    asset_server: Res<AssetServer>,
    loading_scene_items: Query<(Entity, &LoadingSceneItem)>,
    loading_text: Query<With<LoadingSceneItem, &mut Text>>,
) {
    let matched = match game_state.current {
        Some(MyGameScenes::Loading) => true,
        _ => false,
    };

    if !matched {
        return;
    }

    match game_state.status {
        GameStateStatus::Entered => setup(commands, asset_server),
        GameStateStatus::Running => run(loading_state, loading_text),
        GameStateStatus::Exiting => teardown(commands, loading_scene_items),
        GameStateStatus::Idle => {}
    };
}

fn run(loading_state: Res<LoadingStatus>, mut query: Query<With<LoadingSceneItem, &mut Text>>) {
    println!("Running loading screen");
    for mut text in &mut query.iter() {
        text.value = format!(
            "Loading {} of {}",
            loading_state.items_loaded, loading_state.items_to_load
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Setting up loading screen");
    let font_handle = asset_server.load("assets/fonts/teletactile.ttf").unwrap();
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "Loading...".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .with(LoadingSceneItem);
}

fn teardown(mut commands: Commands, mut loading_scene_items: Query<(Entity, &LoadingSceneItem)>) {
    println!("Tearing down loading screen");
    for (entity, _) in &mut loading_scene_items.iter() {
        commands.despawn(entity);
    }
}
