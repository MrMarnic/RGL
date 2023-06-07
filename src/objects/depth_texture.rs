use wgpu::{Device, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureViewDescriptor, SamplerDescriptor, AddressMode, FilterMode, CompareFunction, Texture, TextureView, Sampler, TextureUsages, SurfaceConfiguration};

pub struct DepthTexture {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler
}

impl DepthTexture {
    pub fn new(device:&Device,config:&SurfaceConfiguration) -> DepthTexture{
        let size = Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1
        };

        let desc = TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        let texture = device.create_texture(&desc);

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0,
            lod_max_clamp: 100.0,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });

        return DepthTexture {
            texture,
            view,
            sampler
        }
    }
}