use std::collections::HashMap;
use std::rc::Rc;
use rgl_font::ScaledFont;
use wgpu::{BindGroupLayout, Device, Queue};
use crate::engine::game_engine::GameEngine;
use crate::objects::texture_object::TextureObject;
use crate::text::render_character::RenderCharacter;

pub struct RenderFont {
    pub base:Rc<ScaledFont>,
    pub characters:HashMap<char,RenderCharacter>,
    pub texture: Rc<TextureObject>
}

impl RenderFont {
    pub fn new(base:Rc<ScaledFont>, device:&Device,queue:&Queue, layout:&BindGroupLayout) -> RenderFont{

        let texture = Rc::new(TextureObject::new_from_data(&base.image,"".to_string(),device,queue,layout));

        let mut chars = HashMap::new();

        for (id,c) in base.chars.iter() {
            chars.insert(c.id,RenderCharacter::new(texture.clone(),c.id,&c));
        }

        return RenderFont { base, characters: chars, texture }
    }

    pub fn get_width(&self, text:&String) -> f32 {
        return self.base.get_width(text);
    }

    pub fn get_height(&self, text:&String) -> f32 {
        return self.base.get_height(text);
    }
}