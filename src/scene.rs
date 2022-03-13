use crate::ControllerEvent;

pub trait Scene {
    fn handle_input(&mut self, event: ControllerEvent);
    fn render(&mut self, framecount: u32) -> Option<Box<dyn Scene>>;
}
