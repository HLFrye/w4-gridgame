pub trait Scene {
    pub fn handle_input(&mut self, event: ControllerEvent) -> Option<Scene>
    pub fn render(&self)
}

pub struct MainGameScene {
    
}