use std::{collections::HashMap, string, ops::DerefMut};
use lazy_static::__Deref;
use sdl2::{event::Event, render::WindowCanvas, keyboard::Keycode};

use crate::{traits::{Entity}};

pub struct World {
  entities_map: HashMap<String, Box<dyn Entity>>
}

impl World {
  pub fn new() -> World {
    World { entities_map: HashMap::new() }
  }

  pub fn add_entity(&mut self, id: String, ent: Box<dyn Entity>) {
    self.entities_map.insert(id, ent);
  }

  pub fn get_entity(&mut self, id: String) -> &mut Box<dyn Entity> {
     self.entities_map.get_mut(&id).unwrap_or_else(|| panic!("{} is not a valid entity", id))
  }

  pub fn render(&mut self, canvas: &mut WindowCanvas) {
    for (_key, entity) in self.entities_map.iter_mut() {
      entity.update();
      entity.render(canvas);
    }
  }


  pub fn process_event(&mut self, event: &Event) {

    match event {
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {

        },
        _ => {}
    }
  }
}
