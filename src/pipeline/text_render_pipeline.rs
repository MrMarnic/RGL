use crate::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use wgpu::{Buffer, Device, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BindGroupDescriptor, BindGroupEntry, BindingResource, PrimitiveTopology, PipelineLayoutDescriptor, RenderPipelineDescriptor, FrontFace, IndexFormat, BlendFactor, BlendOperation, BindGroup, MultisampleState, VertexState, PrimitiveState, PolygonMode, FragmentState, BlendState, ColorTargetState, VertexBufferLayout, BufferBindingType, BlendComponent, BufferBinding, BufferSize, ShaderStages, ColorWrites, SurfaceConfiguration, TextureFormat, DepthStencilState, StencilState, DepthBiasState, CompareFunction};
use crate::objects::camera::Camera;
use nalgebra_glm::vec3;
use crate::objects::vertex_buffer::{OnlyCoordsVertexBuffer, VertexBuffer};

pub struct TextRenderPipelineGroup {
    pub group: RenderPipelineGroup,
    pub color_buffer: Buffer,
    pub color_group: BindGroup
}

impl TextRenderPipelineGroup {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,config:&SurfaceConfiguration,device:&Device, depth:bool) -> TextRenderPipelineGroup{

        let mut group2_builder = RenderPipelineGroupBuilder::empty();
        group2_builder.set_shaders(&device,vertex_shader_path,fragment_shader_path,"vertex_sprite".to_string(),"fragment_sprite".to_string());

        let texture_bind_group_layout = group2_builder.create_texture_bind_group_layout(device);
        group2_builder.bind_groups_layouts.push(texture_bind_group_layout);
        group2_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group2_builder.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let color_buffer = crate::objects::matrix_helper::add_dynamic_buffer(device, (device.limits().min_uniform_buffer_offset_alignment * 256) as u64 /*100*/);

        let color_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: None },
            count: None
        }] });

        let color_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &color_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &color_buffer,
                    offset: 0 /*16*/,
                    size: BufferSize::new(16)
                }
            } }]
        });

        group2_builder.bind_groups_layouts.push(color_layout);

        let group2 = TextRenderPipelineGroup::new_with_shaders(group2_builder,config,device,VertexBuffer::desc(),PrimitiveTopology::TriangleList,depth);

        return TextRenderPipelineGroup { group: group2, color_buffer, color_group }
    }

    pub fn new_with_shaders(builder:RenderPipelineGroupBuilder,config:&SurfaceConfiguration,device:&Device,desc:VertexBufferLayout,topology:PrimitiveTopology,depth:bool) -> RenderPipelineGroup {

        let mut layouts: Vec<&BindGroupLayout> = vec![];

        let mut depth_stencil_state = None;

        if depth {
            depth_stencil_state = Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0
                },

            });
        }

        for l in builder.bind_groups_layouts.iter() {
            layouts.push(l);
        }
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &*layouts,
            push_constant_ranges: &[]
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &builder.vertex_shader.unwrap(),
                entry_point: "main",
                buffers: &[desc]
            },
            primitive: PrimitiveState {
                topology: topology,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: depth_stencil_state,
            fragment: Some(
                FragmentState {
                    module: &builder.fragment_shader.unwrap(),
                    entry_point: "main",
                    targets: &[Some(ColorTargetState {
                        format: config.format,
                        write_mask: ColorWrites::ALL,
                        blend: Some(BlendState {
                            color: BlendComponent {
                                src_factor: BlendFactor::SrcAlpha,
                                dst_factor: BlendFactor::OneMinusSrcAlpha,
                                operation: BlendOperation::Add
                            },
                            alpha: BlendComponent {
                                src_factor: BlendFactor::SrcAlpha,
                                dst_factor: BlendFactor::OneMinusSrcAlpha,
                                operation: BlendOperation::Add
                            }
                        })
                    })]
                }
            ),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });

        return RenderPipelineGroup {
            pipeline: render_pipeline,
            bind_groups: builder.bind_groups,
            buffers: builder.buffers,
            vertex_buffers: builder.vertex_buffers
        }
    }
}