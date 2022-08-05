use crate::scene::scene::{Scene, NullScene};

pub struct SceneHandler {
    pub opened_scene: Box<dyn Scene>
}

impl SceneHandler {
    pub fn open_scene(&mut self,scene: Box<dyn Scene>) {
        self.opened_scene = scene;
    }

    pub fn new() -> SceneHandler {
        return SceneHandler { opened_scene: Box::new(NullScene::new()) };
    }
}