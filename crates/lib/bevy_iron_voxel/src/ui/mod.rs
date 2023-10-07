use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self}, EguiPlugin, EguiContexts};

pub mod hotbar;
pub mod inventory;
pub mod menu;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(EguiPlugin)
      //.add_plugins(hotbar::CustomPlugin)
      .add_plugins(inventory::CustomPlugin)
      .add_plugins(menu::CustomPlugin)
      .add_systems(Update, crosshair);
  }
}


fn crosshair(
  mut _ctx: EguiContexts,
  mut _is_initialized: Local<bool>,
  mut _texture_id: Local<egui::TextureId>,
  _images: Local<Images>,

  _windows: Query<&Window, With<PrimaryWindow>>,
) {
  // let res = windows.get_single();
  // if res.is_err() {
  //   return;
  // }
  // let window = res.unwrap();

  // if !*is_initialized {
  //   *is_initialized = true;
  //   *texture_id = ctx.add_image(images.crosshair.clone_weak());
  // }

  // let frame = Frame {
  //   fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
  //   ..Default::default()
  // };

  // let size = [50.0, 50.0];
  // let x = (window.width() * 0.5) - size[0] * 0.5;
  // let y = (window.height() * 0.5) - size[1] * 0.5;

  // egui::Window::new("crosshair")
  //   .title_bar(false)
  //   .frame(frame)
  //   .fixed_rect(Rect {
  //     min: [x, y].into(),
  //     max: [x, y].into(),
  //   })
  //   .show(ctx.ctx_mut(), |ui| {
  //     ui.image(*texture_id, size.clone());
  //   });
}

struct Images {
  // crosshair: Handle<Image>,
}

impl FromWorld for Images {
  fn from_world(_world: &mut World) -> Self {
    // let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
    Self {
      // crosshair: asset_server.load("crosshair.png"),
    }
  }
}


