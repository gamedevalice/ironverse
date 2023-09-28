mod save;
mod load;

use bevy::prelude::*;
use web_sys::HtmlElement;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(save::CustomPlugin)
      .add_plugin(load::CustomPlugin);
  }
}

pub fn html_body() -> HtmlElement {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  body
}