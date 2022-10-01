use std::collections::HashMap;
use std::fs;

use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::{Color};
use sdl2::render::{WindowCanvas};
use sdl2::event::Event;
use rlua::Lua;
use sdl2::sys::KeyCode;

use crate::app::application::CURRENT_EVENT;
use crate::components::Component;

use crate::traits::{Entity};
pub struct Rectangle {
  pub id: i32,
  pub color: Color,
  pub rect: Rect,
  pub components: HashMap<String, Component>,
  lua: Lua
}

impl Rectangle {
  pub fn new(id: i32, color: Color, initial_position: Point) -> Rectangle {
    Rectangle {
      id: id,
      color: color,
      rect: Rect::new(40, 50, 40,  40),
      components: HashMap::new(),
      lua: Lua::new(),
    }
  }
}

impl Entity for Rectangle {
  fn render(&self, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(self.color);
    canvas.draw_rect(self.rect).expect("Unable to render");
  }

  fn update(&mut self) {
    for (_, component) in self.components.iter() {
      match component {
        Component::KeyboardControl(k) => {
          let event = CURRENT_EVENT.read().unwrap();
          match &*event {
            Some(e) => {
              match e {
                Event::KeyDown {  keycode: Some(Keycode::Down), .. } => {
                  println!("event {:?}", self.rect);
                  self.rect.x += 50;
                }
                _ => {}
              }
            }
            None => {}
          }
 

        }
        
        _ => {}
      }
    }
  }

  fn add_component(&mut self, id: String, component: Component) {
      match &component {
          Component::Transform(t) => {
            self.rect.x = t.x;
            self.rect.y = t.y;
          }

          Component::KeyboardControl(k) => {
          }
      }

      self.components.insert(id, component);
  }

  fn get_component(&self, id: String) -> &Component {
    let component = self.components.get(&id).unwrap();
    return component;
  }

  fn attach_script(&mut self, lua_file: String) {
    self.lua.context(|ctx| {
      let source = fs::read_to_string(lua_file).unwrap();
      ctx.load(&source).exec().unwrap();
    })
  }
}