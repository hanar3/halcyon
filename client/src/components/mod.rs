pub mod keyboard_control;

pub enum ComponentId {
  KeyboardControl,
  Color,
  Sprite,
}
pub enum Components {
  KeyboardControl(keyboard_control::KeyboardControl),
}