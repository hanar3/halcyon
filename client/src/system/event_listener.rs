use std::any::Any;
pub struct EventListener<'a> {
  pub event_type: &'a str,
  pub handler_fn: fn(&mut Box<dyn Any>),
}