use bevy::{prelude::*, input::mouse::MouseButtonInput, window::CursorGrabMode};
use bevy_flycam::MovementSettings;
use crate::{input::{MouseInput, InputResource}, data::{CursorState, UIState}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update)
      .add_system(toggle_mouse_grab.in_base_set(CoreSet::PostUpdate))
      .add_system(cursor_free.in_schedule(OnEnter(CursorState::None)))
      .add_system(cursor_locked.in_schedule(OnEnter(CursorState::Locked)))
      ;
  }
}

fn update(
  mut mouse_events: EventReader<MouseButtonInput>,
  mut mouse_inputs: EventWriter<MouseInput>,
  cursor_state: Res<State<CursorState>>,
) {
  for event in mouse_events.iter() {
    if cursor_state.0 == CursorState::None {
      return;
    }

    mouse_inputs.send(MouseInput { mouse_button_input: event.clone() });
  }
}


fn toggle_mouse_grab(
  mouse: Res<Input<MouseButton>>,
  key: Res<Input<KeyCode>>,
  mut cursor_state_next: ResMut<NextState<CursorState>>,
  cursor_state: Res<State<CursorState>>,

  input_res: Res<InputResource>,

  ui_state: Res<State<UIState>>,
) {
  if !input_res.enabled {
    return;
  }

  if ui_state.0 != UIState::Default {
    return;
  }

  if mouse.just_pressed(MouseButton::Left) {
    match cursor_state.0 {
      CursorState::None => {
        cursor_state_next.set(CursorState::Locked);
      },
      CursorState::Locked => {}
    };
  }

  if key.just_pressed(KeyCode::Escape) {
    match cursor_state.0 {
      CursorState::None => {
        cursor_state_next.set(CursorState::Locked);
      },
      CursorState::Locked => {
        cursor_state_next.set(CursorState::None);
      }
    };
  }
}


fn cursor_free(
  mut windows: Query<&mut Window>,
  mut move_setting_res: ResMut<MovementSettings>,
) {
  let mut window = windows.single_mut();
  window.cursor.visible = true;
  window.cursor.grab_mode = CursorGrabMode::None;

  move_setting_res.sensitivity = 0.0;
  move_setting_res.speed = 0.0;
}

fn cursor_locked(
  mut windows: Query<&mut Window>,
  mut move_setting_res: ResMut<MovementSettings>,
) {
  let mut window = windows.single_mut();
  window.cursor.visible = false;
  window.cursor.grab_mode = CursorGrabMode::Locked;

  move_setting_res.sensitivity = 0.00012;
  move_setting_res.speed = 24.0;
  // move_setting_res.speed = 6.0;
}

