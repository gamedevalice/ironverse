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
    mod tests;
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
      app
        .add_plugin(BevyVoxelPlugin)
        .insert_resource(BevyVoxelResource::new(
          4, 
          0.5, 
          1, 
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
          ]
        ))
        .add_plugin(data::CustomPlugin)
        // .add_plugin(physics::CustomPlugin)
        .add_plugin(input::CustomPlugin)
        .add_plugin(components::CustomPlugin)
        .add_plugin(graphics::CustomPlugin)
        .add_plugin(states::CustomPlugin)
        .add_plugin(obj::CustomPlugin);
    }
  }


  cfg_if! {
    if #[cfg(feature = "player")] {
      use bevy_flycam::NoCameraAndGrabPlugin;
      app
        .add_plugin(NoCameraAndGrabPlugin)
        .add_plugin(components::camera::CustomPlugin)
        .add_plugin(components::player::CustomPlugin);
    }
  }
  
  cfg_if! {
    if #[cfg(feature = "chunk")] {
      app
        .add_plugin(components::chunk::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(feature = "graphics_low")] {
      app
        .add_plugin(graphics_low::chunks::CustomPlugin)
        .add_plugin(graphics_low::chunk_preview::CustomPlugin)
        ;
    }
  }

  cfg_if! {
    if #[cfg(feature = "graphics_normal")] {
      app
        .add_plugin(graphics_normal::chunks::CustomPlugin)
        .add_plugin(graphics_normal::chunk_preview::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(feature = "ui")] {
      app
        .add_plugin(ui::CustomPlugin)
        .add_plugin(debugger::CustomPlugin)
        ;
    }
  }

  cfg_if! {
    if #[cfg(all(not(feature = "tests"), target_arch = "wasm32"))] {
      app
        .add_plugin(wasm::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(all(feature = "ui_prompt", target_arch = "wasm32") )] {
      app
        .add_plugin(wasm_ui::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(all(not(feature = "tests"), not(target_arch = "wasm32") ))] {
      app
        .add_plugin(native::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(all(feature = "ui_prompt", not(target_arch = "wasm32") ))] {
      app
        .add_plugin(native_ui::CustomPlugin);
    }
  }

  cfg_if! {
    if #[cfg(feature = "tests")] {
      app
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(tests::ChunkPlugin);
    }
  }
  
  app.run();
}
