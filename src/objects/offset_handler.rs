use std::sync::{Arc, Mutex};

pub struct OffsetHandler {
    pub camera_offset: u64,
    pub line_offset: u64
}

impl OffsetHandler {
    pub fn new() -> OffsetHandler {
        return OffsetHandler { camera_offset: 0, line_offset: 0 }
    }
}

pub struct StaticOffsetHandler {
    pub freed_offset: Vec<u64>,
    pub to_remove: Arc<Mutex<Vec<u64>>>,
    size: i32
}

impl StaticOffsetHandler {
    pub fn new(size:i32,start_cap:u64) -> StaticOffsetHandler {

        let mut freed_offset : Vec<u64> = vec![];
        let start_offset =  start_cap * 256;

        for i in 0..size {
            freed_offset.push(i as u64 * 256 + start_offset)
        }

        return StaticOffsetHandler {
            freed_offset,
            to_remove: Arc::new(Mutex::new(vec![])),
            size
        }
    }

    pub fn remove_queue(&mut self,remove:u64) {
        self.to_remove.lock().unwrap().push(remove);
    }

    pub fn remove(&mut self,remove:u64) {
        self.freed_offset.push(remove);
    }

    pub fn update(&mut self) {
        if !self.to_remove.lock().unwrap().is_empty() {
            let mut to_remove_real  = vec![];
            for i in self.to_remove.lock().unwrap().iter() {
                to_remove_real.push(*i);
            }
            self.to_remove.lock().unwrap().clear();

            for i in to_remove_real {
                self.remove(i);
            }
        }
    }

    pub fn get_offset(&mut self) -> u64 {
        let num = self.freed_offset.remove(0);
        return num;
    }

    pub fn reset(&mut self) {
        self.freed_offset.clear();
        self.to_remove.lock().unwrap().clear();
        let start_offset =  100 * 256;

        for i in 0..self.size {
            self.freed_offset.push(i as u64 * 256 + start_offset)
        }
    }
}