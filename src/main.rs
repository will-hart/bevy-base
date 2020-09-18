use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};

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
        .run();
}
