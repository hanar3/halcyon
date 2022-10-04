use lazy_static::lazy_static;
use sdl2::render::Canvas;
use std::option::Option;
use std::sync::RwLock;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::traits::Entity;
use crate::world::World;
use crate::{components, entities};

pub struct Application {
    sdl_context: Sdl,
    window: Canvas<Window>,
    game_canvas: World,
}

impl Application {
    pub fn new() -> Application {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let mut game_canvas = World::new();
        let mut main_actor =
            entities::test_entity::Rectangle::new(1, Color::RGB(255, 0, 0), Point::new(40, 40));
        main_actor.add_component(
            "transform".to_string(),
            components::Component::Transform(components::transform::Transform::new(40, 150)),
        );
        main_actor.add_component(
            "keyboard_control".to_string(),
            components::Component::KeyboardControl(
                components::keyboard_control::KeyboardControl::new("transform".to_string()),
            ),
        );
        game_canvas.add_entity("main_actor".to_string(), Box::new(main_actor));

        Application {
            sdl_context: sdl_context,
            window: canvas,
            game_canvas: game_canvas,
        }
    }

    pub fn run(&mut self) {
        self.window.clear();
        self.game_canvas.render(&mut self.window);

        self.window.set_draw_color(Color::RGB(0, 0, 0));
        self.window.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn process_input(&self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let event = event_pump.poll_event();
        let mut new_event = CURRENT_EVENT.write().unwrap();
        *new_event = event;
    }
}

lazy_static! {
    pub static ref CURRENT_EVENT: RwLock<Option<Event>> = RwLock::new(Option::None);
}
