use std::any::Any;
use wgpu::{CommandEncoder, TextureView};
use winit::dpi::PhysicalSize;
use crate::engine::game_engine::GameEngine;

pub trait Scene {
    fn loaded(&mut self,engine:&mut GameEngine) {}
    fn process_input(&mut self,engine:&mut GameEngine) {}
    fn update(&mut self,engine:&mut GameEngine);
    fn render(&self,engine:&GameEngine,encoder:&mut CommandEncoder,view:&TextureView);
    fn window_resized(&mut self,engine:&GameEngine, size:&PhysicalSize<u32>);
    fn handle_second(&mut self,engine:&mut GameEngine) {}
    fn close(&mut self,engine:&mut GameEngine){}
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct NullScene {

}

impl NullScene {
    pub fn new() -> NullScene {
        return NullScene{};
    }
}

impl Scene for NullScene {
    fn loaded(&mut self, engine: &mut GameEngine) {
        
    }

    fn process_input(&mut self, engine: &mut GameEngine) {
        
    }

    fn update(&mut self, engine: &mut GameEngine) {
        
    }

    fn render(&self, engine: &GameEngine, encoder: &mut CommandEncoder, view: &TextureView){
    }

    fn window_resized(&mut self, engine: &GameEngine, size: &PhysicalSize<u32>) {
        
    }

    fn handle_second(&mut self, engine: &mut GameEngine) {
        
    }

    fn close(&mut self, engine: &mut GameEngine) {
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}