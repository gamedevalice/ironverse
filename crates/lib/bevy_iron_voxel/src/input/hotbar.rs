use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};
use bevy_voxel::Preview;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(HotbarResource::default())
      .add_systems(Update, update);
  }
}

fn update(
  mut hotbar_res: ResMut<HotbarResource>,
  mut key_events: EventReader<KeyboardInput>,
  mut previews: Query<&mut Preview>,
) {
  for event in key_events.iter() {
    if event.state == ButtonState::Pressed && event.key_code.is_some() {
      let key_code = event.key_code.unwrap();

      for i in 0..hotbar_res.bars.len() {
        let bar = &hotbar_res.bars[i];

        let voxel = bar.voxel;
        if bar.key_code == key_code {
          hotbar_res.selected_keycode = key_code;

          for mut preview in &mut previews {
            preview.voxel = voxel;
          }
        }
      }
      
    }
  }
}



#[derive(Resource)]
pub struct HotbarResource {
  pub bars: Vec<Bar>,
  pub selected_keycode: KeyCode,
}

impl Default for HotbarResource {
  fn default() -> Self {
    Self {
      bars: vec![
        Bar::new(KeyCode::Key1, 1),
        Bar::new(KeyCode::Key2, 2),
        Bar::new(KeyCode::Key3, 3),
        Bar::new(KeyCode::Key4, 4),
        Bar::new(KeyCode::Key5, 5),
        Bar::new(KeyCode::Key6, 6),
        Bar::new(KeyCode::Key7, 7),
        Bar::new(KeyCode::Key8, 8),
        Bar::new(KeyCode::Key9, 9),
        Bar::new(KeyCode::Key0, 10),
      ],
      selected_keycode: KeyCode::Key1,
    }
  }
}

pub struct Bar {
  pub key_code: KeyCode,
  pub voxel: u8,
}

impl Bar {
  pub fn new(k: KeyCode, v: u8) -> Self {
    Bar { key_code: k, voxel: v }
  }
}
