use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Color32, Frame, Vec2, Button}, EguiContexts};
use bevy_egui::egui::Rect;
use crate::data::{CursorState, GameState, GameResource, Data, UIState, UIResource};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(UIMenuResource::default());

    app
      .add_systems(Update, toggle_show)
      .add_systems(Update, render.run_if(in_state(UIState::Menu)));
  }
}


fn toggle_show(
  _key: Res<Input<KeyCode>>,
  mut _cursor_state_next: ResMut<NextState<CursorState>>,
  mut _ui_state_next: ResMut<NextState<UIState>>,
  _ui_state: Res<State<UIState>>,
) {
  // if key.just_pressed(KeyCode::ControlLeft) {
  //   match *State::get(&ui_state) {
  //     UIState::Default => {
  //       ui_state_next.set(UIState::Menu);
  //       cursor_state_next.set(CursorState::None);
  //     },
  //     UIState::Inventory |
  //     UIState::Menu => {
  //       ui_state_next.set(UIState::Default);
  //       cursor_state_next.set(CursorState::Locked);
  //     },
  //     _ => ()
  //   }
    
  // }
}


fn render(
  mut commands: Commands,
  mut contexts: EguiContexts,
  windows: Query<(Entity, &Window), With<PrimaryWindow>>,
  mut _ui_res: ResMut<UIResource>,
  _state: Res<State<UIState>>,
  mut next_state: ResMut<NextState<UIState>>,
  mut next_game_state: ResMut<NextState<GameState>>,
  _local_res: Res<UIMenuResource>,

  mut game_res: ResMut<GameResource>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }
  // info!("Test");
  let (entity, window) = res.unwrap();
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 255),
    ..Default::default()
  };

  let size = [200.0, 300.0];
  let x = (window.width() * 0.5) - size[0] * 0.5;
  let y = window.height() * 0.1;
  let button_size = Vec2::new(125.0, 50.0);

  egui::Window::new("menu")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: [x, y].into(),
      max: [x + size[0], y + size[1]].into(),
    })
    .show(contexts.ctx_mut(), |ui| {
      ui.set_min_size(size.into());

      ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        let back_to_game = Button::new("Back to Game")
          .min_size(button_size);
        if ui.add(back_to_game).clicked() {
          info!("Back to game");
          next_state.set(UIState::Default);
        }

        ui.add_space(20.0);
        let new = Button::new("New")
          .min_size(button_size);
        if ui.add(new).clicked() {
          // next_state.set(UIState::New);
          game_res.data = Data::default();
          next_game_state.set(GameState::New);
        }

        ui.add_space(20.0);
        let load = Button::new("Load")
          .min_size(button_size);
        if ui.add(load).clicked() {
          next_game_state.set(GameState::LoadGame);
        }

        ui.add_space(20.0);
        let save = Button::new("Save")
          .min_size(button_size);
        if ui.add(save).clicked() {
          next_game_state.set(GameState::SaveGame);
        }

        ui.add_space(20.0);
        let quit = Button::new("Quit")
          .min_size(button_size);
        if ui.add(quit).clicked() {
          commands.entity(entity).despawn();
        }
      });
    });

}

#[derive(Resource)]
pub struct UIMenuResource {
}

impl Default for UIMenuResource {
  fn default() -> Self {
    Self {
    }
  }
}