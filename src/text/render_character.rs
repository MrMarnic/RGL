use std::rc::Rc;
use rgl_font::Character;
use crate::objects::tex_coord::TexCoord;
use crate::objects::texture_object::TextureObject;
use crate::objects::vertex::Vertex;

pub struct RenderCharacter {
    pub id: char,
    pub tex_coord: TexCoord,
    pub vertecies: Vec<Vertex>,
    pub indecies: Vec<u16>,
    pub x_offset:f32,
    pub y_offset:f32,
    pub x_advance:i32
}

impl RenderCharacter {
    pub fn new(texture:Rc<TextureObject>,id:char,char:&Character) -> RenderCharacter{
        let coord = TexCoord::new_from_texture(char.x as f32,texture.size.height as f32 - char.y as f32 - char.height as f32,char.width as f32,char.height as f32,texture.clone());

        let mut vexs = vec![Vertex::new(0.0,char.height as f32,0.0,coord.tex_coords[0].x,coord.tex_coords[0].y),Vertex::new(0.0,0.0,0.0,coord.tex_coords[1].x,coord.tex_coords[1].y),
                            Vertex::new(char.width as f32,0.0,0.0,coord.tex_coords[3].x,coord.tex_coords[3].y),Vertex::new(char.width as f32,char.height as f32,0.0,coord.tex_coords[2].x,coord.tex_coords[2].y)];

        let indecies = vec![0,1,3,3,1,2];

        return RenderCharacter {
            id,
            tex_coord: coord,
            vertecies: vexs,
            indecies,
            x_offset: char.x_offset,
            y_offset: char.y_offset,
            x_advance: char.x_advance
        }
    }
}