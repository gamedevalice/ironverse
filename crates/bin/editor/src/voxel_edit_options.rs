use bevy::prelude::*;
use bevy_iron_ui::core::{LayoutNode, UiManager, UiTags};
use bevy_iron_ui::layout::{node, button, text};
use bevy_iron_voxel::{Preview, BevyVoxelResource};
use super::voxel_edit_mode::HotbarVoxels;

use super::AppState;

pub fn on_enter(mut ui_manager: ResMut<UiManager>, mut commands: Commands, asset_server: Res<AssetServer>, query: Query<(Entity, &UiTags)>) {
    ui_manager.set_layout(&mut commands, &asset_server, &query, layout());
}
pub fn layout() -> LayoutNode {
    let mut hotbar_buttons = vec![];
    for i in 1..11 {
        let button_text = if i == 10 { "0".to_string() } else { i.to_string() };
        hotbar_buttons.push(
            button(vec!["hotbar_button", format!("hotbar_button_{}", i).as_str()], vec![
                node(vec!["hotbar_item_color", format!("hotbar_item_color_{}", i).as_str()], vec![]),
                text(vec!["button_text"], button_text.as_str(), vec![])
            ])
        );
    }
    let mut color_picker_buttons = vec![];
    for i in 0..255 {
        color_picker_buttons.push(
            button(vec!["color_picker_button", format!("color_picker_button_{}", i).as_str()], vec![
                node(vec!["color_picker_item_color", format!("color_picker_item_color_{}", i).as_str()], vec![]),
            ])
        );
    }

    node(vec!["voxel_edit_options"], vec![
        node(vec!["hotbar"], hotbar_buttons),
        node(vec!["color_picker"], color_picker_buttons),
        node(vec!["color_edit"], vec![
            node(vec!["rgb_edit"], vec![
                node(vec!["rgb_channel_edit"], vec![
                    //Edit R
                    button(vec!["+-_button", "edit_r-"], vec![
                        text(vec!["button_text"], "-", vec![]),
                    ]),
                    text(vec!["button_text", "r_value"], "R: 255", vec![]),
                    button(vec!["+-_button", "edit_r+"], vec![
                        text(vec!["button_text"], "+", vec![]),
                    ]),
                ]),
                node(vec!["rgb_channel_edit"], vec![
                    //Edit G
                    button(vec!["+-_button", "edit_g-"], vec![
                        text(vec!["button_text"], "-", vec![]),
                    ]),
                    text(vec!["button_text", "g_value"], "G: 255", vec![]),
                    button(vec!["+-_button", "edit_g+"], vec![
                        text(vec!["button_text"], "+", vec![]),
                    ]),
                ]),
                node(vec!["rgb_channel_edit"], vec![
                    //Edit B
                    button(vec!["+-_button", "edit_b-"], vec![
                        text(vec!["button_text"], "-", vec![]),
                    ]),
                    text(vec!["button_text", "b_value"], "B: 255", vec![]),
                    button(vec!["+-_button", "edit_b+"], vec![
                        text(vec!["button_text"], "+", vec![]),
                    ]),
                ]),
            ])
        ]),
    ])
}

pub fn controls(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    } else if keyboard_input.just_pressed(KeyCode::R) {
        next_state.set(AppState::VoxelEditMode);
    }
}

pub fn update_color_picker(mouse: Res<Input<MouseButton>>, time: Res<Time>, mut local: Local<ControlsLocal>, mut _previews: Query<&mut Preview>, mut button_query: Query<(&UiTags, &Interaction)>, mut color_picker_query: Query<(&UiTags, &mut BackgroundColor, &mut BorderColor)>, mut rgb_text_query: Query<(&UiTags, &mut Text)>, mut hotbar_voxels: ResMut<HotbarVoxels>, mut voxel_res: ResMut<BevyVoxelResource>) {

    while voxel_res.chunk_manager.colors.len() < 255 {
        voxel_res.chunk_manager.colors.push([0.8, 0.8, 0.8]);
    }

    //update color picker colors
    for (ui_tags, mut background_color, mut border_color) in &mut color_picker_query.iter_mut() {
        if ui_tags.tags.contains(&"color_picker_item_color".to_string()) {
            //highlight selected color
            *border_color = BorderColor(Color::BLACK);
            if ui_tags.tags.contains(&format!("color_picker_item_color_{}", hotbar_voxels.voxels[hotbar_voxels.selected_index] - 1).to_string()) {
                *border_color = BorderColor(Color::GRAY);
            }
            //show colors
            for i in 0..255 {
                let color = voxel_res.chunk_manager.colors[i];
                if ui_tags.tags.contains(&format!("color_picker_item_color_{}", i).to_string()) {
                    *background_color = Color::rgb(color[0], color[1], color[2]).into();
                }
            }
        }
    }

    //update rgb text
    let index = (hotbar_voxels.voxels[hotbar_voxels.selected_index] - 1) as usize;
    for (ui_tags, mut text) in &mut rgb_text_query.iter_mut() {
        if ui_tags.tags.contains(&"r_value".to_string()) {
            text.sections[0].value = format!("R: {:.0}", voxel_res.chunk_manager.colors[index][0] * 255.0);
        }
        if ui_tags.tags.contains(&"g_value".to_string()) {
            text.sections[0].value = format!("G: {:.0}", voxel_res.chunk_manager.colors[index][1] * 255.0);
        }
        if ui_tags.tags.contains(&"b_value".to_string()) {
            text.sections[0].value = format!("B: {:.0}", voxel_res.chunk_manager.colors[index][2] * 255.0);
        }
    }

    //update color
    let index = (hotbar_voxels.voxels[hotbar_voxels.selected_index] - 1) as usize;
    for (ui_tags, interaction) in &mut button_query {
        if interaction == &Interaction::Pressed {
            local.is_pressing = true;
        }
        if local.is_pressing && (interaction == &Interaction::Pressed || interaction == &Interaction::Hovered) {
            local.pressed_time += time.delta_seconds();
            if local.pressed_time > (local.edit_count as f32) * 0.02 {
                if ui_tags.tags.contains(&"edit_r-".to_string()) {
                    voxel_res.chunk_manager.colors[index][0] -= 1.0 / 255.0;
                    voxel_res.chunk_manager.colors[index][0] = voxel_res.chunk_manager.colors[index][0].clamp(0.0, 1.0);
                    local.edit_count += 1;
                }
                if ui_tags.tags.contains(&"edit_r+".to_string()) {
                    voxel_res.chunk_manager.colors[index][0] += 1.0 / 255.0;
                    voxel_res.chunk_manager.colors[index][0] = voxel_res.chunk_manager.colors[index][0].clamp(0.0, 1.0);
                    local.edit_count += 1;
                }
                if ui_tags.tags.contains(&"edit_g-".to_string()) {
                    voxel_res.chunk_manager.colors[index][1] -= 1.0 / 255.0;
                    voxel_res.chunk_manager.colors[index][1] = voxel_res.chunk_manager.colors[index][1].clamp(0.0, 1.0);
                    local.edit_count += 1;
                }
                if ui_tags.tags.contains(&"edit_g+".to_string()) {
                    voxel_res.chunk_manager.colors[index][1] += 1.0 / 255.0;
                    voxel_res.chunk_manager.colors[index][1] = voxel_res.chunk_manager.colors[index][1].clamp(0.0, 1.0);
                    local.edit_count += 1;
                }
                if ui_tags.tags.contains(&"edit_b-".to_string()) {
                    voxel_res.chunk_manager.colors[index][2] -= 1.0 / 255.0;
                    voxel_res.chunk_manager.colors[index][2] = voxel_res.chunk_manager.colors[index][2].clamp(0.0, 1.0);
                    local.edit_count += 1;
                }
                if ui_tags.tags.contains(&"edit_b+".to_string()) {
                    voxel_res.chunk_manager.colors[index][2] += 1.0 / 255.0;
                    voxel_res.chunk_manager.colors[index][2] = voxel_res.chunk_manager.colors[index][2].clamp(0.0, 1.0);
                    local.edit_count += 1;
                }
            }
        }
    }
    
    if mouse.just_released(MouseButton::Left) {
        local.pressed_time = 0.0;
        local.edit_count = 0;
        local.is_pressing = false;
        voxel_res.update_colors();
    }

    //change voxel assigned to hotbar
    for (ui_tags, interaction) in &mut button_query {
        if interaction == &Interaction::Pressed {
            if ui_tags.tags.contains(&"color_picker_button".to_string()) {
                for i in 0..255 {
                    if ui_tags.tags.contains(&format!("color_picker_button_{}", i).to_string()) {
                        let selected = hotbar_voxels.selected_index;
                        hotbar_voxels.voxels[selected] = i as u8 + 1;
                    }
                }
            }
        }
    }

    //select from hotbar using click
    for (ui_tags, interaction) in &mut button_query {
        if ui_tags.tags.contains(&"hotbar_button".to_string()) {
            for i in 1..11 {
                if ui_tags.tags.contains(&format!("hotbar_button_{}", i).to_string()) {
                    if interaction == &Interaction::Pressed {
                        hotbar_voxels.selected_index = i - 1;
                    }
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct ControlsLocal {
    pub is_pressing: bool,
    pub pressed_time: f32,
    pub edit_count: i32,
}