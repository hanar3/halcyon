use core::panic;
use std::{vec::{Vec}, collections::HashMap};
use sdl2::{event::Event, keyboard::Keycode, render::WindowCanvas};

use crate::{traits::{Entity}};

pub struct World<'a> {
  entities_map: HashMap<&'a str, Box<dyn Entity>>
}

impl<'a> World<'a> {
  pub fn new() -> World<'a> {
    World { entities_map: HashMap::new() }
  }

  pub fn add_entity(&mut self, label: &'a str, ent: Box<dyn Entity>) {
    self.entities_map.insert(label, ent);
  }


  pub fn render(&self, canvas: &mut WindowCanvas) {
    for (_key, entity) in self.entities_map.iter() {
      entity.as_ref().render(canvas);
    }
  }
  pub fn get_entity(&self, label: &'a str ) -> Option<&Box<dyn Entity>> {
    self.entities_map.get(label)
  }


  pub fn process_event(&mut self, event: &Event) {

    // match event {
    //     Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
    //       self.players[0].rect.y += 1;
    //     },
    //     Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
    //       self.players[0].rect.y -= 1;
    //     },
    //     Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
    //       self.players[0].rect.x -= 1;
    //     },
    //     Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
    //       self.players[0].rect.x += 1;
    //     },
    //     _ => {}
    }
  }


}

