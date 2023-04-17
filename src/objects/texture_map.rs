use std::rc::Rc;
use crate::objects::tex_coord::TexCoord;
use nalgebra_glm::{TVec2, vec2};
use crate::objects::texture_object::TextureObject;

pub struct TextureMap {
    pub texture:Rc<TextureObject>,
    pub width: i32,
    pub height:i32,
    pub normal_size:i32
}

impl TextureMap{
    pub fn new(tex:Rc<TextureObject>,normal_size:i32) -> TextureMap {
        return TextureMap {
            texture: tex.clone(),
            width: tex.size.width as i32,
            height: tex.size.height as i32,
            normal_size
        }
    }

    pub fn get_tex_coord(&self,x:i32,y:i32) -> TexCoord {
        return TexCoord::new_from_texture(x as f32 * self.normal_size as f32,y as f32 * self.normal_size as f32,self.normal_size as f32,self.normal_size as f32,self.texture.clone());
    }

    pub fn get_tex_coord_at(&self,x:i32,y:i32) -> TexCoord {
        return TexCoord::new_from_texture(x as f32,y as f32,self.normal_size as f32,self.normal_size as f32,self.texture.clone());
    }

    pub fn get_tex_coord_at_with_size(&self,x:i32,y:i32,width:i32,height:i32) -> TexCoord {
        return TexCoord::new_from_texture(x as f32,y as f32,width as f32,height as f32,self.texture.clone());
    }
}