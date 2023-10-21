use std::rc::Rc;
use wgpu::RenderPass;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::obj_model::ObjModel;
use crate::objects::transform::Transform;

pub struct ModelInstance {
    pub model: Rc<ObjModel>,
    pub transform: Transform,
    pub offset: u32
}

impl ModelInstance {

    pub fn new(transform:Transform, model:Rc<ObjModel>) -> Self {
        Self { model, transform, offset: 0 }
    }

    pub fn init(&mut self, engine:&mut GameEngine) {
        self.offset = engine.static_offset_handler.get_offset() as u32;
    }
    pub fn update(&mut self, engine:&mut GameEngine, camera:&Camera) {
        engine.queue.write_buffer(&camera.buffers[2],self.offset as u64,&*crate::objects::matrix_helper::get_bytes(&self.transform.matrix));
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera, engine:&'a GameEngine) {
        self.model.render(render_pass,camera,self.offset,engine);
    }
}