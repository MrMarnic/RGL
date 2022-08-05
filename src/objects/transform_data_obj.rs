use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::transform::Transform;

pub struct TransformDataObj {
    pub offset: u32,
    pub transform:Transform,
    pub update: bool
}

impl TransformDataObj {
    pub fn new(transform:Transform,engine:&mut GameEngine) -> TransformDataObj {
        let offset = engine.static_offset_handler.get_offset();

        return TransformDataObj {
            transform,
            offset: offset as u32,
            update: false
        }
    }

    pub fn write(&self,engine:&GameEngine, camera:&Camera) {
        engine.queue.write_buffer(&camera.buffers[2], self.offset as u64, &crate::objects::matrix_helper::get_bytes(&self.transform.matrix));
    }
}