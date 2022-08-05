/*
use crate::objects::tex_coord::TexCoord;
use std::rc::Rc;
use crate::objects::texture_object::TextureObject;
use crate::objects::vertex::{Vertex, OnlyCoordsVertex};

pub struct Character {
    pub id:u8,
    pub x:i32,
    pub y:i32,
    pub width:i32,
    pub height:i32,
    pub x_offset:f32,
    pub y_offset:f32,
    pub x_advance:i32,
    pub page:i32,
    pub chnl:i32,
    pub tex_coord: TexCoord,
    pub vertecies: Vec<Vertex>,
    pub indecies: Vec<u16>
}

impl Character {
    pub fn set_coord_and_vertecies_and_indecies(&mut self,texture:Rc<TextureObject>) {
        let coord = TexCoord::new_from_texture(self.x as f32,texture.size.height as f32 - self.y as f32 - self.height as f32,self.width as f32,self.height as f32,texture.clone());
        self.tex_coord = coord;

        let mut vexs = vec![Vertex::new(0.0,self.height as f32,0.0,self.tex_coord.tex_coords[0].x,self.tex_coord.tex_coords[0].y),Vertex::new(0.0,0.0,0.0,self.tex_coord.tex_coords[1].x,self.tex_coord.tex_coords[1].y),
                            Vertex::new(self.width as f32,0.0,0.0,self.tex_coord.tex_coords[3].x,self.tex_coord.tex_coords[3].y),Vertex::new(self.width as f32,self.height as f32,0.0,self.tex_coord.tex_coords[2].x,self.tex_coord.tex_coords[2].y)];

        let indecies = vec![0,1,3,3,1,2];

        self.vertecies = vexs;
        self.indecies = indecies;
    }

    pub fn empty() -> Character {
        return Character{
            id: 0,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            x_offset: 0.0,
            y_offset: 0.0,
            x_advance: 0,
            page: 0,
            chnl: 0,
            tex_coord: TexCoord::default(),
            vertecies: vec![],
            indecies: vec![]
        }
    }
}
 */