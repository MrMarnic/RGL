use crate::objects::vertex_buffer::{VertexBuffer, OnlyCoordsVertexBuffer};
use crate::pipeline::color_render_pipeline::ColorRenderPipelineGroup;
use wgpu::{Device, RenderPass, Queue, CommandEncoderDescriptor, BufferAddress, Surface, SurfaceConfiguration};
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use nalgebra_glm::{TVec3, vec4, vec3, Mat4};
use crate::objects::vertex::OnlyCoordsVertex;
use std::ops::Range;
use crate::engine::game_engine::GameEngine;
use crate::objects::offset_handler::StaticOffsetHandler;
use crate::objects::transform::Transform;

pub struct ColorRenderer {
    pub color_shader: ColorRenderPipelineGroup,
    pub mesh: OnlyCoordsVertexBuffer,
    pub to_render: Vec<u64>,
    pub color_offset_handler: StaticOffsetHandler
}

impl ColorRenderer {
    pub fn new(wd:String,device:&Device,config:&SurfaceConfiguration) -> ColorRenderer {
        unsafe {
            let group = ColorRenderPipelineGroup::new(format!("{}\\{}",wd.clone(),"assets\\shader\\color\\vertex.shader"),format!("{}\\{}",wd.clone(),"assets\\shader\\color\\fragment.shader"),device,config,false);
            let mesh = OnlyCoordsVertexBuffer::default_vertex_buffer(device,false);
            return ColorRenderer { color_shader: group, mesh, to_render: vec![], color_offset_handler: StaticOffsetHandler::new(20,20) }
        }
    }


    pub fn render_color_queue<'a>(&'a mut self, offset:&mut BufferAddress, color:&Color, pos:&TVec3<f32>, scale:&TVec3<f32>, camera:&'a Camera, queue:&Queue) {
        queue.write_buffer(&camera.buffers[2],*offset,&*crate::objects::matrix_helper::get_bytes(&nalgebra_glm::scale(&nalgebra_glm::translation(pos),scale)));
        queue.write_buffer(&self.color_shader.color_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(color.r,color.g,color.b,color.a)));
        self.to_render.push(*offset);
        *offset += 256;
    }

    pub fn render_color_queue_transform<'a>(&'a mut self, offset:&mut BufferAddress, color:&Color,transform:&Transform, camera:&'a Camera, queue:&Queue) {
        queue.write_buffer(&camera.buffers[2],*offset,&*crate::objects::matrix_helper::get_bytes(&transform.matrix));
        queue.write_buffer(&self.color_shader.color_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(color.r,color.g,color.b,color.a)));
        self.to_render.push(*offset);
        *offset += 256;
    }

    pub fn render_color_queue_matrix<'a>(&'a mut self, offset:&mut BufferAddress, color:&Color,matrix:&Mat4, camera:&'a Camera, queue:&Queue) {
        queue.write_buffer(&camera.buffers[2],*offset,&*crate::objects::matrix_helper::get_bytes(matrix));
        queue.write_buffer(&self.color_shader.color_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(color.r,color.g,color.b,color.a)));
        self.to_render.push(*offset);
        *offset += 256;
    }

    pub fn begin<'a>(&'a self,render_pass:&mut RenderPass<'a>,camera:&'a Camera){
        render_pass.set_pipeline(&self.color_shader.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
    }

    pub fn finish_custom<'a>(&'a self,render_pass:&mut RenderPass<'a>,camera:&'a Camera,camera_offset:u32,color_offset:u32, mesh:&'a OnlyCoordsVertexBuffer){
        render_pass.set_bind_group(0,&self.color_shader.group.bind_groups[0],&[color_offset]);
        render_pass.set_bind_group(2,&camera.transform_bind_group,&[camera_offset]);
        mesh.render(render_pass);
    }

    pub fn finish<'a>(&'a self,render_pass:&mut RenderPass<'a>,camera:&'a Camera,range:Range<usize>){
        render_pass.set_pipeline(&self.color_shader.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
        for (i,off) in self.to_render[range.clone()].iter().enumerate() {
            render_pass.set_bind_group(0,&self.color_shader.group.bind_groups[0],&[((i as u32 + range.start as u32) * 256 as u32) as u32]);
            render_pass.set_bind_group(2,&camera.transform_bind_group,&[*off as u32]);
            self.mesh.render(render_pass);
        }
        //self.to_render = 0;
    }
}