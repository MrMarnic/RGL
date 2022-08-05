use nalgebra_glm::{TMat4, TVec3, TVec4, TVec2};
use wgpu::{Device, Buffer, BufferDescriptor, BufferAddress, BufferUsages};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub fn get_bytes(matrix:&TMat4<f32>) -> Vec<u8> {
    let data = matrix.as_slice();
    let mut dst = vec![];

    for f in data {
        dst.extend_from_slice(f.to_le_bytes().as_ref());
    }

    return dst;
}

pub fn get_bytes_from_vec3(matrix:&TVec3<f32>) -> Vec<u8> {
    let data = matrix.as_slice();
    let mut dst = vec![];

    for f in data {
        dst.extend_from_slice(f.to_le_bytes().as_ref());
    }

    return dst;
}

pub fn get_bytes_from_vec2(matrix:&TVec2<f32>) -> Vec<u8> {
    let data = matrix.as_slice();
    let mut dst = vec![];

    for f in data {
        dst.extend_from_slice(f.to_le_bytes().as_ref());
    }

    return dst;
}

pub fn get_bytes_from_vec(matrix:Vec<f32>) -> Vec<u8> {
    let data = matrix.as_slice();
    let mut dst = vec![];

    for f in data {
        dst.extend_from_slice(f.to_le_bytes().as_ref());
    }

    return dst;
}

pub fn get_bytes_from_f32(matrix:f32) -> Vec<u8> {
    let mut dst = vec![];
    dst.extend_from_slice(matrix.to_le_bytes().as_ref());
    return dst;
}

pub fn get_bytes_from_i32(matrix:i32) -> Vec<u8> {
    let mut dst = vec![];
    dst.extend_from_slice(matrix.to_le_bytes().as_ref());
    return dst;
}

pub fn get_bytes_from_vec_ref(matrix:&Vec<f32>) -> Vec<u8> {
    let data = matrix.as_slice();
    let mut dst = vec![];

    for f in data {
        dst.extend_from_slice(f.to_le_bytes().as_ref());
    }

    return dst;
}

pub fn get_bytes_from_vec4(matrix:&TVec4<f32>) -> Vec<u8> {
    let data = matrix.as_slice();
    let mut dst = vec![];

    for f in data {
        dst.extend_from_slice(f.to_le_bytes().as_ref());
    }

    return dst;
}

pub fn add_buffer(device:&Device,data:&[u8]) -> Buffer{
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: data,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });
    return buffer;
}

pub fn add_dynamic_buffer(device:&Device,size:BufferAddress) -> Buffer{
    let buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false
    });
    return buffer;
}