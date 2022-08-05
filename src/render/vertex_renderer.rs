use wgpu::{Device, Queue, RenderPass, Surface, SurfaceConfiguration};
use crate::objects::texture_map::TextureMap;
use std::rc::Rc;
use image::{ImageBuffer, Rgba, RgbaImage};
use crate::objects::texture_object::TextureObject;
use crate::objects::camera::Camera;
use crate::objects::vertex_buffer::{NormalVertexBuffer, VertexBuffer};
use crate::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use crate::pipeline::vertex_render_pipeline::VertexRenderPipelineGroup;

pub struct VertexRenderer {
    pub shader: VertexRenderPipelineGroup,
    pub default_tex: Rc<TextureObject>
}

impl VertexRenderer {
    pub fn new(working_dir:String,device:&Device,config:&SurfaceConfiguration,queue:&Queue) -> VertexRenderer {

        let mut img : RgbaImage = ImageBuffer::new(1,1);
        img.put_pixel(0,0,Rgba([255,255,255,255]));

        let obj = TextureObject::new_from_data(&img,"".to_string(),device,queue,&RenderPipelineGroupBuilder::empty().create_texture_bind_group_layout(device));

        unsafe {
            let shader = VertexRenderPipelineGroup::new(format!("{}\\{}", &working_dir.to_string(), "assets\\shader\\vertex\\vertex.shader"), format!("{}\\{}", &working_dir.to_string(), "assets\\shader\\vertex\\fragment.shader"), config, device);
            return VertexRenderer { shader, default_tex: Rc::new(obj) }
        }
    }

    pub fn begin<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        render_pass.set_pipeline(&self.shader.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera,tex:&'a Rc<TextureObject>, offset: u32, mesh: &'a NormalVertexBuffer, material_offset: u32) {
        render_pass.set_bind_group(0,&tex.bind_group,&[]);
        render_pass.set_bind_group(2,&camera.transform_bind_group,&[offset]);
        render_pass.set_bind_group(3,&self.shader.group.bind_groups[0],&[material_offset]);
        mesh.render(render_pass);
    }
}