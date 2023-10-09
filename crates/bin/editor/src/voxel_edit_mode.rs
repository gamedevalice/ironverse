use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_iron_ui::core::{LayoutNode, UiTags, UiManager};
use bevy_iron_ui::layout::{node, button, text, image};
use bevy_iron_voxel::{BevyVoxelResource, EditEvent, EditEvents, EditState, Preview};

use super::AppState;

const HOTBAR_KEYS: [bevy::prelude::KeyCode; 10] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
    KeyCode::Key0,
];
pub fn on_enter(mut ui_manager: ResMut<UiManager>, mut commands: Commands, asset_server: Res<AssetServer>, query: Query<(Entity, &UiTags)>, mut window: Query<&mut Window>) {
    ui_manager.set_layout(&mut commands, &asset_server, &query, layout(&asset_server));
    
    //lock the cursor
    let mut window = window.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}
pub fn on_exit(mut window: Query<&mut Window>) {
    
    //unlock the cursor
    let mut window = window.single_mut();
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}

pub fn layout(asset_server: &Res<AssetServer>) -> LayoutNode {
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
    node(vec!["voxel_edit_mode"], vec![
        image(vec!["crosshair"], asset_server.load("images/crosshair.png"), vec![]),
        node(vec!["voxel_edit_mode_controls_list"], vec![
            text(vec!["voxel_edit_mode_controls_text"], "WASD: Move", vec![]),
            text(vec!["voxel_edit_mode_controls_text"], "Mouse: Look", vec![]),
            text(vec!["voxel_edit_mode_controls_text"], "Mouse Wheel: Change Size", vec![]),
            text(vec!["voxel_edit_mode_controls_text"], "Left Click: Perform Edit", vec![]),
            text(vec!["voxel_edit_mode_controls_text"], "Right Click: Toggle Add/Remove", vec![]),
            text(vec!["voxel_edit_mode_controls_text"], "R: Options", vec![]),
            text(vec!["voxel_edit_mode_controls_text"], "ESC: Menu", vec![])
        ]),
        node(vec!["hotbar"], hotbar_buttons)
    ])
}

#[derive(Default, Clone)]
pub struct ControlsLocal {
    pub is_pressing: bool,
    pub pressed_time: f32,
    pub edit_count: i32,
}
pub fn controls(time: Res<Time>, mut local: Local<ControlsLocal>, mut previews: Query<&mut Preview>, mut mouse_wheel: EventReader<MouseWheel>, edit_state_reader: Res<State<EditState>>, mut edit_state_writer: ResMut<NextState<EditState>>, mut edit_event_writer: EventWriter<EditEvents>, mouse: Res<Input<MouseButton>>, keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    //open menu
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    } 
    //open options
    if keyboard_input.just_pressed(KeyCode::R) {
        next_state.set(AppState::VoxelEditOptions);
    }

    //start pressing
    if mouse.just_pressed(MouseButton::Left) {
        local.is_pressing = true;
    }
    //perform edit
    if local.is_pressing {
        local.pressed_time += time.delta_seconds();
        if local.pressed_time > (local.edit_count as f32) * 0.1 {
            if edit_state_reader.get() == &EditState::AddNormal {
                edit_event_writer.send(EditEvents {
                  event: EditEvent::AddCube
                });
            } else {
                edit_event_writer.send(EditEvents {
                  event: EditEvent::RemoveCube
                });
            }
            local.edit_count += 1;
        }
    }
    if mouse.just_released(MouseButton::Left) {
        local.pressed_time = 0.0;
        local.edit_count = 0;
        local.is_pressing = false;
    }

    //toggle add/remove
    if mouse.just_pressed(MouseButton::Right) {
        if edit_state_reader.get() == &EditState::AddNormal {
            edit_state_writer.set(EditState::RemoveNormal);
        } else {
            edit_state_writer.set(EditState::AddNormal);
        }
    }

    //scale edit preview
    for event in mouse_wheel.iter() {
        let y = event.y.clamp(-1.0, 1.0);
        if y < 0.0 {
            for mut preview in previews.iter_mut() {
                if preview.level > 0 {
                    preview.level -= 1;
                    preview.size = 2_u8.pow(preview.level as u32);
                }
            }
        } else if y > 0.0 {
            for mut preview in previews.iter_mut() {
                if preview.level < 3 {
                    preview.level += 1;
                    preview.size = 2_u8.pow(preview.level as u32);
                }
            }
        }
    }
}

    
//update hotbar colors & highlight selected
pub fn update_hotbar(mut ui_query: Query<(&UiTags, &mut BackgroundColor, &mut BorderColor)>, keyboard_input: Res<Input<KeyCode>>, mut previews: Query<&mut Preview>, mut hotbar_voxels: ResMut<HotbarVoxels>, voxel_res: Res<BevyVoxelResource>) {
    //select voxel type with keyboard
    for i in 0..HOTBAR_KEYS.len() {
        if keyboard_input.just_pressed(HOTBAR_KEYS[i]) {
            hotbar_voxels.selected_index = i;
        }
    }

    //update selected voxel
    for mut preview in &mut previews {
        preview.voxel = hotbar_voxels.voxels[hotbar_voxels.selected_index];
    }
    //update hightlighted hotbar item
    for (ui_tags, _, mut border_color) in &mut ui_query.iter_mut() {
        if ui_tags.tags.contains(&"hotbar_item_color".to_string()) {
            *border_color = BorderColor(Color::BLACK);
        }
        let i = hotbar_voxels.selected_index + 1;
        if ui_tags.tags.contains(&format!("hotbar_item_color_{}", i).to_string()) {
            *border_color = BorderColor(Color::GRAY);
        }
    }
    //keep hotbar colors updated
    for (ui_tags, mut background_color, _) in &mut ui_query.iter_mut() {
        if ui_tags.tags.contains(&"hotbar_item_color".to_string()) {
            for i in 1..11 {
                if ui_tags.tags.contains(&format!("hotbar_item_color_{}", i).to_string()) {
                    let i = i - 1;
                    let color = voxel_res.chunk_manager.colors[hotbar_voxels.voxels[i] as usize - 1];
                    *background_color = BackgroundColor(Color::rgb(color[0], color[1], color[2]));
                }
            }
        }
    }
}

//stores hotbar info
#[derive(Resource)]
pub struct HotbarVoxels {
    pub voxels: Vec<u8>,
    pub selected_index: usize,
}
impl Default for HotbarVoxels {
    fn default() -> Self {
        Self {
            voxels: vec![16, 95, 103, 107, 172, 216, 221, 199, 240, 255],
            selected_index: 0,
        }
    }
}