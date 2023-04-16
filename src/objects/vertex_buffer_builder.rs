use std::rc::Rc;
use nalgebra_glm::{TVec2, TVec3, vec2, vec3};
use wgpu::Device;
use crate::engine::game_engine::GameEngine;
use crate::objects::vertex::Vertex;
use crate::objects::vertex_buffer::VertexBuffer;
use crate::text::render_font::RenderFont;

pub struct VertexBufferBuilder {
    pub vertecies: Vec<Vertex>,
    pub indecies: Vec<u32>,
    index_count: u32
}

impl VertexBufferBuilder {
    pub fn new() -> VertexBufferBuilder {
        return VertexBufferBuilder { vertecies: vec![], indecies: vec![], index_count: 0 }
    }

    pub fn square(mut self, x:f32, y:f32, width:f32) -> Self{
        let vertecies = vec![Vertex::new(x-width,y+width,0.0,0.0,0.0),Vertex::new(x-width,y-width,0.0,0.0,1.0),Vertex::new(x+width,y-width,0.0,1.0,1.0),Vertex::new(x+width,y+width,0.0,1.0,0.0)];
        let indecies = vec![self.index_count,self.index_count+1,self.index_count+3,self.index_count+3,self.index_count+1,self.index_count+2];

        self.vertecies.extend(vertecies);
        self.indecies.extend(indecies);
        self.index_count +=4;

        self
    }

    pub fn square_a(&mut self, x:f32, y:f32, width:f32){
        let vertecies = vec![Vertex::new(x-width,y+width,0.0,0.0,0.0),Vertex::new(x-width,y-width,0.0,0.0,1.0),Vertex::new(x+width,y-width,0.0,1.0,1.0),Vertex::new(x+width,y+width,0.0,1.0,0.0)];
        let indecies = vec![self.index_count,self.index_count+1,self.index_count+3,self.index_count+3,self.index_count+1,self.index_count+2];

        self.vertecies.extend(vertecies);
        self.indecies.extend(indecies);
        self.index_count +=4;
    }

    pub fn rectangle(mut self, x:f32, y:f32, z:f32, width:f32, height:f32,depth:f32) -> Self{
        let vertecies = vec![Vertex::new(x-width,y+height,z+depth,0.0,0.0),Vertex::new(x-width,y-height,z-depth,0.0,1.0),Vertex::new(x+width,y-height,z-depth,1.0,1.0),Vertex::new(x+width,y+height,z+depth,1.0,0.0)];
        let indecies = vec![self.index_count,self.index_count+1,self.index_count+3,self.index_count+3,self.index_count+1,self.index_count+2];

        self.vertecies.extend(vertecies);
        self.indecies.extend(indecies);

        self.index_count +=4;

        self
    }

    pub fn add_rectangle(&mut self, x:f32, y:f32, z:f32, width:f32, height:f32,depth:f32){
        let vertecies = vec![Vertex::new(x-width,y+height,z+depth,0.0,0.0),Vertex::new(x-width,y-height,z-depth,0.0,1.0),Vertex::new(x+width,y-height,z-depth,1.0,1.0),Vertex::new(x+width,y+height,z+depth,1.0,0.0)];
        let indecies = vec![self.index_count,self.index_count+1,self.index_count+3,self.index_count+3,self.index_count+1,self.index_count+2];

        self.vertecies.extend(vertecies);
        self.indecies.extend(indecies);

        self.index_count +=4;
    }

    pub fn line(mut self, x1:f32, y1:f32, z1:f32, x2:f32, y2:f32, z2:f32, width:f32) -> Self{

        let mut width = width;
        let mut height = width;

        let mut m_vec = vec2((y2-y1),(x2-x1));
        m_vec = m_vec.normalize();

        let vertecies = vec![Vertex::new(x1-width*m_vec.x,y1+height*m_vec.y,z1,0.0,0.0),Vertex::new(x1+width*m_vec.x,y1-height*m_vec.y,z1,0.0,1.0),Vertex::new(x2+width*m_vec.x,y2-height*m_vec.y,z2,1.0,1.0),Vertex::new(x2-width*m_vec.x,y2+height*m_vec.y,z2,1.0,0.0)];
        let indecies = vec![self.index_count,self.index_count+1,self.index_count+3,self.index_count+3,self.index_count+1,self.index_count+2];

        self.vertecies.extend(vertecies);
        self.indecies.extend(indecies);

        self.index_count +=4;

        self
    }

    //                              0                       1
    pub fn line_connected(&mut self,ox1:f32,oy1:f32,oz1:f32, x2:f32, y2:f32, z2:f32, width:f32, origin_max:TVec3<f32>,origin_min:TVec3<f32>, t:bool) -> (TVec2<f32>,TVec3<f32>,TVec3<f32>){

        let mut width = width;
        let mut height = width;

        let mut m_vec = vec2((y2-oy1),(x2-ox1));
        m_vec = m_vec.normalize();
                                        //0                                                                     1                                                                       2                                                                   3
        let vertecies = vec![Vertex::new(ox1-width*m_vec.x,oy1+height*m_vec.y,oz1,0.0,0.0),Vertex::new(ox1+width*m_vec.x,oy1-height*m_vec.y,oz1,0.0,1.0),Vertex::new(x2+width*m_vec.x,y2-height*m_vec.y,z2,1.0,1.0),Vertex::new(x2-width*m_vec.x,y2+height*m_vec.y,z2,1.0,0.0)];
        let indecies = vec![self.index_count,self.index_count+1,self.index_count+3,self.index_count+3,self.index_count+1,self.index_count+2];

        self.vertecies.extend(vertecies);
        self.indecies.extend(indecies);

        self.index_count +=4;

        if t {
            self.add_triangle_vertex(Vertex::new(origin_max.x,origin_max.y,origin_max.z,0.0,0.0),Vertex::new(ox1-width*m_vec.x,oy1+height*m_vec.y,oz1,0.0,0.0),Vertex::new(ox1,oy1,oz1,0.0,0.0));
        }
        //self.add_triangle_vertex(Vertex::new(origin_min.x,origin_min.y,origin_min.z,0.0,0.0),Vertex::new(ox1+width*m_vec.x,oy1-height*m_vec.y,oz1,1.0,0.0),Vertex::new(ox1,oy1,oz1,0.0,1.0));

        return (vec2(x2,y2),vec3(x2-width*m_vec.x,y2+height*m_vec.y,z2),vec3(x2+width*m_vec.x,y2-height*m_vec.y,z2));
    }

    pub fn add_triangle_vertex(&mut self, v1: Vertex, v2: Vertex, v3: Vertex){
        let vertecies = vec![v1,v2,v3];
        let indecies = vec![self.index_count,self.index_count+1,self.index_count+2];

        self.vertecies.extend(vertecies);
        self.indecies.extend(indecies);
        self.index_count +=3;
    }

    pub fn circle(mut self,x_base:f32,y_base:f32,radius:f32,quality:i32) -> Self{
        let mut old_x = 0.0;
        let mut old_y = 0.0;

        for deg in (0..360*quality) {
            let x = (deg as f32/quality as f32).sin();
            let y = (deg as f32/quality as f32).cos();

            self.add_triangle_vertex(Vertex::new(x_base + old_x * radius,y_base + old_y * radius,0.0,x,y),Vertex::new(x_base + x * radius,y_base + y * radius,0.0,x,y),Vertex::new(x_base,y_base,0.0,x,y));

            old_x = x;
            old_y = y;
        }

        self
    }

    pub fn add_text(&mut self, lines:&Vec<String>,engine:&GameEngine, font:&Rc<RenderFont>,width:f32, x_start: f32, y_start: f32){
        let mut number = 0;

        let mut chars : Vec<Vec<char>> = vec![];

        for l in lines {
            chars.push(l.chars().collect::<Vec<char>>())
        }

        let mut unedited_chars = chars.clone();
        let mut remapped_lines : Vec<String> = vec![];

        while unedited_chars.len() > 0 {
            let mut cs = unedited_chars.remove(0);

            if let Some(index) = font.base.get_chars_test(width,&cs) {
                if index == cs.len() - 1 {
                    remapped_lines.push(cs.iter().collect());
                } else {
                    let a = cs.drain(0..index).collect::<String>();
                    unedited_chars.insert(0,cs);
                    remapped_lines.push(a);
                }
            }
        }

        for (line_number,text) in remapped_lines.iter().enumerate() {
            let mut last_pos = 0.0;

            for (id,c) in text.chars().enumerate() {
                if !c.is_whitespace() {
                    if id > 0 {
                        let char = &font.characters[&c];
                        let mut vexs = vec![];
                        for v in &font.characters[&c].vertecies {
                            let mut vv = v.clone();
                            vv.x = x_start + vv.x + last_pos + char.x_offset as f32;
                            vv.y = y_start + vv.y - line_number as f32 * font.base.pixel_height as f32 + char.y_offset as f32;
                            vexs.push(vv);
                        }
                        self.vertecies.extend_from_slice(&vexs);

                        self.indecies.extend_from_slice(&vec![self.index_count + 4 * number as u32,(self.index_count + 1) + 4 * number as u32,(self.index_count + 3) + 4 * number as u32,(self.index_count + 3) + 4 * number as u32,(self.index_count + 1) + 4 * number as u32,(self.index_count + 2) + 4 * number as u32]);
                        last_pos += char.x_advance as f32;
                    } else {
                        let char = &font.characters[&c];
                        let mut vexs = vec![];
                        for v in &font.characters[&c].vertecies {
                            let mut vv = v.clone();
                            vv.x = x_start + vv.x + char.x_offset as f32;
                            vv.y = y_start + vv.y - line_number as f32 * font.base.pixel_height as f32 + char.y_offset as f32;
                            vexs.push(vv);
                        }
                        self.vertecies.extend_from_slice(&vexs);
                        self.indecies.extend_from_slice(&vec![self.index_count + 4 * number as u32,(self.index_count + 1) + 4 * number as u32,(self.index_count + 3) + 4 * number as u32,(self.index_count + 3) + 4 * number as u32,(self.index_count + 1) + 4 * number as u32,(self.index_count + 2) + 4 * number as u32]);
                        last_pos += char.x_advance as f32;
                    }
                    number += 1;
                } else {
                    last_pos += font.characters[&c].x_advance as f32;
                }
            }
        }

        self.index_count += number * 4;
    }

    pub fn build(self,device:&Device) -> VertexBuffer {
        let buffer = VertexBuffer::new(&device,self.vertecies,self.indecies,false);

        return buffer;
    }

    pub fn is_empty(&self) -> bool {
        return self.vertecies.is_empty();
    }
}