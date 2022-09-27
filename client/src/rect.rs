use sdl2::rect::{Point, Rect};
use sdl2::pixels::{Color};
use sdl2::render::{WindowCanvas};
use crate::traits::{Entity};

pub struct Rectangle {
  pub id: i32,
  pub color: Color,
  pub initial_position: Point,
}

impl Rectangle {
  pub fn new(id: i32, color: Color, initial_position: Point) -> Rectangle {
    Rectangle {
      id: id,
      color: color,
      initial_position: initial_position,
    }
  }
}

impl Entity for Rectangle {
  fn render(&self, canvas: &mut WindowCanvas) {
      canvas.set_draw_color(self.color);
      canvas.draw_rect(Rect::new(self.initial_position.x, self.initial_position.y, 40,  40)).expect("Unable to render");
  }
}