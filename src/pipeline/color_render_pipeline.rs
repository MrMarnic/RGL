use crate::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use wgpu::{Buffer, Device, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BindGroupDescriptor, BindGroupEntry, BindingResource, PrimitiveTopology, BufferBindingType, BufferSize, BufferBinding, BufferDescriptor, ShaderStages, Surface, SurfaceConfiguration};
use crate::objects::camera::Camera;
use nalgebra_glm::vec3;
use crate::objects::vertex_buffer::OnlyCoordsVertexBuffer;
use wgpu::util::BufferInitDescriptor;
use crate::engine::game_engine::GameEngine;

pub struct ColorRenderPipelineGroup {
    pub group: RenderPipelineGroup,
    pub color_buffer: Buffer
}

impl ColorRenderPipelineGroup {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,device:&Device,config:&SurfaceConfiguration, depth:bool) -> ColorRenderPipelineGroup{

        let color_buffer = crate::objects::matrix_helper::add_dynamic_buffer(device, (device.limits().min_uniform_buffer_offset_alignment * 256) as u64);

        let mut group_builder = RenderPipelineGroupBuilder::empty();
        group_builder.set_shaders(device,vertex_shader_path,fragment_shader_path,"vertex_color".to_string(),"fragment_color".to_string());

        let color_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform,has_dynamic_offset: true, min_binding_size: wgpu::BufferSize::new(16) },
            count: None
        }] });

        let color_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &color_group_layout,
            entries: &[BindGroupEntry { binding: 0, resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &color_buffer,
                offset: 0,
                size: wgpu::BufferSize::new(16)
            })}]
        });


        group_builder.add_bind_group(color_group,color_group_layout);
        group_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group_builder.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let group = group_builder.build(device,config,OnlyCoordsVertexBuffer::desc(),PrimitiveTopology::TriangleList,depth,"Color".to_string());

        return ColorRenderPipelineGroup { group, color_buffer }
    }
}