use std::ffi::CString;
use std::fs::File;
use std::io::Write;

use nalgebra_glm::{TVec2, vec2};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use wgpu::{Device, TextureDescriptor, TextureDimension, TextureFormat, Extent3d, Queue, Origin3d, BindGroup, BindGroupDescriptor, BindGroupLayout, TextureViewDescriptor, SamplerDescriptor, AddressMode, FilterMode, CompareFunction, ImageCopyTexture, ImageDataLayout, TextureAspect, TextureUsages, TextureViewDimension};
use image::{RgbaImage, RgbImage, ImageBuffer, Rgba, GenericImageView};
use std::rc::Rc;
use std::num::NonZeroU32;
use std::convert::TryFrom;
use wgpu::util::StagingBelt;

pub struct TextureObject {
    pub texture: wgpu::Texture,
    pub size: Extent3d,
    pub name: String,
    pub bind_group: BindGroup,
    pub data: Vec<u8>,
    pub layout: ImageDataLayout
}

impl TextureObject{
    pub fn new(path : String, name : String,device:&Device,queue:&Queue,layout:&BindGroupLayout) -> TextureObject{
        let image = image::open(path).unwrap();

        let rgba = image.as_rgba8().unwrap();

        let dimensions = rgba.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(name.as_str()),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
        });

        let data_layout = ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(4 * size.width as u32),
            rows_per_image: NonZeroU32::new(size.height as u32)
        };

        queue.write_texture(ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All
        }, &rgba, data_layout.clone(), size);

        let diffuse_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                }]
        });

        return TextureObject {
            texture,
            size,
            name,
            bind_group,
            data: rgba.to_vec(),
            layout: data_layout
        };
    }

    pub fn update(&self,queue:&Queue,image:&ImageBuffer<Rgba<u8>,Vec<u8>>) {
        let data_layout = ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(4 * self.size.width as u32),
            rows_per_image: NonZeroU32::new(self.size.height as u32)
        };

        queue.write_texture(ImageCopyTexture {
            texture: &self.texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All
        }, &image.to_vec(), data_layout, self.size.clone());
    }

    pub fn new_from_data(image:&ImageBuffer<Rgba<u8>,Vec<u8>>,name : String,device:&Device,queue:&Queue,layout:&BindGroupLayout) -> TextureObject{
        let data = image.to_vec();

        let size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(name.as_str()),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
        });

        let data_layout = ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(4 * size.width as u32),
            rows_per_image: NonZeroU32::new(size.height as u32)
        };

        queue.write_texture(ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All
        }, &data, data_layout.clone(), size);

        let diffuse_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                }]
        });

        return TextureObject {
            texture,
            size,
            name,
            bind_group,
            data,
            layout: data_layout
        };
    }
}