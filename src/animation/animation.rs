use crate::animation::key_frame::KeyFrame;

pub struct Animation {
    pub key_frames: Vec<KeyFrame>,
    pub name: String
}

impl Animation {
    pub fn get_next_key_frame(&self, current:usize) -> Option<&KeyFrame> {
        return self.key_frames.get(current+1)
    }
}

