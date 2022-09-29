use std::{any::Any, sync::Mutex};
use lazy_static::lazy_static;
use super::{event_type::EventType, event_listener::EventListener};

pub struct EventQueue <'a> {
  pub listeners: Vec<EventListener<'a>>,
}

impl<'a> EventQueue<'a> {
  pub fn on(&mut self, event_name: &'a str, handlr: fn(&mut Box<dyn Any>)) {
    self.listeners.push(EventListener { event_type: event_name, handler_fn: handlr })
  }

  pub fn emit<T>(&self, event_name: &'a str, data: &mut Box<dyn Any>) {
    for listener in self.listeners.iter() {
      if listener.event_type == event_name {
        (listener.handler_fn)(data);
      }
    }

  }
}

lazy_static! {  
  pub static ref EVENT_EMITTER: Mutex<EventQueue<'static>> = Mutex::new(EventQueue { listeners: vec![] });
}