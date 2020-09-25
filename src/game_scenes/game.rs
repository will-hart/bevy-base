use bevy::prelude::*;
use spectre_animations::prelude::spawn_animated_spritesheet;
use spectre_state::*;

use super::MyGameScenes;
use crate::constants::ANIMATED_SPRITESHEED_ID;

pub struct GameSceneEntity;

pub fn setup_game_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    asset_server: Res<AssetServer>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Game)
        || !game_state.is_in_status(&GameStatus::Entering)
    {
        return;
    }

    let font_handle = asset_server.load("assets/fonts/teletactile.ttf").unwrap();

    commands
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "Press [SPACE] to spawn a unit".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .with(GameSceneEntity);
}

// demonstrates spawning a player using the spawn_animated_spritesheet helper
pub fn run_game_scene(
    commands: Commands,
    input: Res<Input<KeyCode>>,
    game_state: Res<GameState<MyGameScenes>>,
    textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Game) {
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let handle: Handle<Texture> = Handle::from_u128(ANIMATED_SPRITESHEED_ID);
    let texture = textures.get(&handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(handle, texture.size, 9, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    spawn_animated_spritesheet(
        commands,
        texture_atlas_handle,
        0.05,
        vec![(0, 8), (9, 17), (18, 26), (27, 35)],
        Vec3::new(0., 0., 0.),
    )
}

pub fn teardown_game_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut loading_scene_items: Query<(Entity, &GameSceneEntity)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Game)
        || !game_state.is_in_status(&GameStatus::Exiting)
    {
        return;
    }

    println!("Tearing down loading screen");
    for (entity, _) in &mut loading_scene_items.iter() {
        commands.despawn(entity);
    }
}
