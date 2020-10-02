use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
use spectre_animations::prelude::AnimationPlugin;
use spectre_combat::prelude::AllegiancePlugin;
use spectre_core::prelude::{BuffableStatistic, CharacterStats, Health, Mana, Movement, Stats};
use spectre_loaders::{LoadAssets, ResourceLoaderPlugin};
use spectre_time::{GameSpeedRequest, GameTimePlugin};

mod constants;
mod data;
mod game_scenes;

use constants::ANIMATED_SPRITESHEED_ID;
use data::DataFileLoaderPlugin;
use game_scenes::*;

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
        .add_plugin(GameTimePlugin)
        .add_plugin(ResourceLoaderPlugin)
        .add_plugin(DataFileLoaderPlugin)
        .add_plugin(AllegiancePlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(GameStatePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn the camera
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // spawn a "character" with stats
        .spawn(CharacterStats {
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
        })
        // this loaders approach requires at least one tick of the game loop before
        // assets handles are available, therefore can't directly spawn player sprite here
        .spawn((LoadAssets {
            assets: vec![("assets/walk_sprite_sheet.png", ANIMATED_SPRITESHEED_ID)]
                .into_iter()
                .map(|a| a.into())
                .collect(),
        },))
        // start the game clock running
        .spawn((GameSpeedRequest::new(1.0),));
}
