use bevy::{prelude::*, input::mouse::MouseButtonInput};
use bevy_flycam::MovementSettings;
use crate::{input::{MouseInput, InputResource}, data::{CursorState, UIState}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update)
      .add_systems(PostUpdate, toggle_mouse_grab)
      .add_systems(OnEnter(CursorState::None), cursor_free)
      .add_systems(OnEnter(CursorState::Locked), cursor_locked)
      ;
  }
}

fn update(
  mut mouse_events: EventReader<MouseButtonInput>,
  mut mouse_inputs: EventWriter<MouseInput>,
  cursor_state: Res<State<CursorState>>,
) {
  for event in mouse_events.iter() {
    if *State::get(&cursor_state) == CursorState::None {
      return;
    }

    mouse_inputs.send(MouseInput { mouse_button_input: event.clone() });
  }
}


fn toggle_mouse_grab(
  _mouse: Res<Input<MouseButton>>,
  _key: Res<Input<KeyCode>>,
  mut _cursor_state_next: ResMut<NextState<CursorState>>,
  _cursor_state: Res<State<CursorState>>,
  input_res: Res<InputResource>,
  ui_state: Res<State<UIState>>,
) {
  if !input_res.enabled {
    return;
  }

  if *State::get(&ui_state) != UIState::Default {
    return;
  }

  // if mouse.just_pressed(MouseButton::Left) {
  //   match *State::get(&cursor_state) {
  //     CursorState::None => {
  //       cursor_state_next.set(CursorState::Locked);
  //     },
  //     CursorState::Locked => {}
  //   };
  // }

  // if key.just_pressed(KeyCode::Escape) {
  //   match *State::get(&cursor_state) {
  //     CursorState::None => {
  //       cursor_state_next.set(CursorState::Locked);
  //     },
  //     CursorState::Locked => {
  //       cursor_state_next.set(CursorState::None);
  //     }
  //   };
  // }
}


fn cursor_free(
  mut _windows: Query<&mut Window>,
  mut _move_setting_res: ResMut<MovementSettings>,
) {
  // let mut window = windows.single_mut();
  // window.cursor.visible = true;
  // window.cursor.grab_mode = CursorGrabMode::None;

  // move_setting_res.sensitivity = 0.0;
  // move_setting_res.speed = 0.0;
}

fn cursor_locked(
  mut _windows: Query<&mut Window>,
  mut _move_setting_res: ResMut<MovementSettings>,
) {
  // let mut window = windows.single_mut();
  // window.cursor.visible = false;
  // window.cursor.grab_mode = CursorGrabMode::Locked;

  // move_setting_res.sensitivity = 0.00012;
  // move_setting_res.speed = 24.0;
  // move_setting_res.speed = 6.0;
}

