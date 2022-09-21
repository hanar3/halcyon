use std::vec::{Vec};
use sdl2::{event::Event, EventPump, keyboard::Keycode};

use crate::entity::{Player, Renderable};

pub struct World {
  players: Vec<Player>,
}

impl World {
  pub fn new() -> World {
    World { players: Vec::new() }
  }

  pub fn add_player(&mut self, player: Player) {
    self.players.push(player);
  }

  pub fn process_event(&mut self, event: &Event) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
          self.players[0].rect.y += 1;
        },
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
          self.players[0].rect.y -= 1;
        },
        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
          self.players[0].rect.x -= 1;
        },
        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
          self.players[0].rect.x += 1;
        },
        _ => {}
    }
  }
}

impl Renderable for World {
  fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) {
    for player in &mut self.players.iter_mut() {
      player.render(canvas); 
    }
  }
}