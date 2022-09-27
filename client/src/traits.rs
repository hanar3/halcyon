use sdl2::render::{WindowCanvas};

pub trait Entity {
  fn render(&self, _canvas: &mut WindowCanvas) {}
}