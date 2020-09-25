use bevy::prelude::*;
use spectre_state::*;

use super::{ButtonMaterials, MyGameScenes};

pub struct MainMenuSceneEntity;
pub struct MenuButtonText;

pub fn run_menu_scene(
    mut game_state: ResMut<GameState<MyGameScenes>>,
    mut interaction_query: Query<(&Button, Mutated<Interaction>)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Menu) {
        return;
    }

    for (_button, interaction) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                game_state.set_transition(MyGameScenes::Game);
            }
            _ => {}
        }
    }
}

pub fn setup_menu_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    asset_server: Res<AssetServer>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Menu)
        || !game_state.is_in_status(&GameStatus::Entering)
    {
        return;
    }

    let font_handle = asset_server.load("assets/fonts/teletactile.ttf").unwrap();
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonComponents {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Px(15.)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal,
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn(TextComponents {
                        text: Text {
                            value: "Button".to_string(),
                            font: font_handle,
                            style: TextStyle {
                                font_size: 20.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                            },
                        },
                        ..Default::default()
                    });
                })
                .spawn(TextComponents {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text {
                        value: "MY GAME MENU".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(MenuButtonText);
        })
        .with(MainMenuSceneEntity);
}

pub fn teardown_menu_scene(
    mut commands: Commands,
    game_state: Res<GameState<MyGameScenes>>,
    mut menu_scene_items: Query<(Entity, &MainMenuSceneEntity)>,
) {
    if !game_state.is_in_scene(&MyGameScenes::Menu)
        || !game_state.is_in_status(&GameStatus::Exiting)
    {
        return;
    }

    println!("Tearing down loading screen");
    for (entity, _) in &mut menu_scene_items.iter() {
        commands.despawn_recursive(entity);
    }
}
