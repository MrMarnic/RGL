use crate::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use wgpu::{Buffer, Device, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BindGroupDescriptor, BindGroupEntry, BindingResource, PrimitiveTopology, BufferBindingType, BufferBinding, BufferSize, ShaderStages, Surface, SurfaceConfiguration};
use crate::objects::camera::Camera;
use nalgebra_glm::{vec3, vec4};
use crate::objects::vertex_buffer::OnlyCoordsVertexBuffer;

pub struct LineRenderPipelineGroup {
    pub group: RenderPipelineGroup,
    pub color_buffer: Buffer,
    pub transform_buffer: Buffer
}

impl LineRenderPipelineGroup {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,config:&SurfaceConfiguration,device:&Device) -> LineRenderPipelineGroup{

        let color_buffer = crate::objects::matrix_helper::add_dynamic_buffer(device, (device.limits().min_uniform_buffer_offset_alignment * 256) as u64);

        let mut group_builder = RenderPipelineGroupBuilder::empty();
        group_builder.set_shaders(device,vertex_shader_path,fragment_shader_path,"vertex_line".to_string(),"fragment_line".to_string());

        let color_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: None },
            count: None
        }] });

        let color_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &color_group_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &color_buffer,
                    offset: 0 /*16*/,
                    size: BufferSize::new(16)
                }
            }}]
        });

        group_builder.add_bind_group(color_group,color_group_layout);
        group_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));

        let buffer2 = crate::objects::matrix_helper::add_dynamic_buffer(device,1024 * 10);

        let layout2 = device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: None },
            count: None
        }] });

        let group2 = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout2,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &buffer2,
                    offset: 0 /*128*/,
                    size: BufferSize::new(128)
                }
            } }]
        });
        group_builder.bind_groups_layouts.push(layout2);
        group_builder.bind_groups.push(group2);

        let group = group_builder.build(device,config,OnlyCoordsVertexBuffer::desc(),PrimitiveTopology::LineList,false,"Line".to_string());

        return LineRenderPipelineGroup { group, color_buffer, transform_buffer: buffer2 }
    }
}