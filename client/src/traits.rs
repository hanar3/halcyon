use sdl2::render::{WindowCanvas};

use crate::components::Component;

pub trait Entity {
  fn render(&self, _canvas: &mut WindowCanvas) {}
  fn add_component(&mut self, id: String, _component: Component);
  fn get_component(&self, id: String) -> &Component;
  fn attach_script(&mut self, _lua_file: String);
  fn update(&mut self);
}