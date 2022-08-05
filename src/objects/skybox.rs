use std::rc::Rc;
use nalgebra_glm::vec3;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::texture_map::TextureMap;
use crate::objects::texture_object::TextureObject;
use crate::objects::transform::Transform;
use crate::objects::vertex::Vertex;
use crate::objects::vertex_buffer::VertexBuffer;

pub struct SkyBox {
    pub texture_map: TextureMap,
    pub transform: Transform,
    pub offset:u32,
    pub layout:SkyboxLayout,
    pub mesh: VertexBuffer
}

pub struct SkyboxLayout {
    pub front:i32,
    pub down:i32,
    pub up:i32,
    pub back:i32,
    pub right:i32,
    pub left:i32
}

impl SkyBox {
    pub fn new(size:f32,texture:Rc<TextureObject>,layout:SkyboxLayout, engine:&mut GameEngine) -> Self{
        let w = texture.size.width as i32 / 6;
        
        let map = TextureMap::new(texture,w);

        let mut vertecies = vec![Vertex::new(-size,size,-size,map.get_tex_coord(layout.front,0).tex_coords[0].x,map.get_tex_coord(layout.front,0).tex_coords[0].y)
                                 , Vertex::new(-size,-size,-size,map.get_tex_coord(layout.front,0).tex_coords[1].x,map.get_tex_coord(layout.front,0).tex_coords[1].y)
                                 ,Vertex::new(size,size,-size,map.get_tex_coord(layout.front,0).tex_coords[2].x,map.get_tex_coord(layout.front,0).tex_coords[2].y)
                                 ,Vertex::new(size,-size,-size,map.get_tex_coord(layout.front,0).tex_coords[3].x,map.get_tex_coord(layout.front,0).tex_coords[3].y)];

        let mut indecies = vec![0,1,2,1,3,2]; //FRONT


        vertecies.extend_from_slice(&vec![Vertex::new(-size,-size,-size,map.get_tex_coord(layout.down,0).tex_coords[0].x,map.get_tex_coord(layout.down,0).tex_coords[0].y),
                                          Vertex::new(-size,-size,size,map.get_tex_coord(layout.down,0).tex_coords[1].x,map.get_tex_coord(layout.down,0).tex_coords[1].y)
                                          ,Vertex::new(size,-size,-size,map.get_tex_coord(layout.down,0).tex_coords[2].x,map.get_tex_coord(layout.down,0).tex_coords[2].y),
                                          Vertex::new(size,-size,size,map.get_tex_coord(layout.down,0).tex_coords[3].x,map.get_tex_coord(layout.down,0).tex_coords[3].y)]);

        indecies.extend_from_slice(&vec![4,5,6,5,7,6]); //DOWN



        vertecies.extend_from_slice(&vec![Vertex::new(-size,size,-size,map.get_tex_coord(layout.up,0).tex_coords[1].x,map.get_tex_coord(layout.up,0).tex_coords[1].y),
                                          Vertex::new(-size,size,size,map.get_tex_coord(layout.up,0).tex_coords[0].x,map.get_tex_coord(layout.up,0).tex_coords[0].y)
                                          ,Vertex::new(size,size,-size,map.get_tex_coord(layout.up,0).tex_coords[3].x,map.get_tex_coord(layout.up,0).tex_coords[3].y),
                                          Vertex::new(size,size,size,map.get_tex_coord(layout.up,0).tex_coords[2].x,map.get_tex_coord(layout.up,0).tex_coords[2].y)]);

        indecies.extend_from_slice(&vec![8,9,10,9,11,10]); //UP



        vertecies.extend_from_slice(&vec![Vertex::new(-size,size,size,map.get_tex_coord(layout.back,0).tex_coords[2].x,map.get_tex_coord(layout.back,0).tex_coords[2].y),
                                          Vertex::new(-size,-size,size,map.get_tex_coord(layout.back,0).tex_coords[3].x,map.get_tex_coord(layout.back,0).tex_coords[3].y)
                                          ,Vertex::new(size,size,size,map.get_tex_coord(layout.back,0).tex_coords[0].x,map.get_tex_coord(layout.back,0).tex_coords[0].y),
                                          Vertex::new(size,-size,size,map.get_tex_coord(layout.back,0).tex_coords[1].x,map.get_tex_coord(layout.back,0).tex_coords[1].y)]);

        indecies.extend_from_slice(&vec![12,13,14,13,15,14]); //BACK


        vertecies.extend_from_slice(&vec![Vertex::new(size,size,-size,map.get_tex_coord(layout.right,0).tex_coords[0].x,map.get_tex_coord(layout.right,0).tex_coords[0].y),
                                          Vertex::new(size,-size,-size,map.get_tex_coord(layout.right,0).tex_coords[1].x,map.get_tex_coord(layout.right,0).tex_coords[1].y)
                                          ,Vertex::new(size,size,size,map.get_tex_coord(layout.right,0).tex_coords[2].x,map.get_tex_coord(layout.right,0).tex_coords[2].y),
                                          Vertex::new(size,-size,size,map.get_tex_coord(layout.right,0).tex_coords[3].x,map.get_tex_coord(layout.right,0).tex_coords[3].y)]);

        indecies.extend_from_slice(&vec![16,17,18,17,19,18]); //RIGHT



        vertecies.extend_from_slice(&vec![Vertex::new(-size,size,-size,map.get_tex_coord(layout.left,0).tex_coords[2].x,map.get_tex_coord(layout.left,0).tex_coords[2].y)
                                          ,Vertex::new(-size,-size,-size,map.get_tex_coord(layout.left,0).tex_coords[3].x,map.get_tex_coord(layout.left,0).tex_coords[3].y)
                                          ,Vertex::new(-size,size,size,map.get_tex_coord(layout.left,0).tex_coords[0].x,map.get_tex_coord(layout.left,0).tex_coords[0].y),
                                          Vertex::new(-size,-size,size,map.get_tex_coord(layout.left,0).tex_coords[1].x,map.get_tex_coord(layout.left,0).tex_coords[1].y)]); //LEFT

        indecies.extend_from_slice(&vec![20,21,22,21,23,22]);

        let mesh = VertexBuffer::new(&engine.device,vertecies,indecies,false);
        
        SkyBox {
            texture_map: map,
            transform: Transform::new(0.0,0.0,0.0,vec3(1.0,1.0,1.0)),
            offset: engine.static_offset_handler.get_offset() as u32,
            layout,
            mesh
        }
    }
    
    pub fn write(&self,camera:&Camera,engine:&GameEngine) {
        engine.queue.write_buffer(&camera.buffers[2],self.offset as u64,&*crate::objects::matrix_helper::get_bytes(&self.transform.matrix));
    }
}