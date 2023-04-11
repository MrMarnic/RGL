use std::time::Instant;
use log::info;
use wgpu::{Backends, CommandEncoder, Device, Dx12Compiler, InstanceDescriptor, LoadOp, Operations, PresentMode, Queue, RenderPass, RenderPassDepthStencilAttachment, Surface, SurfaceConfiguration, TextureView};
use winit::dpi::PhysicalSize;
use crate::audio::audio_handler::AudioHandler;
use crate::engine::game_window::GameWindow;
use crate::engine::input_handler::InputHandler;
use crate::engine::material_manager::MaterialManager;
use crate::engine::resource_loader::ResourceLoader;
use crate::objects::depth_texture::DepthTexture;
use crate::objects::offset_handler::{OffsetHandler, StaticOffsetHandler};
use crate::render::color_renderer::ColorRenderer;
use crate::render::custom_vertex_renderer::CustomVertexRenderer;
use crate::render::line_renderer::LineRenderer;
use crate::render::text_renderer::TextRenderer;
use crate::render::texture_renderer::TextureRenderer;
use crate::render::vertex_renderer::VertexRenderer;
use crate::scene::scene::Scene;
use crate::ui::bounds::Bounds;

pub struct GameEngine {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub game_window: GameWindow,
    pub scene_to_open: Option<Box<dyn Scene>>,
    pub offset_handler: OffsetHandler,
    pub working_dir: String,
    pub color_renderer: ColorRenderer,
    pub alignment_settings: AlignmentSettings,
    pub audio_handler: AudioHandler,
    pub static_offset_handler: StaticOffsetHandler,
    pub resource_loader: ResourceLoader,
    pub texture_renderer: TextureRenderer,
    pub input_handler: InputHandler,
    pub line_renderer: LineRenderer,
    pub custom_vertex_renderer: CustomVertexRenderer,
    pub text_renderer: TextRenderer,
    pub vertex_renderer: VertexRenderer,
    pub time: Instant,
    pub delta_time: f32,
    pub fps: i32,
    pub material_manager: MaterialManager,
    pub bounds: Bounds
}

impl GameEngine {
    pub async fn new(game_window:GameWindow, backend:Backends) -> GameEngine{
        let window = &game_window.window;

        let size = window.inner_size();

        let bounds = Bounds::new(0.0,0.0,size.width as f32,size.height as f32);

        let instance = wgpu::Instance::new(InstanceDescriptor { backends: backend, dx12_shader_compiler: Dx12Compiler::Fxc });
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        info!("{:?}",adapter.get_info());

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: game_window.get_present_mode(),
            alpha_mode: Default::default(),
            view_formats: vec![],
        };

        surface.configure(&device,&config);

        let wd = get_working_dir();

        let cr = ColorRenderer::new(wd.clone(),&device,&config);
        let texr = TextureRenderer::new(wd.clone(),&device,&config);
        let liner = LineRenderer::new(wd.clone(),&device,&config);
        let cvr = CustomVertexRenderer::new(wd.clone(),&device,&config);
        let textr = TextRenderer::new(wd.clone(),&device,&config);
        let vr = VertexRenderer::new(wd.clone(),&device,&config,&queue);

        let rsc_loader = ResourceLoader::new(&device);

        let a_settings = AlignmentSettings { uniform_offset: device.limits().min_uniform_buffer_offset_alignment, storage_offset: device.limits().min_storage_buffer_offset_alignment };

        return GameEngine {
            surface,
            device,
            queue,
            config,
            size,
            game_window,
            scene_to_open: None,
            offset_handler: OffsetHandler::new(),
            working_dir: wd,
            color_renderer: cr,
            alignment_settings: a_settings,
            audio_handler: AudioHandler::new(),
            static_offset_handler: StaticOffsetHandler::new(4900,100),
            resource_loader: rsc_loader,
            texture_renderer: texr,
            input_handler: InputHandler::new(),
            line_renderer: liner,
            custom_vertex_renderer: cvr,
            text_renderer: textr,
            vertex_renderer: vr,
            time: Instant::now(),
            delta_time: 0.0,
            fps: 0,
            material_manager: MaterialManager::new(),
            bounds
        }
    }

    pub fn create_render_pass<'a>(&self,encoder:&'a mut CommandEncoder,view:&'a TextureView) -> RenderPass<'a>{
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.game_window.clear_color.r as f64,
                            g: self.game_window.clear_color.g as f64,
                            b: self.game_window.clear_color.b as f64,
                            a: self.game_window.clear_color.a as f64,
                        }),
                        store: true,
                    }
                })
            ],
            depth_stencil_attachment: None,
        });

        return render_pass;
    }

    pub fn create_render_pass_with_depth<'a>(&self,encoder:&'a mut CommandEncoder,frame:&'a TextureView,depth_texture:&'a DepthTexture) -> RenderPass<'a>{
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: frame,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.game_window.clear_color.r as f64,
                            g: self.game_window.clear_color.g as f64,
                            b: self.game_window.clear_color.b as f64,
                            a: self.game_window.clear_color.a as f64,
                        }),
                        store: true,
                    }
                })
            ],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_texture.view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true
                }),
                stencil_ops: None
            }),
        });

        return render_pass;
    }

    pub fn create_render_pass_load<'a>(&self,encoder:&'a mut CommandEncoder,frame:&'a TextureView) -> RenderPass<'a>{
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: frame,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    }
                })
            ],
            depth_stencil_attachment: None,
        });

        return render_pass;
    }

    pub fn create_render_pass_load_with_depth<'a>(&self,encoder:&'a mut CommandEncoder,frame:&'a TextureView,depth_texture:&'a DepthTexture) -> RenderPass<'a>{
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: frame,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    }
                })
            ],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: &depth_texture.view,
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: true
            }),
            stencil_ops: None
        }),
        });

        return render_pass;
    }

    pub fn exit(&mut self) {
        self.game_window.should_close = true;
    }

    pub fn reset_render(&mut self) {
        self.offset_handler.line_offset = 0;
        self.offset_handler.camera_offset = 0;
        self.color_renderer.to_render.clear();
        self.texture_renderer.to_render.clear();
        self.line_renderer.to_render.clear();
        self.text_renderer.meshes_to_render.clear();
        self.text_renderer.color_offset = 0;
    }
}

fn get_working_dir() -> String {
    let working_dir_buf = std::env::current_dir().unwrap();
    let working_dir = working_dir_buf.to_str().unwrap();
    return working_dir.to_string();
}

pub struct AlignmentSettings {
    pub uniform_offset: u32,
    pub storage_offset: u32
}