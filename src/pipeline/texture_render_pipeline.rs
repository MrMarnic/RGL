use crate::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use wgpu::{Buffer, Device, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BindGroupDescriptor, BindGroupEntry, BindingResource, PrimitiveTopology, QUERY_RESOLVE_BUFFER_ALIGNMENT, BufferBindingType, Limits, BufferBinding, BufferSize, ShaderStages, Surface, SurfaceConfiguration};
use crate::objects::camera::Camera;
use nalgebra_glm::vec3;
use crate::objects::vertex_buffer::{OnlyCoordsVertexBuffer, VertexBuffer};

pub struct TextureRenderPipelineGroup {
    pub group: RenderPipelineGroup,
    pub offset_buffer: Buffer
}

impl TextureRenderPipelineGroup {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,config:&SurfaceConfiguration,device:&Device) -> TextureRenderPipelineGroup {

        let mut group2_builder = RenderPipelineGroupBuilder::empty();
        group2_builder.set_shaders(&device,vertex_shader_path,fragment_shader_path,"vertex_sprite".to_string(),"fragment_sprite".to_string());

        let texture_bind_group_layout = group2_builder.create_texture_bind_group_layout(device);
        group2_builder.bind_groups_layouts.push(texture_bind_group_layout);
        group2_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group2_builder.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let offset_buffer = crate::objects::matrix_helper::add_dynamic_buffer(device,QUERY_RESOLVE_BUFFER_ALIGNMENT * 256 /*50*/);

        let offset_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: None },
            count: None
        }] });

        let offset_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &offset_group_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &offset_buffer,
                    offset: 0 /*64*/,
                    size: BufferSize::new(64)
                }
            } }]
        });

        group2_builder.add_bind_group(offset_group,offset_group_layout);
        let group2 = group2_builder.build(device,config,VertexBuffer::desc(),PrimitiveTopology::TriangleList,false,"Texture".to_string());

        return TextureRenderPipelineGroup { group: group2, offset_buffer }
    }
}