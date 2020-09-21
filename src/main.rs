use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
use spectre_animations::prelude::{spawn_animated_spritesheet, AnimationPlugin};
use spectre_combat::prelude::AllegiancePlugin;
use spectre_core::prelude::{BuffableStatistic, CharacterStats, Health, Mana, Movement, Stats};
use spectre_loaders::{AssetsToLoad, ResourceLoaderPlugin};
use spectre_time::{GameSpeedRequest, GameTimePlugin};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Spectre".to_string(),
            width: 1024,
            height: 768,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.005, 0.005, 0.005)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(spawn_player_debug.system())
        .add_plugin(GameTimePlugin)
        .add_plugin(ResourceLoaderPlugin)
        .add_plugin(AllegiancePlugin)
        .add_plugin(AnimationPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn the camera
    commands.spawn(Camera2dComponents::default());

    // spawn a "character" with stats
    commands.spawn(CharacterStats {
        stats: Stats {
            strength: BuffableStatistic::new(10.),
            agility: BuffableStatistic::new(10.),
            intelligence: BuffableStatistic::new(10.),
            is_changed: true,
        },
        health: Health::new(100.),
        mana: Mana::new(200.),
        movement: Movement {
            movement_speed: BuffableStatistic::new(50.),
        },
    });

    // this loaders approach requires at least one tick of the game loop before
    // assets handles are available, therefore can't directly spawn player sprite here
    commands.spawn((AssetsToLoad {
        asset_paths: vec!["assets/walk_sprite_sheet.png"],
    },));

    // start the game
    commands.spawn((GameSpeedRequest::new(1.0),));
}

// demonstrates spawning a player using the spawn_animated_spritesheet helper
fn spawn_player_debug(
    commands: Commands,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    // cheat and load a sprite sheet animation synchronously
    let texture_handle = asset_server
        .get_handle("assets/walk_sprite_sheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 9, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    spawn_animated_spritesheet(
        commands,
        texture_atlas_handle,
        0.05,
        vec![(0, 8), (9, 17), (18, 26), (27, 35)],
        Vec3::new(0., 0., 0.),
    )
}
