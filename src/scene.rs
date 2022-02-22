use crate::ControllerEvent;

pub trait Scene {
    fn handle_input(&mut self, event: ControllerEvent) -> Option<Box<dyn Scene>>;
    fn render(&self);
}
