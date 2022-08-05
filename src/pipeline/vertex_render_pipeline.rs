use nalgebra_glm::{vec3, vec4};
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType, BufferSize, Device, PrimitiveTopology, ShaderStages, Surface, SurfaceConfiguration};
use crate::light::light::Lights;
use crate::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use crate::objects::camera::Camera;
use crate::objects::obj_model::Material;
use crate::objects::vertex::NormalVertex;
use crate::objects::vertex_buffer::{NormalVertexBuffer, VertexBuffer};

pub struct VertexRenderPipelineGroup {
    pub group: RenderPipelineGroup,
    pub light_buffer: Buffer,
    pub material_buffer: Buffer,
    pub cam_test_buffer: Buffer
}

impl VertexRenderPipelineGroup {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,config:&SurfaceConfiguration,device:&Device) -> VertexRenderPipelineGroup {

        let mut group2_builder = RenderPipelineGroupBuilder::empty();
        group2_builder.set_shaders(&device,vertex_shader_path,fragment_shader_path,"vertex".to_string(),"fragment".to_string());

        let texture_bind_group_layout = group2_builder.create_texture_bind_group_layout(device);
        group2_builder.bind_groups_layouts.push(texture_bind_group_layout);
        group2_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group2_builder.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let light_buffer = crate::objects::matrix_helper::add_buffer(device,&Lights::new().data);
        let material_buffer = crate::objects::matrix_helper::add_dynamic_buffer(device,50 * 256);
        let cam_test_buffer = crate::objects::matrix_helper::add_buffer(device,&crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(0.0,0.0,0.0,0.0)));

        let light_material_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[
            BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
            count: None
        }, BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: BufferSize::new(64) },
                count: None
            }, BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None
            }
        ]
        });

        let light_material_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &light_material_group_layout,
            entries: &[BindGroupEntry { binding: 0, resource: light_buffer.as_entire_binding()},
                BindGroupEntry { binding: 1, resource: BindingResource::Buffer(BufferBinding {
                    buffer: &material_buffer,
                    offset: 0,
                    size: BufferSize::new(64) })
                },BindGroupEntry { binding: 2, resource: cam_test_buffer.as_entire_binding()},]
        });

        group2_builder.add_bind_group(light_material_group,light_material_group_layout);

        let group2 = group2_builder.build(device,config,NormalVertexBuffer::desc(),PrimitiveTopology::TriangleList,true,"Vertex".to_string());

        return VertexRenderPipelineGroup { group: group2, light_buffer, material_buffer, cam_test_buffer }
    }
}