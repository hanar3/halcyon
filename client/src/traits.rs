use sdl2::render::{WindowCanvas};
use std::any::{Any};

use crate::components::Components;

pub trait Entity {
  fn render(&self, _canvas: &mut WindowCanvas) {}
  fn add_component(&mut self, _component: Components) {}
  fn get_component(&mut self) {}
}

pub trait Component {
  fn execute(&self){}
  fn set_owner(&self, entity: &Box<dyn Entity>) {}
}