pub mod keyboard_control;
pub mod transform;
pub enum ComponentId {
  KeyboardControl,
  Color,
  Sprite,
}
pub enum Component {
  KeyboardControl(keyboard_control::KeyboardControl),
  Transform(transform::Transform)
}