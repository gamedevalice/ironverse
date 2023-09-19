use bevy::{prelude::*, input::mouse::MouseButtonInput};

pub mod hotbar;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(InputResource::default())
      .add_event::<MouseInput>()
      .add_plugins(hotbar::CustomPlugin);
      
  }
}

#[derive(Event)]
pub struct MouseInput {
  pub mouse_button_input: MouseButtonInput,
}

#[derive(Resource)]
pub struct InputResource {
  pub enabled: bool,
}

impl Default for InputResource {
  fn default() -> Self {
    Self {
      enabled: true,
    }
  }
}