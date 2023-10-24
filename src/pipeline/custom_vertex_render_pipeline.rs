use wgpu::{BindGroupLayout, Device, PrimitiveTopology, SamplerBindingType, Surface, SurfaceConfiguration};
use crate::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use crate::objects::camera::Camera;
use crate::objects::vertex_buffer::VertexBuffer;

pub struct CustomVertexRenderPipelineGroup {
    pub group: RenderPipelineGroup,
    pub group_depth: RenderPipelineGroup
}

impl CustomVertexRenderPipelineGroup {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,config:&SurfaceConfiguration,device:&Device) -> CustomVertexRenderPipelineGroup {

        let mut group2_builder = RenderPipelineGroupBuilder::empty();
        group2_builder.set_shaders(&device,vertex_shader_path.clone(),fragment_shader_path.clone(),"vertex_custom".to_string(),"fragment_custom".to_string());

        let texture_bind_group_layout = group2_builder.create_texture_bind_group_layout(device);
        group2_builder.bind_groups_layouts.push(texture_bind_group_layout);
        group2_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group2_builder.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let group2_depth = group2_builder.build(device,config,VertexBuffer::desc(),PrimitiveTopology::TriangleList,true,"Custom Vertex Depth".to_string());

        let mut group2_builder2 = RenderPipelineGroupBuilder::empty();
        group2_builder2.set_shaders(&device,vertex_shader_path,fragment_shader_path,"vertex_custom".to_string(),"fragment_custom".to_string());

        let texture_bind_group_layout = group2_builder2.create_texture_bind_group_layout(device);
        group2_builder2.bind_groups_layouts.push(texture_bind_group_layout);
        group2_builder2.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group2_builder2.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let group2 = group2_builder2.build(device,config,VertexBuffer::desc(),PrimitiveTopology::TriangleList,true,"Custom Vertex Depth".to_string());

        return CustomVertexRenderPipelineGroup { group: group2, group_depth: group2_depth }
    }
}