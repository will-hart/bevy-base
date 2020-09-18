use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
use spectre_core::prelude::{BuffableStatistic, CharacterStats, Health, Movement, Stats};
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
        .add_plugin(GameTimePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(CharacterStats {
        stats: Stats {
            strength: BuffableStatistic::new(10),
            agility: BuffableStatistic::new(10),
            intelligence: BuffableStatistic::new(10),
        },
        health: Health::new(100),
        movement: Movement {
            movement_speed: BuffableStatistic::new(50),
        },
    });

    commands.spawn((GameSpeedRequest::new(1.0),));
}
