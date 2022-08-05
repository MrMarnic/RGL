use nalgebra_glm::{TVec2, vec2};
use std::rc::Rc;
use crate::objects::texture_object::TextureObject;

#[derive(Clone)]
pub struct TexCoord {
    pub tex_coords: Vec<TVec2<f32>>,
    pub array: Vec<f32>,
    pub default:bool
}

impl TexCoord {

    /*
        array.push(0.0);
        array.push(0.0);
        Was added to to align to vec4 layout.
     */

    pub fn default() -> TexCoord {
        let mut vecs : Vec<TVec2<f32>> = vec![];
        vecs.push(vec2(0.0,0.0));
        vecs.push(vec2(0.0,1.0));
        vecs.push(vec2(1.0,1.0));
        vecs.push(vec2(1.0,0.0));

        let mut array = vec![];
        for v in &vecs {
            array.push(v.x);
            array.push(v.y);
            array.push(0.0);
            array.push(0.0);
        }

        return TexCoord { tex_coords: vecs, array,default: true }
    }

    pub fn new(v1:TVec2<f32>,v2:TVec2<f32>,v3:TVec2<f32>,v4:TVec2<f32>) -> TexCoord {
        let mut vecs : Vec<TVec2<f32>> = vec![];
        vecs.push(v1);
        vecs.push(v2);
        vecs.push(v3);
        vecs.push(v4);

        let mut array = vec![];
        for v in &vecs {
            array.push(v.x);
            array.push(v.y);
            array.push(0.0);
            array.push(0.0);
        }

        return TexCoord { tex_coords: vecs, array,default:false }
    }

    pub fn new_from_texture(x:f32,y:f32,width:f32,height:f32,texture:Rc<TextureObject>) -> TexCoord {
        let mut vecs : Vec<TVec2<f32>> = vec![];


        let value_w = width / texture.size.width as f32;
        let value_h = height / texture.size.height as f32;

        vecs.push(vec2(x/texture.size.width as f32,(texture.size.height as f32 - y - height)/(texture.size.height as f32)));// 0 0
        vecs.push(vec2(x/texture.size.width as f32,(texture.size.height as f32- y - height)/texture.size.height as f32 + value_h)); // 0 1
        vecs.push(vec2(x/(texture.size.width as f32) + value_w,(texture.size.height as f32 - y - height)/texture.size.height as f32));// 1 0
        vecs.push(vec2(x/(texture.size.width as f32) + value_w,(texture.size.height as f32 - y - height)/texture.size.height as f32 + value_h));// 1 1

        let mut array = vec![];
        for v in &vecs {
            array.push(v.x);
            array.push(v.y);
            array.push(0.0);
            array.push(0.0);
        }

        return TexCoord { tex_coords: vecs, array,default:false }
    }
}