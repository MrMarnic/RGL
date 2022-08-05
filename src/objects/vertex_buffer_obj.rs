use nalgebra_glm::vec3;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::transform::Transform;
use crate::objects::vertex_buffer::VertexBuffer;

pub struct VertexBufferObject {
    pub mesh:VertexBuffer,
    pub transform:Transform,
    pub offset:u32
}

impl VertexBufferObject {
    pub fn new(mesh:VertexBuffer,transform:Transform,engine:&mut GameEngine) -> VertexBufferObject {
        let offset = engine.static_offset_handler.get_offset();

        return VertexBufferObject {
            mesh,
            transform,
            offset: offset as u32
        }
    }

    pub fn write(&self,engine:&GameEngine, camera:&Camera) {
        engine.queue.write_buffer(&camera.buffers[2], self.offset as u64, &crate::objects::matrix_helper::get_bytes(&self.transform.matrix));
    }
}