use wgpu::{Device, Buffer, RenderPass, BufferAddress, Queue, IndexFormat, VertexBufferLayout, BufferUsages};
use crate::objects::vertex::{Vertex, OnlyCoordsVertex, NormalVertex};
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::objects::vertex_buffer_data::{VertexBufferData, NormalVertexBufferData};

pub struct VertexBuffer{
    pub vertecies: Vec<Vertex>,
    pub indecies: Vec<u32>,
    pub buffer: Buffer,
    pub index_buffer: Buffer
}

impl VertexBuffer {
    pub fn default_vertex_buffer(device:&Device,edit:bool) -> VertexBuffer {
        let vertecies = vec![Vertex::new(-1.0,1.0,0.0,0.0,0.0),Vertex::new(-1.0,-1.0,0.0,0.0,1.0),Vertex::new(1.0,-1.0,0.0,1.0,1.0),Vertex::new(1.0,1.0,0.0,1.0,0.0)];
        let indecies = vec![0,1,3,3,1,2];

        return VertexBuffer::new(device,vertecies,indecies,edit);
    }

    pub fn new_from_data(device:&Device,data:VertexBufferData) -> VertexBuffer{
        return VertexBuffer::new(device,data.vertecies,data.indecies,false);
    }

    pub fn new(device:&Device,vertecies: Vec<Vertex>,indecies:Vec<u32>,edit:bool) -> VertexBuffer {

        let mut bytes : Vec<u8> = vec![];

        for v in vertecies.iter() {
            bytes.extend_from_slice(v.x.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.y.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.z.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.u.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.v.to_le_bytes().as_ref());
        }

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &*bytes,
            usage: if !edit {BufferUsages::VERTEX} else {BufferUsages::VERTEX | BufferUsages::COPY_DST}
        });

        let mut bytes_index : Vec<u8> = vec![];

        for v in indecies.iter() {
            bytes_index.extend_from_slice(v.to_le_bytes().as_ref());
        }

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: &*bytes_index,
            usage: BufferUsages::INDEX
        });

        return VertexBuffer { vertecies, indecies, buffer, index_buffer };
    }

    pub fn desc() -> VertexBufferLayout<'static>{
        VertexBufferLayout {
            array_stride: (5 * std::mem::size_of::<f32>()) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[ // 3.
                wgpu::VertexAttribute {
                    offset: 0, // 4.
                    shader_location: 0, // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;3]>() as BufferAddress, // 4.
                    shader_location: 1, // 5.
                    format: wgpu::VertexFormat::Float32x2, // 6.
                }
            ]
        }
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0,self.buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..),IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.indecies.len() as u32,0,0..1);
    }

    pub fn edit_data(&self,vertecies: &Vec<Vertex>,queue:&Queue,offset:u64) {
        let mut bytes : Vec<u8> = vec![];

        for v in vertecies.iter() {
            bytes.extend_from_slice(v.x.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.y.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.z.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.u.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.v.to_le_bytes().as_ref());
        }
        queue.write_buffer(&self.buffer,offset,&*bytes);
    }
}

pub struct OnlyCoordsVertexBuffer{
    pub(crate) vertecies: Vec<OnlyCoordsVertex>,
    pub indecies: Vec<u32>,
    pub(crate) buffer: Buffer,
    pub index_buffer: Buffer
}

impl OnlyCoordsVertexBuffer {
    pub fn default_vertex_buffer(device:&Device,edit:bool) -> OnlyCoordsVertexBuffer {
        let vertecies = vec![OnlyCoordsVertex::new(-1.0,1.0,0.0),OnlyCoordsVertex::new(-1.0,-1.0,0.0),OnlyCoordsVertex::new(1.0,-1.0,0.0),OnlyCoordsVertex::new(1.0,1.0,0.0)];
        let indecies = vec![0,1,3,3,1,2];

        return OnlyCoordsVertexBuffer::new(device,vertecies,indecies,edit);
    }

    pub fn new(device:&Device,vertecies: Vec<OnlyCoordsVertex>,indecies:Vec<u32>,edit:bool) -> OnlyCoordsVertexBuffer {

        let mut bytes : Vec<u8> = vec![];

        for v in vertecies.iter() {
            bytes.extend_from_slice(v.x.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.y.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.z.to_le_bytes().as_ref());
        }

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &*bytes,
            usage: if !edit {BufferUsages::VERTEX} else {BufferUsages::VERTEX | BufferUsages::COPY_DST}
        });

        let mut bytes_index : Vec<u8> = vec![];

        for v in indecies.iter() {
            bytes_index.extend_from_slice(v.to_le_bytes().as_ref());
        }

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: &*bytes_index,
            usage: BufferUsages::INDEX
        });

        return OnlyCoordsVertexBuffer { vertecies, indecies, buffer, index_buffer };
    }

    pub fn desc() -> VertexBufferLayout<'static>{
        wgpu::VertexBufferLayout {
            array_stride: (3 * std::mem::size_of::<f32>()) as wgpu::BufferAddress, // 1.
            step_mode: wgpu::VertexStepMode::Vertex, // 2.
            attributes: &[ // 3.
                wgpu::VertexAttribute {
                    offset: 0, // 4.
                    shader_location: 0, // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                }
            ]
        }
    }

    pub fn edit_data(&self,data: &Vec<f32>,queue:&Queue) {
        let mut bytes : Vec<u8> = vec![];

        for v in data.iter() {
            bytes.extend_from_slice(v.to_le_bytes().as_ref());
        }
        queue.write_buffer(&self.buffer,0,&*bytes);
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0,self.buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..),IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.indecies.len() as u32,0,0..1);
    }
}

pub struct NormalVertexBuffer{
    pub(crate) vertecies: Vec<NormalVertex>,
    pub indecies: Vec<u32>,
    pub(crate) buffer: Buffer,
    pub index_buffer: Buffer
}

impl NormalVertexBuffer {
    pub fn default_vertex_buffer(device:&Device,edit:bool) -> NormalVertexBuffer {
        let vertecies = vec![NormalVertex::new(-1.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0),NormalVertex::new(-1.0,-1.0,0.0,0.0,1.0,0.0,0.0,0.0),NormalVertex::new(1.0,-1.0,0.0,1.0,1.0,0.0,0.0,0.0),NormalVertex::new(1.0,1.0,0.0,1.0,0.0,0.0,0.0,0.0)];
        let indecies = vec![0,1,3,3,1,2];

        return NormalVertexBuffer::new(device,vertecies,indecies,edit);
    }

    pub fn new_from_data(device:&Device,data:NormalVertexBufferData) -> NormalVertexBuffer{
        return NormalVertexBuffer::new(device,data.vertecies,data.indecies,false);
    }

    pub fn new(device:&Device,vertecies: Vec<NormalVertex>,indecies:Vec<u32>,edit:bool) -> NormalVertexBuffer {

        let mut bytes : Vec<u8> = vec![];

        for v in vertecies.iter() {
            bytes.extend_from_slice(v.x.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.y.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.z.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.u.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.v.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.n_x.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.n_y.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.n_z.to_le_bytes().as_ref());
        }

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &*bytes,
            usage: if !edit {BufferUsages::VERTEX} else {BufferUsages::VERTEX | BufferUsages::COPY_DST}
        });

        let mut bytes_index : Vec<u8> = vec![];

        for v in indecies.iter() {
            bytes_index.extend_from_slice(v.to_le_bytes().as_ref());
        }

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: &*bytes_index,
            usage: BufferUsages::INDEX
        });

        return NormalVertexBuffer { vertecies, indecies, buffer, index_buffer };
    }

    pub fn desc() -> VertexBufferLayout<'static>{
        wgpu::VertexBufferLayout {
            array_stride: (8 * std::mem::size_of::<f32>()) as wgpu::BufferAddress, // 1.
            step_mode: wgpu::VertexStepMode::Vertex, // 2.
            attributes: &[ // 3.
                wgpu::VertexAttribute {
                    offset: 0, // 4.
                    shader_location: 0, // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;3]>() as BufferAddress, // 4.
                    shader_location: 1, // 5.
                    format: wgpu::VertexFormat::Float32x2, // 6.
                },wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;5]>() as BufferAddress, // 4.
                    shader_location: 2, // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                }
            ]
        }
    }

    pub fn edit_data(&self,data: &Vec<f32>,queue:&Queue) {
        let mut bytes : Vec<u8> = vec![];

        for v in data.iter() {
            bytes.extend_from_slice(v.to_le_bytes().as_ref());
        }
        queue.write_buffer(&self.buffer,0,&*bytes);
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0,self.buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..),IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.indecies.len() as u32,0,0..1);
    }
}