use std::collections::HashMap;
use wgpu::{Buffer, Queue};
use crate::engine::resource_loader::ResourceLoader;
use crate::objects::obj_model::Material;
use crate::objects::offset_handler::StaticOffsetHandler;

pub struct MaterialManager {
    pub offset_handler: StaticOffsetHandler
}

impl MaterialManager {

    pub fn new() -> Self {
        MaterialManager { offset_handler: StaticOffsetHandler::new(50,0) }
    }

    pub fn register(&mut self, mats:&mut HashMap<String,Material>, buffer:&Buffer, queue:&Queue, rsc_loader:&ResourceLoader) {
        for (id,m) in mats {
            m.offset = self.offset_handler.get_offset() as u32;
            m.register(queue,buffer,rsc_loader);
        }
    }
}