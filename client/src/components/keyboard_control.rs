use crate::traits::Component;
pub struct KeyboardControl {
}

impl Component for KeyboardControl {
  fn execute(&self) {
  }

  fn set_owner(&self, entity: &Box<dyn crate::traits::Entity>) {
      
  }
}
