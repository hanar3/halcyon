use sdl2::rect::{Rect, Point};
use sdl2::pixels::{Color};
use sdl2::render::{WindowCanvas};


pub trait Renderable {
  fn render(&mut self, _canvas: &mut WindowCanvas) {}
}

pub struct Player {
  pub id: i32,
  pub color: Color,
  pub rect: Rect,
}

impl Player {
  pub fn new(id: i32, color: Color, initial_position: Point) -> Player {
    Player {
      id: id,
      rect: Rect::new(initial_position.x, initial_position.y, 40, 40),
      color: color,
    }
  }
}

impl Renderable for Player {
  fn render(&mut self, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(self.color);
    canvas.fill_rect(self.rect).expect("Failed to draw rectangle here...");
  }
    
}
