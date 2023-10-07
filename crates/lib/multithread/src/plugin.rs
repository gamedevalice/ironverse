use bevy::prelude::*;
use flume;
use flume::{Sender, Receiver};
use voxels::chunk::chunk_manager::Chunk;
use voxels::data::voxel_octree::MeshData;
use web_sys::{CustomEvent, CustomEventInit};
use wasm_bindgen::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PluginResource::default())
      .add_systems(Startup, init);
  }
}

fn init(
  local_res: ResMut<PluginResource>,
) {
  receive_chunk(local_res.send_chunk.clone());
  receive_mesh(local_res.send_mesh.clone());
}


pub fn receive_chunk(send: Sender<Chunk>) {
  let callback = Closure::wrap(Box::new(move |event: CustomEvent | {
    let data = event.detail().as_string().unwrap();
    let bytes = array_bytes::hex2bytes(data).unwrap();
    let chunk: Chunk = bincode::deserialize(&bytes).unwrap();

    let _ = send.send(chunk);
  }) as Box<dyn FnMut(CustomEvent)>);

  let window = web_sys::window().unwrap();
  let _ = window.add_event_listener_with_callback(
    &EventType::KeyRecv.to_string(),
    callback.as_ref().unchecked_ref()
  );

  callback.forget();
}

pub fn receive_mesh(send: Sender<MeshData>) {
  let callback = Closure::wrap(Box::new(move |event: CustomEvent | {
    // info!("receive_mesh()");

    let data = event.detail().as_string().unwrap();
    let bytes = array_bytes::hex2bytes(data).unwrap();
    let mesh: MeshData = bincode::deserialize(&bytes).unwrap();
    let _ = send.send(mesh);
  }) as Box<dyn FnMut(CustomEvent)>);

  let window = web_sys::window().unwrap();
  let _ = window.add_event_listener_with_callback(
    &EventType::ChunkRecv.to_string(),
    callback.as_ref().unchecked_ref()
  );

  callback.forget();
}


pub fn send_key(key: Key) {
  // let k: Vec<[u8; 8]> = key.iter().map(|a| a.to_be_bytes()).collect();
  // let mut bytes = Vec::new();
  // for k1 in k.iter() {
  //   bytes.append(&mut k1.to_vec());
  // }
  // let str = array_bytes::bytes2hex("", &bytes);

  let encoded: Vec<u8> = bincode::serialize(&key).unwrap();
  let str = array_bytes::bytes2hex("", &encoded);

  let e = CustomEvent::new_with_event_init_dict(
    &EventType::KeySend.to_string(), CustomEventInit::new().detail(&JsValue::from_str(&str))
  ).unwrap();

  let window = web_sys::window().unwrap();
  let _ = window.dispatch_event(&e);
}

pub fn send_chunk(chunk: Chunk) {
  let encoded: Vec<u8> = bincode::serialize(&chunk).unwrap();
  let str = array_bytes::bytes2hex("", &encoded);

  let e = CustomEvent::new_with_event_init_dict(
    &EventType::ChunkSend.to_string(), CustomEventInit::new().detail(&JsValue::from_str(&str))
  ).unwrap();

  let window = web_sys::window().unwrap();
  let _ = window.dispatch_event(&e);
}

pub fn send_colors(colors: &Vec<[f32; 3]>) {
  let encoded: Vec<u8> = bincode::serialize(colors).unwrap();
  let str = array_bytes::bytes2hex("", &encoded);

  let e = CustomEvent::new_with_event_init_dict(
    &EventType::SendColors.to_string(), CustomEventInit::new().detail(&JsValue::from_str(&str))
  ).unwrap();

  let window = web_sys::window().unwrap();
  let _ = window.dispatch_event(&e);
}


#[derive(Resource)]
pub struct PluginResource {
  // timer: Timer,
  send_chunk: Sender<Chunk>,
  pub recv_chunk: Receiver<Chunk>,

  send_mesh: Sender<MeshData>,
  pub recv_mesh: Receiver<MeshData>,
}

impl Default for PluginResource {
  fn default() -> Self {
    let (send_chunk, recv_chunk) = flume::unbounded();
    let (send_mesh, recv_mesh) = flume::unbounded();
    Self {
      // timer: Timer::from_seconds(100.0, TimerMode::Repeating),

      send_chunk: send_chunk,
      recv_chunk: recv_chunk,
      send_mesh: send_mesh,
      recv_mesh: recv_mesh,
    }
  }
}


use serde::{Serialize, Deserialize};

use crate::EventType;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Octree {
  pub key: [i64; 3],
  pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Key {
  pub key: [i64; 3],
  pub lod: usize,
}