#[macro_use]
pub mod world;
pub mod entities;
pub mod traits;
pub mod components;
pub mod app;
use sdl2::event::Event;


pub fn main() {
    let mut  app = app::application::Application::new();
    'main: loop {
        app.run();
        app.process_input();
        let current_event = app::application::CURRENT_EVENT.read().unwrap();
        match &*current_event {
            Some(e) => {
                match e {
                    Event::Quit { .. } => {
                        break 'main;
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }
}