#[macro_use]
pub mod world;
pub mod entities;
pub mod traits;
pub mod components;
pub mod system;

use sdl2::rect::{Point};
use sdl2::pixels::{Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use system::event_queue::{EventQueue, EVENT_EMITTER};
use world::World;
use std::any::Any;
use std::time::Duration;

use crate::system::{event_listener::EventListener, event_type::EventType};


struct KeydownEvent {
    keycode: Keycode,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut world = World::new();
    
    let main_actor = entities::test_entity::Rectangle::new(1, Color::RGB(255, 0, 0), Point::new(40, 40));
    
    
    EVENT_EMITTER.lock().unwrap().on("keydown", |event| {
        let d = event.downcast_ref::<KeydownEvent>().unwrap();
        println!("Keydown event 1! {}", d.keycode);
    });

    world.add_entity(Box::new(main_actor));
    
    'main: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => {
                    let mut boxed  = Box::new(KeydownEvent {
                        keycode: keycode.unwrap(),
                    }) as Box<dyn Any>;

                    EVENT_EMITTER.lock().unwrap().emit("keydown", &mut boxed);
                }

                _ => {

                }
            }
        }

        world.render(&mut canvas);

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}