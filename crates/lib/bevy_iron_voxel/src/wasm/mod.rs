use bevy::prelude::*;
use bevy_flycam::MovementSettings;
use web_sys::HtmlElement;
use flume::*;
use wasm_bindgen::prelude::*;
use crate::{input::MouseInput, data::{CursorState, UIState}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_event::<PointerLockEvent>()
      .add_event::<MouseMoveEvent>()
      .add_event::<WasmInputEvent>()
      .add_systems(Update, update_fullscreen)
      .add_systems(Update, grab_mouse)
      .add_systems(OnEnter(CursorState::None), cursor_free)
      .add_systems(OnEnter(CursorState::Locked), cursor_locked)
      ;

    app
      .add_systems(Startup, startup)
      .add_systems(Update, send_mouse_events);
  }
}

fn startup(local_res: Res<LocalResource>,) {
  let send_mouse_click = local_res.send_mouse_click.clone();
  let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    let _ = send_mouse_click.send(event.button());
  }) as Box<dyn FnMut(web_sys::MouseEvent)>);

  let window = web_sys::window().expect("no global `window` exists");
  window.set_onmousedown(Some(cb.as_ref().unchecked_ref()));
  cb.forget();

  let send_error = local_res.send_error.clone();
  let cb1 = Closure::wrap(Box::new(move |event: web_sys::ErrorEvent| {
    // event.message()
    let _ = send_error.send(event.message());
  }) as Box<dyn FnMut(web_sys::ErrorEvent)>);
  window.set_onerror(Some(cb1.as_ref().unchecked_ref()));
  cb1.forget();

  // let send_key = local_res.send_key.clone();
  // let cb = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
  //   let _ = send_key.send(event.char_code());
  // }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

  // window.set_onkeydown(Some(cb.as_ref().unchecked_ref()));
  // cb.forget();
}

fn send_mouse_events(
  local_res: Res<LocalResource>,
  // mut wasm_events: EventWriter<WasmInputEvent>,
  mut _mouse_inputs: EventWriter<MouseInput>,
) {
  for _e in local_res.recv_mouse_click.drain() {
    // info!("clicked {}", is_pointer_locked());
    if !is_pointer_locked() {
      continue;
    }
    
    // Defer: Improve getting mouse events from WASM
    // if e == 0 {
    //   mouse_inputs.send(MouseInput { mouse_button_input: MouseButtonInput {
    //     button: MouseButton::Left,
    //     state: ButtonState::Pressed,
    //   }});
    // }
    // if e == 2 {
    //   mouse_inputs.send(MouseInput { mouse_button_input: MouseButtonInput {
    //     button: MouseButton::Right,
    //     state: ButtonState::Pressed,
    //   }});
    // }
  }
}


fn update_fullscreen(
  _input: Res<Input<KeyCode>>,
) {
  // if input.just_pressed(KeyCode::F) {
  //   let _ = html_body().request_fullscreen();
  //   html_body().request_pointer_lock();
  // }
}

fn grab_mouse(
  _mouse: Res<Input<MouseButton>>,
  mut _cursor_state_next: ResMut<NextState<CursorState>>,
  _ui_state: Res<State<UIState>>,
) {
  // if mouse.just_pressed(MouseButton::Left) {
  //   match ui_state.0 {
  //     UIState::Inventory => { },
  //     UIState::Default => { cursor_state_next.set(CursorState::Locked); },
  //     _ => {  }
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
  // window.cursor.grab_mode = CursorGrabMode::Confined;

  // move_setting_res.sensitivity = 0.00012;
  // move_setting_res.speed = 6.0;
}

#[allow(dead_code)]
pub fn html_body() -> HtmlElement {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  body
}

#[allow(dead_code)]
#[derive(Resource)]
struct LocalResource {
  send_mouse_click: Sender<i16>,
  recv_mouse_click: Receiver<i16>,
  send_error: Sender<String>,
  recv_error: Receiver<String>,
  prev_pointer_locked_val: bool,
  pending_to_lock: bool,
}

impl Default for LocalResource {
  fn default() -> Self {
    let (send_mouse_click, recv_mouse_click) = flume::bounded(10);
    let (send_error, recv_error) = flume::bounded(10);
    Self {
      send_mouse_click: send_mouse_click,
      recv_mouse_click: recv_mouse_click,
      send_error: send_error,
      recv_error: recv_error,
      prev_pointer_locked_val: false,
      pending_to_lock: false,
    }
  }
}

#[derive(Event)]
pub struct PointerLockEvent(pub bool);

#[derive(Event)]
pub struct MouseMoveEvent(bool);

#[derive(Event)]
pub struct WasmInputEvent {
  pub mouse: MouseButton,
}


pub fn is_pointer_locked() -> bool {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  

  let lock_op = document.pointer_lock_element();
  lock_op.is_some()
}
