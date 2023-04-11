use std::borrow::Cow;
use wgpu::{Device, PipelineLayoutDescriptor, RenderPipelineDescriptor, PrimitiveTopology, FrontFace, IndexFormat, RenderPipeline, BindGroupDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BindGroupEntry, BindingResource, BindGroup, Buffer, ShaderModule, BindGroupLayout, BufferDescriptor, PUSH_CONSTANT_ALIGNMENT, Features, PushConstantRange, Limits, TextureFormat, CompareFunction, ShaderModuleDescriptor, VertexState, FragmentState, ColorTargetState, BlendState, BlendFactor, BlendOperation, PrimitiveState, PolygonMode, DepthStencilState, DepthBiasState, MultisampleState, VertexBufferLayout, StencilState, BlendComponent, Face, ShaderSource, BufferUsages, ColorWrites, SurfaceConfiguration, SamplerBindingType, ShaderStages};
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::objects::vertex_buffer::VertexBuffer;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{Read, Write};
use log::info;
use naga::ShaderStage;
use crate::engine::game_engine::GameEngine;

pub struct RenderPipelineGroupBuilder {
    pub bind_groups: Vec<BindGroup>,
    pub bind_groups_layouts: Vec<BindGroupLayout>,
    pub buffers: Vec<Buffer>,
    pub vertex_buffers: Vec<VertexBuffer>,
    pub vertex_shader: Option<ShaderModule>,
    pub fragment_shader: Option<ShaderModule>
}

impl RenderPipelineGroupBuilder {
    pub fn empty() -> RenderPipelineGroupBuilder {
        return RenderPipelineGroupBuilder {
            bind_groups: vec![],
            bind_groups_layouts: vec![],
            buffers: vec![],
            vertex_buffers: vec![],
            vertex_shader: None,
            fragment_shader: None
        }
    }

    pub fn set_shaders(&mut self,device:&Device,vertex_shader_path : String, fragment_shader_path: String,name_vertex:String,name_fragment:String) {
        info!("Loading Vertex Shader on path: {}",vertex_shader_path);
        let vs_src = std::fs::read_to_string(vertex_shader_path.clone()).unwrap();
        let fs_src = std::fs::read_to_string(fragment_shader_path.clone()).unwrap();

        let vs_module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Glsl {
                shader: Cow::from(vs_src.as_str()),
                stage: ShaderStage::Vertex,
                defines: Default::default()
            }
        });

        let fs_module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Glsl {
                shader: Cow::from(fs_src.as_str()),
                stage: ShaderStage::Fragment,
                defines: Default::default()
            }
        });


        self.vertex_shader = Some(vs_module);
        self.fragment_shader = Some(fs_module);
    }

    pub fn add_bind_group(&mut self,bind_group:BindGroup,layout:BindGroupLayout) {
        self.bind_groups.push(bind_group);
        self.bind_groups_layouts.push(layout);
    }

    pub fn add_buffer(&mut self,device:&Device,data:&[u8]) {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        self.buffers.push(buffer);
    }

    pub fn add_uniform_buffer(&mut self,device:&Device,data:&[u8]) {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        self.buffers.push(buffer);
    }

    pub fn add_index_buffer(&mut self,device:&Device,data:&[u8]) {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data,
            usage: BufferUsages::INDEX
        });
        self.buffers.push(buffer);
    }

    pub fn create_texture_bind_group_layout(&self,device:&Device) -> BindGroupLayout {
        return device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            0: SamplerBindingType::Filtering
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        );
    }

    pub fn build(self,device:&Device,config:&SurfaceConfiguration,desc:VertexBufferLayout,topology:PrimitiveTopology,depth:bool,name:String) -> RenderPipelineGroup {
        return RenderPipelineGroup::new_with_shaders(self,device,config,desc,topology,depth,name);
    }
}

pub struct RenderPipelineGroup {
    pub pipeline:RenderPipeline,
    pub bind_groups: Vec<BindGroup>,
    pub buffers: Vec<Buffer>,
    pub vertex_buffers: Vec<VertexBuffer>
}

impl RenderPipelineGroup {
    pub fn new_with_shaders(builder:RenderPipelineGroupBuilder,device:&Device,config:&SurfaceConfiguration,desc:VertexBufferLayout,topology:PrimitiveTopology,depth:bool, name:String) -> RenderPipelineGroup {

        let mut layouts: Vec<&BindGroupLayout> = vec![];

        for l in builder.bind_groups_layouts.iter() {
            layouts.push(l);
        }
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &*layouts,
            push_constant_ranges: &[]
        });

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

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(name.as_str()),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &builder.vertex_shader.unwrap(),
                entry_point: "main",
                buffers: &[desc]
            },
            primitive: PrimitiveState {
                topology,
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