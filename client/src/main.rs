pub mod world;
pub mod entity;
use entity::Renderable;
use sdl2::rect::{Point};
use sdl2::pixels::{Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use world::World;
use std::time::Duration;


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
    
    world.add_player(entity::Player::new(1, Color::RGB(255, 0, 0), Point::new(40, 40)));
    world.add_player(entity::Player::new(1, Color::RGB(0, 255, 0), Point::new(120, 40)));

    'running: loop {
        canvas.clear();


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {
                    world.process_event(&event);
                }
            }
        }

        world.render(&mut canvas);

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}