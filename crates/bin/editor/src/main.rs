use bevy::{prelude::*, window::PresentMode};
use bevy_voxel::{BevyVoxelPlugin, BevyVoxelResource};
use cfg_if::cfg_if;

mod utils;

cfg_if! {
  if #[cfg(feature = "core")] {
    mod input;
    mod components;
    mod states;
    mod graphics;
    mod physics;
    mod data;
    mod obj;
  }
}


cfg_if! {
  if #[cfg(all(not(feature = "tests"), target_arch = "wasm32"))] {
    mod wasm;
  }
}

cfg_if! {
  if #[cfg(all(feature = "ui_prompt", target_arch = "wasm32") )] {
    mod wasm_ui;
  }
}

// mod native;
cfg_if! {
  if #[cfg(all(not(feature = "tests"), not(target_arch = "wasm32") ))] {
    mod native;
  }
}

cfg_if! {
  if #[cfg(all(feature = "ui_prompt", not(target_arch = "wasm32") ))] {
    mod native_ui;
  }
}


cfg_if! {
  if #[cfg(feature = "graphics_low")] {
    mod graphics_low;
  }
}

cfg_if! {
  if #[cfg(feature = "graphics_normal")] {
    mod graphics_normal;
  }
}

cfg_if! {
  if #[cfg(feature = "ui")] {
    mod ui;
    mod debugger;
  }
}

cfg_if! {
  if #[cfg(feature = "tests")] {
    // mod tests;
    use bevy_flycam::NoCameraPlayerPlugin;
  }
}

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Ironverse Editor".into(),
        resolution: (800., 600.).into(),
        present_mode: PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        ..default()
      }),
      ..default()
    }));

  cfg_if! {
    if #[cfg(feature = "core")] {
      let range = 2;
      app
        .add_plugins(BevyVoxelPlugin)
        .insert_resource(BevyVoxelResource::new(
          4, 
          // 0.5,
          1.0, 
          range, 
          vec![
            [1.0, 0.0, 0.0], 
            [0.0, 1.0, 0.0], 
            [0.0, 0.0, 1.0], 
            [0.0, 0.0, 0.0],
      
            [0.2, 0.0, 0.0],
            [0.4, 0.0, 0.0],
            [0.6, 0.0, 0.0],
            [0.8, 0.0, 0.0],
      
            [0.0, 0.2, 0.0],
            [0.0, 0.4, 0.0],
          ],
          vec![0, range as u32, 4, 6, 8],
        ))
        .add_plugins(data::CustomPlugin)
        // .add_plugins(physics::CustomPlugin)
        .add_plugins(input::CustomPlugin)
        .add_plugins(components::CustomPlugin)
        .add_plugins(graphics::CustomPlugin)
        .add_plugins(states::CustomPlugin)
        .add_plugins(obj::CustomPlugin);
    }
  }


  cfg_if! {
    if #[cfg(feature = "player")] {
      use bevy_flycam::NoCameraAndGrabPlugin;
      app
        .add_plugins(NoCameraAndGrabPlugin)
        .add_plugins(components::camera::CustomPlugin)
        .add_plugins(components::player::CustomPlugin);
    }
  }
  cfg_if! {
    if #[cfg(feature = "graphics_low")] {
      app
        .add_plugins(graphics_low::chunks::CustomPlugin)
        .add_plugins(graphics_low::chunk_preview::CustomPlugin)
        ;
    }
  }

  cfg_if! {
    if #[cfg(feature = "graphics_normal")] {
      app
        .add_plugins(graphics_normal::chunks::CustomPlugin)
        .add_plugins(graphics_normal::chunk_preview::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(feature = "ui")] {
      app
        .add_plugins(ui::CustomPlugin)
        .add_plugins(debugger::CustomPlugin)
        ;
    }
  }

  cfg_if! {
    if #[cfg(all(not(feature = "tests"), target_arch = "wasm32"))] {
      app
        .add_plugins(wasm::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(all(feature = "ui_prompt", target_arch = "wasm32") )] {
      app
        .add_plugins(wasm_ui::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(all(not(feature = "tests"), not(target_arch = "wasm32") ))] {
      app
        .add_plugins(native::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(all(feature = "ui_prompt", not(target_arch = "wasm32") ))] {
      app
        .add_plugins(native_ui::CustomPlugin);
    }
  }
/* 
  cfg_if! {
    if #[cfg(feature = "tests")] {
      app
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(tests::ChunkPlugin);
    }
  }
   */
  app.run();
}
