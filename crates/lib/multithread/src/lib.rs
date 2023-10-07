#![feature(async_closure)]

use std::{future::Future, task::{Context, Poll}};
use plugin::Key;
use wasm_mt_pool::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_mt::utils::{console_ln, fetch_as_arraybuffer};
use voxels::{chunk::chunk_manager::*, data::{voxel_octree::{MeshData, VoxelMode}, surface_nets::VoxelReuse}};
use flume::{Sender, Receiver};
use web_sys::{CustomEvent, HtmlInputElement, CustomEventInit};


use std::sync::RwLock;
static COLORS: RwLock<Vec<[f32; 3]>> = RwLock::new(Vec::new());

pub mod plugin;

#[wasm_bindgen]
pub fn app() {
  // let (send_queue, recv_queue) = flume::unbounded();
  // let (send_chunk, recv_chunk) = flume::unbounded();
  // recv_key_from_wasm(send_queue);
  // recv_chunk_from_wasm(send_chunk);

  for _ in 0..500 {
    COLORS.write().unwrap().push([0.0, 0.0, 0.0]);
  }

  let (send, recv) = flume::unbounded();
  recv_data_key_from_wasm(send.clone());
  recv_data_chunk_from_wasm(send.clone());
  recv_colors_from_wasm();

  spawn_local(async move {
    let ab_js = fetch_as_arraybuffer("./wasm/multithread/multithread.js").await.unwrap();
    let ab_wasm = fetch_as_arraybuffer("./wasm/multithread/multithread_bg.wasm").await.unwrap();
    let window = web_sys::window().expect("no global `window` exists");
    let max_threads = window.navigator().hardware_concurrency() as usize;

    let document = window.document().expect("should have a document on window");
    let e = document.get_element_by_id("concurrency").unwrap();
    let input = e.dyn_into::<HtmlInputElement>().unwrap();
    let threads = input.value().parse::<usize>().unwrap();
    console_ln!("max threads {} current threads {}", max_threads, threads);
    let pool = ThreadPool::new_with_arraybuffers(threads, ab_js, ab_wasm)
      .and_init().await.unwrap();

    // load_data(&pool, recv_queue, recv_chunk).await;
    load_data_from_wasm(&pool, recv).await;
  });
}

fn recv_data_key_from_wasm(send: Sender<WasmMessage>) {
  let callback = Closure::wrap(Box::new(move |event: CustomEvent | {
    let data = event.detail().as_string().unwrap();
    let bytes = array_bytes::hex2bytes(data).unwrap();
    let key: Key = bincode::deserialize(&bytes).unwrap();

    let msg = WasmMessage {
      key: Some(key),
      ..Default::default()
    };

    let _ = send.send(msg);

    // console_ln!("from_wasm_key {:?}", key);
  }) as Box<dyn FnMut(CustomEvent)>);

  let window = web_sys::window().unwrap();
  let _ = window.add_event_listener_with_callback(
    &EventType::KeySend.to_string(),
    callback.as_ref().unchecked_ref()
  );

  callback.forget();
}

fn recv_data_chunk_from_wasm(send: Sender<WasmMessage>) {
  let callback = Closure::wrap(Box::new(move |event: CustomEvent | {
    let data = event.detail().as_string().unwrap();
    let bytes = array_bytes::hex2bytes(data).unwrap();
    let chunk: Chunk = bincode::deserialize(&bytes).unwrap();

    // console_ln!("from wasm chunk {:?}", chunk.key);
    let msg = WasmMessage {
      chunk: Some(chunk),
      ..Default::default()
    };

    let _ = send.send(msg);
  }) as Box<dyn FnMut(CustomEvent)>);

  let window = web_sys::window().unwrap();
  let _ = window.add_event_listener_with_callback(
    &EventType::ChunkSend.to_string(),
    callback.as_ref().unchecked_ref()
  );

  callback.forget();
}

fn recv_colors_from_wasm() {
  let callback = Closure::wrap(Box::new(move |event: CustomEvent | {
    let data = event.detail().as_string().unwrap();
    let bytes = array_bytes::hex2bytes(data).unwrap();
    let colors: Vec<[f32; 3]> = bincode::deserialize(&bytes).unwrap();

    COLORS.write().unwrap().clear();
    COLORS.write().unwrap().append(&mut colors.clone());
  }) as Box<dyn FnMut(CustomEvent)>);

  let window = web_sys::window().unwrap();
  let _ = window.add_event_listener_with_callback(
    &EventType::SendColors.to_string(),
    callback.as_ref().unchecked_ref()
  );

  callback.forget();
}

async fn load_data_from_wasm(
  pool: &ThreadPool,
  recv: Receiver<WasmMessage>
) {

  
  console_ln!("COLORS.len() 1 {}", COLORS.read().unwrap().len());

  while let Ok(msg) = recv.recv_async().await {
    // console_ln!("load_data_from_wasm {:?}", );

    if msg.key.is_some() {
      let key = msg.key.unwrap();
      // console_ln!("load_data {:?}", key);

      let cb = move |result: Result<JsValue, JsValue>| {
        let r = result.unwrap();
        let ab = r.dyn_ref::<js_sys::ArrayBuffer>().unwrap();
        let vec = js_sys::Uint8Array::new(ab).to_vec();
        let str = array_bytes::bytes2hex("", vec);
  
        let e = CustomEvent::new_with_event_init_dict(
          &EventType::KeyRecv.to_string(), CustomEventInit::new().detail(&JsValue::from_str(&str))
        ).unwrap();
  
        let window = web_sys::window().unwrap();
        let _ = window.dispatch_event(&e);
      };
    
      pool_exec!(pool, move || {
        let chunk = compute_chunk(key);
        let encoded: Vec<u8> = bincode::serialize(&chunk).unwrap();
        Ok(wasm_mt::utils::u8arr_from_vec(&encoded).buffer().into())
      }, cb);
    }

    if msg.chunk.is_some() {
      let chunk = msg.chunk.unwrap();
      let c = chunk.clone();

      let colors = COLORS.read().unwrap().clone();
      // console_ln!("load_chunk {:?}", chunk.clone().key);

      let cb = move |result: Result<JsValue, JsValue>| {
        if result.is_err() {
          let err = result.clone().err().unwrap();
          console_ln!("Error {:?} {:?}", c.key, err);
        }
        let r = result.unwrap();
        let ab = r.dyn_ref::<js_sys::ArrayBuffer>().unwrap();
        let vec = js_sys::Uint8Array::new(ab).to_vec();
        let str = array_bytes::bytes2hex("", vec);
  
        
        let e = CustomEvent::new_with_event_init_dict(
          &EventType::ChunkRecv.to_string(), CustomEventInit::new().detail(&JsValue::from_str(&str))
        ).unwrap();
  
        let window = web_sys::window().unwrap();
        let _ = window.dispatch_event(&e);
      };
  
      pool_exec!(pool, move || {
        let mesh = compute_mesh(chunk, &colors);

        let r = bincode::serialize(&mesh);
        if r.is_err() {
          console_ln!("Error encoding");
        }
        // let encoded: Vec<u8> = bincode::serialize(&mesh).unwrap();
  
        Ok(wasm_mt::utils::u8arr_from_vec(&r.unwrap()).buffer().into())
      }, cb);
    }

  }

}

fn compute_chunk(key: Key) -> Chunk {
  let manager = ChunkManager::default();
  ChunkManager::new_chunk(&key.key, 4, key.lod, manager.noise)
}

fn compute_mesh(chunk: Chunk, colors: &Vec<[f32; 3]>) -> MeshData {
  // let mut v = Vec::new();
  // for _ in 0..500 {
  //   v.push([0.0, 0.0, 0.0]);
  // }

  // console_ln!("COLORS.len() {}", COLORS.read().unwrap().len());
  // if COLORS.read().unwrap().len() == 0 {
  //   console_ln!("Testing");
  //   // for _ in 0..500 {
  //   //   COLORS.write().unwrap().push([0.0, 0.0, 0.0]);
  //   // }  
  // }

  // console_ln!("colors.len() {}", colors.len());

  chunk.octree.compute_mesh(
    VoxelMode::SurfaceNets, 
    &mut VoxelReuse::default(), 
    colors,
    // &v, 
    // &COLORS.read().unwrap(),
    1.0, 
    chunk.key,
    chunk.lod
  )
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum EventType {
  KeySend,
  KeyRecv,
  ChunkSend,
  ChunkRecv,
  SendColors,
}

impl ToString for EventType {
  fn to_string(&self) -> String {
    match self {
      EventType::KeySend => String::from("KeySend"),
      EventType::KeyRecv => String::from("KeyRecv"),
      EventType::ChunkSend => String::from("ChunkSend"),
      EventType::ChunkRecv => String::from("ChunkRecv"),
      EventType::SendColors => String::from("SendColors"),
    }
  }
}

struct ChannelFuture {
  // unit: ChannerRef,
  recv: Receiver<[i64; 3]>,
  recv_chunk: Receiver<Chunk>,
}

use std::pin::Pin;
impl Future for ChannelFuture {
  type Output = Result<Res, String>;
  fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
    // let recv = self.unit.borrow().recv.clone();
    // let recv_chunk = self.unit.borrow().recv_chunk.clone();

    // console_ln!("Testing");

    let recv = self.recv.clone();
    let recv_chunk = self.recv_chunk.clone();
    
    let mut res = Res::default();
    for key in recv.drain() {
      res.keys.push(key);
    }
    for chunk in recv_chunk.drain() {
      res.chunks.push(chunk);
    }

    // let mut m = self.unit.borrow_mut();
    // m.recv = self.recv.clone();
    // m.recv_chunk = self.recv_chunk.clone();

    if !res.keys.is_empty() || !res.chunks.is_empty() {
      return Poll::Ready(Ok(res));
    }

    Poll::Pending
  }
}

// async fn load_res(unit: ChannerRef) -> Result<Res, String> {
//   ChannelFuture {
//     unit,
//   };
// }

#[derive(Default)]
struct Res {
  keys: Vec<[i64; 3]>,
  chunks: Vec<Chunk>,
}

#[derive(Default)]
struct WasmMessage {
  key: Option<Key>,
  chunk: Option<Chunk>,
}
