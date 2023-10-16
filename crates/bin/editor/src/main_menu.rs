use bevy::prelude::*;
use bevy_iron_ui::core::{LayoutNode, UiTags, UiManager};
use bevy_iron_ui::layout::{node, button, text, image};
use bevy_iron_voxel::data::GameState;

use super::AppState;

pub fn on_enter(mut ui_manager: ResMut<UiManager>, mut commands: Commands, asset_server: Res<AssetServer>, query: Query<(Entity, &UiTags)>) {
    ui_manager.set_layout(&mut commands, &asset_server, &query, layout(&asset_server));
}

pub fn layout(asset_server: &Res<AssetServer>) -> LayoutNode {
    let mut menu_items = vec![
        image(vec!["menu_logo"], asset_server.load("images/logo/anvil-constellation-logo-512.png"), vec![]),
        button(vec!["menu_button", "menu_back_button"], vec![
            text(vec!["button_text", "menu_button_text"], "Back to Game", vec![])
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
    ];
    
    //Only show Quit Button if not on web
    #[cfg(not(target_arch = "wasm32"))]
    menu_items.push(
        button(vec!["menu_button", "menu_quit_button"], vec![
            text(vec!["button_text", "menu_button_text"], "Quit Game", vec![])
        ])
    );

    node(vec!["menu_container"], vec![
        node(vec!["voxel_edit_mode_controls_container"], vec![
            node(vec!["voxel_edit_mode_controls_list"], vec![
                text(vec!["voxel_edit_mode_controls_text"], "WASD: Move", vec![]),
                text(vec!["voxel_edit_mode_controls_text"], "Mouse: Look", vec![]),
                text(vec!["voxel_edit_mode_controls_text"], "Mouse Wheel: Change Size", vec![]),
                text(vec!["voxel_edit_mode_controls_text"], "Left Click: Perform Edit", vec![]),
                text(vec!["voxel_edit_mode_controls_text"], "Right Click: Toggle Add/Remove", vec![]),
                text(vec!["voxel_edit_mode_controls_text"], "R: Options", vec![]),
                text(vec!["voxel_edit_mode_controls_text"], "ESC: Menu", vec![])
            ]),
        ]),
        node(vec!["menu"], menu_items)
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
    mut app_state: ResMut<NextState<AppState>>, 
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, ui_tags) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if ui_tags.tags.contains(&"menu_back_button".to_string()) {
                    app_state.set(AppState::VoxelEditMode);
                } else if ui_tags.tags.contains(&"menu_new_button".to_string()) {
                    game_state.set(GameState::New);
                } else if ui_tags.tags.contains(&"menu_save_button".to_string()) {
                    game_state.set(GameState::SaveGame);
                } else if ui_tags.tags.contains(&"menu_load_button".to_string()) {
                    game_state.set(GameState::LoadGame);
                } else if ui_tags.tags.contains(&"menu_quit_button".to_string()) {
                    exit.send(bevy::app::AppExit);
                }
            },
            _ => {}
        };
    }
}
pub fn controls(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    //Don't allow this hotkey on web since it fails to trigger fullscreen
    #[cfg(not(target_arch = "wasm32"))]
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::VoxelEditMode);
    }
}