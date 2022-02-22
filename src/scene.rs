use crate::ControllerEvent;

pub trait Scene {
    fn handle_input(&mut self, event: ControllerEvent);
    fn render(&self) -> Option<Box<dyn Scene>>;
}
