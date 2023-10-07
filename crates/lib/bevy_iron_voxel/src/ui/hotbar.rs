use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{EguiContexts, egui::{self, TextureId, Frame, Color32, Style, ImageButton, Rect, Vec2, Pos2}};
use crate::{input::hotbar::HotbarResource, data::UIResource};
use super::inventory::InventoryTexture;


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(HotbarUIResource::default())
      .add_systems(Update, render)
      .add_systems(Update, render_items.after(render))
      ;
  }
}


fn render(
  mut ctx: EguiContexts,
  mut is_initialized: Local<bool>,
  mut texture_id: Local<TextureId>,
  images: Local<Images>,

  windows: Query<&Window, With<PrimaryWindow>>,
  hotbar_res: Res<HotbarResource>,
  mut hotbar_ui_res: ResMut<HotbarUIResource>,
  _ui_res: Res<UIResource>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }
  let window = res.unwrap();

  if !*is_initialized {
    *is_initialized = true;
    *texture_id = ctx.add_image(images.block.clone_weak());
  }

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  let size = [50.0, 50.0];
  // let blocks = play_res.key_mapping.len();
  let blocks = 10;
  let x = (window.width() * 0.5) - size[0] * (blocks as f32) * 0.5;
  let y = (window.height() * 1.0) - size[1] * 1.5;

  egui::Window::new("block_ui")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: [x, y].into(),
      max: [x, y].into(),
    })
    .show(ctx.ctx_mut(), |ui| {
      ui.horizontal(|ui| {
        let mut style = Style::default();
        style.spacing.item_spacing = Vec2::new(0.0, 0.0);
        ui.set_style(style);

        for index in 0..hotbar_res.bars.len() {
          let bar = &hotbar_res.bars[index];

          // Background button
          let mut img_button = ImageButton::new(*texture_id, size.clone()).frame(false);
          if hotbar_res.selected_keycode == bar.key_code {
            img_button = img_button.tint(Color32::RED);
          }

          let res = ui.add(img_button);
          let rect = res.rect.clone();
          hotbar_ui_res.pos_bars[index] = rect;
        }
      });
    });
}

/*
  Have to create a new system to render on top of hotbars because ui.put() still modifies
  the arrangement of ui inside the Window which in this case we don't need.
  Once I find a way to disallowing it, this function has to be integrated in render()
 */
fn render_items(
  mut ctx: EguiContexts,
  mut _is_initialized: Local<bool>,
  mut _texture_id: Local<TextureId>,
  _images: Local<Images>,

  windows: Query<&Window, With<PrimaryWindow>>,
  hotbar_res: Res<HotbarResource>,
  hotbar_ui_res: Res<HotbarUIResource>,
  ui_res: Res<UIResource>,
  inventory_texture: Res<InventoryTexture>,
) {
  let res = windows.get_single();
  if res.is_err() || !inventory_texture.is_loaded {
    return;
  }
  let window = res.unwrap();

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  let size = [50.0, 50.0];
  // let blocks = play_res.key_mapping.len();
  let blocks = 10;
  let x = (window.width() * 0.5) - size[0] * (blocks as f32) * 0.5;
  let y = (window.height() * 1.0) - size[1] * 1.5;

  egui::Window::new("hotbar_items")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect {
      min: [x, y].into(),
      max: [x, y].into(),
    })
    .show(ctx.ctx_mut(), |ui| {
      ui.horizontal(|ui| {
        let mut style = Style::default();
        style.spacing.item_spacing = Vec2::new(0.0, 0.0);
        ui.set_style(style);

        for index in 0..hotbar_res.bars.len() {
          let bar = &hotbar_res.bars[index];
          let rect = hotbar_ui_res.pos_bars[index].clone();

          if bar.voxel > 0 {
            // Item image
            let total_items = ui_res.total_materials as f32;
            let div_index = 1.0 / total_items;
            let uv_index = bar.voxel as f32 - 1.0;

            let item_size = [35.0, 35.0];
            let item = egui::Image::new(inventory_texture.albedo_id, item_size.clone()).uv(Rect {
              min: Pos2::new(0.0, div_index * uv_index),
              max: Pos2::new(1.0, div_index * (uv_index + 1.0)),
            });
  
            let adj = Vec2::new(7.0, 7.0);
            let pos = rect.min + adj;
            let item_rect = egui::Rect::from_min_size(pos, item_size.into());
            let item_res = ui.put(item_rect, item);
            if item_res.clicked() {
              info!("Item clicked");
            }
          }
        }
      });
    });
}


struct Images {
  block: Handle<Image>,
}

impl FromWorld for Images {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
    Self {
      block: asset_server.load("block_ui.png"),
    }
  }
}

#[derive(Resource)]
pub struct HotbarUIResource {
  pub pos_bars: [Rect; 10]
}

impl Default for HotbarUIResource {
  fn default() -> Self {
    Self {
      pos_bars: [Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)); 10],
    }
  }
}