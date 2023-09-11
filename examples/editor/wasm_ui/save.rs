use bevy::{prelude::*, utils::HashMap};
use voxels::chunk::chunk_manager::Chunk;
use crate::components::chunk::Chunks;
use crate::data::{Terrains, Data, Status, GameState, GameResource};
use super::html_body;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(enter.in_schedule(OnEnter(GameState::SaveGame)))
      .add_system(track_modified_chunks)
      .add_system(export_obj)
    ;
  }
}

fn enter(local_res: Res<LocalResource>,) {
  let body = html_body();
  let res = body.query_selector("#download");
  
  let a_ops = match res {
    Ok(ops) => ops,
    Err(e) => { 
      info!("{:?}", e);
      return ()
    }
  };

  if a_ops.is_some() {
    let mut terrains = Terrains { keys: Vec::new(), voxels: Vec::new() };
    for (key, chunk) in local_res.chunks.iter() {
      terrains.keys.push(key.clone());
      terrains.voxels.push(array_bytes::bytes2hex("", &chunk.octree.data));
    }

    let data = Data {
      status: Status {
        position: [0.0, 1.0, 0.0],
      },
      terrains: terrains
    };
    let str = toml::to_string_pretty(&data).unwrap();

    let parts = js_sys::Array::of1(&unsafe {
      js_sys::Uint8Array::view(str.as_bytes())
          .into()
    });
    let blob_res = web_sys::Blob::new_with_u8_array_sequence(&parts);
    if blob_res.is_err() {
      return;
    }
    let blob = blob_res.unwrap();

    let url_res = web_sys::Url::create_object_url_with_blob(&blob);
    if url_res.is_err() {
      return;
    }

    let a = a_ops.unwrap();
    a.set_attribute("download", "save.toml");
    a.set_attribute("href", &url_res.unwrap());
    let a1: HtmlElement = a.dyn_into::<HtmlElement>().unwrap();
    a1.click();
  }
}


fn track_modified_chunks(
  mut chunks_query: Query<&Chunks, Changed<Chunks>>,
  mut local_res: ResMut<LocalResource>,
) {
  for c in &chunks_query {
    for mesh in c.data.iter() {
      if !mesh.chunk.is_default {
        if !mesh.chunk.is_default {
          local_res.chunks.insert(mesh.key.clone(), mesh.chunk.clone());
        }
        
        // info!("chunks.len() {:?}", local_res.chunks.len());
      }
    }
    
  }
}


fn export_obj(
  mut game_res: ResMut<GameResource>,
) {
  if game_res.export_obj.is_none() {
    return;
  }

  let str = game_res.export_obj.take().unwrap();

  let body = html_body();
  let res = body.query_selector("#download");
  
  let a_ops = match res {
    Ok(ops) => ops,
    Err(e) => { 
      info!("{:?}", e);
      return ()
    }
  };

  if a_ops.is_some() {
    let parts = js_sys::Array::of1(&unsafe {
      js_sys::Uint8Array::view(str.as_bytes())
          .into()
    });
    let blob_res = web_sys::Blob::new_with_u8_array_sequence(&parts);
    if blob_res.is_err() {
      return;
    }
    let blob = blob_res.unwrap();

    let url_res = web_sys::Url::create_object_url_with_blob(&blob);
    if url_res.is_err() {
      return;
    }

    let a = a_ops.unwrap();
    a.set_attribute("download", "save.obj");
    a.set_attribute("href", &url_res.unwrap());
    let a1: HtmlElement = a.dyn_into::<HtmlElement>().unwrap();
    a1.click();
  }
}



#[derive(Resource)]
struct LocalResource {
  chunks: HashMap<[i64; 3], Chunk>,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      chunks: HashMap::default(),
    }
  }
}
