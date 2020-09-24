use bevy::prelude::*;
use spectre_animations::prelude::spawn_animated_spritesheet;
use spectre_state::*;

use super::MyGameScenes;
use crate::constants::ANIMATED_SPRITESHEED_ID;

// demonstrates spawning a player using the spawn_animated_spritesheet helper
pub fn run_game_scene(
    commands: Commands,
    input: Res<Input<KeyCode>>,
    game_state: Res<GameState<MyGameScenes>>,
    textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let matched = match game_state.current {
        Some(MyGameScenes::Game) => match game_state.status {
            GameStateStatus::Running => true,
            _ => false,
        },
        _ => false,
    };

    // only run if in Game scene and running, otherwise just exit
    if !matched {
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
