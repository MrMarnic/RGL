use wgpu::{Device, RenderPass, Surface, SurfaceConfiguration};
use crate::objects::texture_map::TextureMap;
use std::rc::Rc;
use crate::objects::texture_object::TextureObject;
use crate::objects::camera::Camera;
use crate::pipeline::custom_vertex_render_pipeline::CustomVertexRenderPipelineGroup;
use crate::objects::vertex_buffer::VertexBuffer;

pub struct CustomVertexRenderer {
    pub shader: CustomVertexRenderPipelineGroup
}

impl CustomVertexRenderer {
    pub fn new(working_dir:String,device:&Device,config:&SurfaceConfiguration) -> CustomVertexRenderer {
        unsafe {
            let shader = CustomVertexRenderPipelineGroup::new(format!("{}\\{}", &working_dir.to_string(), "assets\\shader\\custom_vertex\\vertex.shader"), format!("{}\\{}", &working_dir.to_string(), "assets\\shader\\custom_vertex\\fragment.shader"), config, device);

            return CustomVertexRenderer { shader }
        }
    }

    pub fn begin<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        render_pass.set_pipeline(&self.shader.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
    }

    pub fn render_textureless<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera, offset: u32, mesh: &'a VertexBuffer) {
        render_pass.set_bind_group(2,&camera.transform_bind_group,&[offset]);
        mesh.render(render_pass);
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera,tex:&'a Rc<TextureObject>, offset: u32, mesh: &'a VertexBuffer) {
        render_pass.set_bind_group(0,&tex.bind_group,&[]);
        render_pass.set_bind_group(2,&camera.transform_bind_group,&[offset]);
        mesh.render(render_pass);
    }

    pub fn begin_depth<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        render_pass.set_pipeline(&self.shader.group_depth.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
    }
}