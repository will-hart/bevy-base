use bevy::prelude::*;
use spectre_loaders::LoadingStatus;
use spectre_state::*;

use super::MyGameScenes;

pub struct LoadingSceneEntity;

pub fn run_loading_scene(
    game_state: Res<GameState<MyGameScenes>>,
    loading_state: Res<LoadingStatus>,
    mut loading_text: Query<With<LoadingSceneEntity, &mut Text>>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Loading)
        || !game_state.is_in_status(&GameStatus::Running)
    {
        return;
    }

    println!("Running loading screen");
    for mut text in &mut loading_text.iter() {
        text.value = format!(
            "Loading {} of {}",
            loading_state.items_loaded, loading_state.items_to_load
        );
    }
}

pub fn setup_loading_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    asset_server: Res<AssetServer>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Loading)
        || !game_state.is_in_status(&GameStatus::Entering)
    {
        return;
    }

    println!("Setting up loading screen");
    let font_handle = asset_server.load("assets/fonts/teletactile.ttf").unwrap();
    commands
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
        .with(LoadingSceneEntity);
}

pub fn teardown_loading_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut loading_scene_items: Query<(Entity, &LoadingSceneEntity)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Loading)
        || !game_state.is_in_status(&GameStatus::Exiting)
    {
        return;
    }

    println!("Tearing down loading screen");
    for (entity, _) in &mut loading_scene_items.iter() {
        commands.despawn(entity);
    }
}
