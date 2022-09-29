use std::{collections::HashMap, string, ops::DerefMut};
use sdl2::{event::Event, render::WindowCanvas, keyboard::Keycode};

use crate::{traits::{Entity}, system::event_queue::EventQueue};

pub struct World {
  entities_map: HashMap<&'static str, Box<dyn Entity>>
}

impl World {
  pub fn new() -> World {
    World { entities_map: HashMap::new() }
  }

  pub fn add_entity(&mut self, ent: Box<dyn Entity>) {
    self.entities_map.insert(stringify!(ent.as_ref()), ent);
  }


  pub fn render(&self, canvas: &mut WindowCanvas) {
    for (_key, entity) in self.entities_map.iter() {
      entity.as_ref().render(canvas);
    }
  }
  
  // pub fn get_entity(&self) -> Option<&Box<dyn Entity>> {
  //   self.entities_map.get(label)
  // }

  pub fn process_event(&mut self, event: &Event) {

    match event {
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {

        },
        _ => {}
    }
  }
}
