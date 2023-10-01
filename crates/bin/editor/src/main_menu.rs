use bevy::prelude::*;
use bevy_iron_ui::core::{LayoutNode, UiTags, UiManager};
use bevy_iron_ui::layout::{node, button, text, image};

use super::AppState;

pub fn on_enter(mut ui_manager: ResMut<UiManager>, mut commands: Commands, asset_server: Res<AssetServer>, query: Query<(Entity, &UiTags)>) {
    ui_manager.set_layout(&mut commands, &asset_server, &query, layout(&asset_server));
}

pub fn layout(asset_server: &Res<AssetServer>) -> LayoutNode {
    node(vec!["menu"], vec![
        image(vec!["menu_logo"], asset_server.load("images/logo/anvil-constellation-logo-512.png"), vec![]),
        button(vec!["menu_button", "menu_back_button"], vec![
            text(vec!["button_text", "menu_button_text"], "Back to Game (ESC)", vec![])
        ]),
        button(vec!["menu_button", "menu_new_button"], vec![
            text(vec!["button_text", "menu_button_text"], "New World", vec![])
        ]),
        button(vec!["menu_button", "menu_save_button"], vec![
            text(vec!["button_text", "menu_button_text"], "Save World", vec![])
        ]),
        button(vec!["menu_button", "menu_load_button"], vec![
            text(vec!["button_text", "menu_button_text"], "Load World", vec![])
        ]),
        button(vec!["menu_button", "menu_quit_button"], vec![
            text(vec!["button_text", "menu_button_text"], "Quit Game", vec![])
        ]),
    ])
}

pub fn button_actions(
    mut interaction_query: Query<
        (
            &Interaction,
            &UiTags,
        ),
        (Changed<Interaction>, With<Button>),
    >, 
    mut exit: EventWriter<bevy::app::AppExit>, 
    mut next_state: ResMut<NextState<AppState>>
) {
    for (interaction, ui_tags) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if ui_tags.tags.contains(&"menu_back_button".to_string()) {
                    next_state.set(AppState::VoxelEditMode);
                } else if ui_tags.tags.contains(&"menu_new_button".to_string()) {
                } else if ui_tags.tags.contains(&"menu_save_button".to_string()) {
                } else if ui_tags.tags.contains(&"menu_load_button".to_string()) {
                } else if ui_tags.tags.contains(&"menu_quit_button".to_string()) {
                    exit.send(bevy::app::AppExit);
                }
            },
            _ => {}
        };
    }
}
pub fn controls(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::VoxelEditMode);
    }
}