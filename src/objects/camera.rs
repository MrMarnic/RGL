use nalgebra::{Point, Point3, ArrayStorage, Perspective3, Orthographic3, TProjective};
use nalgebra_glm::{Mat4, TMat4, TVec3, vec3, vec4};
use crate::objects::transform::Transform;
use wgpu::{Queue, Buffer, Device, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BindGroup, BindGroupLayout, BindGroupDescriptor, BindGroupEntry, BindingResource, BufferDescriptor, BufferSize, BufferSlice, BufferBindingType, BufferBinding, BufferUsages, ShaderStages, BufferAddress};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::engine::game_engine::GameEngine;

pub struct Camera {
    pub fov: f32,
    pub width : f32,
    pub height : f32,
    pub near : f32,
    pub far : f32,
    pub transform: Transform,
    pub projection : TMat4<f32>,
    pub view : Mat4,
    pub direction: TVec3<f32>,
    pub target: TVec3<f32>,
    pub orto: bool,
    pub buffers: Vec<Buffer>,
    pub bind_group: BindGroup,
    pub transform_bind_group: BindGroup
}

impl Camera {

    fn create_orto_projection(width: f32, height:f32) -> TMat4<f32> {
        return nalgebra_glm::ortho(0.0, width, 0.0, height, -1.0, 1.0);
    }

    pub fn new_perspective(fov: f32, width : f32, height: f32,near : f32,far : f32, position : TVec3<f32>,device:&Device) -> Camera{
        let trans = Transform::new(position.x,position.y,position.z,vec3(1.0,1.0,1.0));
        let projection = nalgebra_glm::perspective_fov(fov,width,height,near,far);
        let view = nalgebra_glm::translation(&nalgebra_glm::vec3(0.0,0.0,0.0));

        let mut buffers = vec![];

        Camera::add_buffer(&mut buffers,device,&*crate::objects::matrix_helper::get_bytes(&projection));
        Camera::add_buffer(&mut buffers,device,&*crate::objects::matrix_helper::get_bytes(&view));
        //Camera::add_buffer(&mut buffers,device,&*crate::objects::matrix_helper::get_bytes(&nalgebra_glm::translation(&vec3(0.0,0.0,0.0))));
        Camera::add_dynamic_buffer(&mut buffers,device);

        let camera_layout = Camera::bind_group_layout(device);

        let camera_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &camera_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &buffers[0],
                    offset: 0,
                    size: None
                },
            }
            }, BindGroupEntry { binding: 1, resource: BindingResource::Buffer{
                0: BufferBinding {
                    buffer: &buffers[1],
                    offset: 0,
                    size: None
                }
                },
                },]
        });

        let transform_layout = Camera::transform_bind_group(device);

        let transform_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("transformPers"),
            layout: &transform_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &buffers[2],
                    offset: 0 /*64*/,
                    size: BufferSize::new(64)
                }
            }}]
        });

        let mut camera = Camera {
            fov,
            width,
            height,
            near,
            far,
            transform: trans,
            projection,
            view,
            direction: nalgebra_glm::vec3(0.0,0.0,1.0),
            target: vec3(0.0,0.0,1.0),
            orto: false,
            buffers,
            bind_group: camera_group,
            transform_bind_group: transform_group
        };
        camera.update();
        return camera;
    }

    pub fn new_orto(width: i32, height: i32,position : TVec3<f32>,device:&Device) -> Camera{

        let trans = Transform::new(position.x,position.y,position.z,vec3(1.0,1.0,1.0));

        let projection=  Camera::create_orto_projection(width as f32,height as f32);
        let view =  nalgebra_glm::translation(&nalgebra_glm::vec3(0.0,0.0,1.0));

        let mut buffers = vec![];

        Camera::add_buffer(&mut buffers,device,&*crate::objects::matrix_helper::get_bytes(&projection));
        Camera::add_buffer(&mut buffers,device,&*crate::objects::matrix_helper::get_bytes(&view));
        //Camera::add_buffer(&mut buffers,device,&*crate::objects::matrix_helper::get_bytes(&nalgebra_glm::translation(&vec3(0.0,0.0,0.0))));
        Camera::add_dynamic_buffer(&mut buffers,device);

        let camera_layout = Camera::bind_group_layout(device);

        let camera_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &camera_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &buffers[0],
                    offset: 0,
                    size: None
                }
            } },
                BindGroupEntry { binding: 1, resource: BindingResource::Buffer {
                    0: BufferBinding {
                        buffer: &buffers[1],
                        offset: 0,
                        size: None
                    }
                } },]
        });

        let transform_layout = Camera::transform_bind_group(device);

        let transform_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("transformOrto"),
            layout: &transform_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer {
                0: BufferBinding {
                    buffer: &buffers[2],
                    offset: 0 /*64*/,
                    size: BufferSize::new(64)
                }
            }}]
        });

        let mut camera = Camera {
            fov: 0.0,
            width: width as f32,
            height: height as f32,
            near: -1.0,
            far: 1.0,
            transform: trans,
            projection,
            view,
            direction: nalgebra_glm::vec3(0.0,0.0,1.0),
            target: vec3(0.0,0.0,1.0),
            orto: true,
            buffers,
            bind_group: camera_group,
            transform_bind_group: transform_group
        };

        camera.update();
        return camera;
    }

    pub fn unproject(&self, pos:&TVec3<f32>, engine:&GameEngine) -> TVec3<f32> {
        return nalgebra_glm::unproject(pos,&self.view,&self.projection,vec4(0.0,0.0,engine.size.width as f32, engine.size.height as f32));
    }

    pub fn project(&self, pos:&TVec3<f32>, engine:&GameEngine) -> TVec3<f32> {
        return nalgebra_glm::project(pos,&self.view,&self.projection,vec4(0.0,0.0,engine.size.width as f32, engine.size.height as f32));
    }

    pub fn load_up(&self, queue : &Queue) {
        queue.write_buffer(&self.buffers[0],0,&*crate::objects::matrix_helper::get_bytes(&self.projection));
        queue.write_buffer(&self.buffers[1],0,&*crate::objects::matrix_helper::get_bytes(&self.view));
    }

    pub fn update_projection(&mut self, fov: f32, width: f32,height : f32, near : f32, far : f32 ) {
        self.fov = fov;
        self.width = width;
        self.height = height;
        self.near = near;
        self.far = far;
        if !self.orto {
            self.projection = nalgebra_glm::perspective_fov(fov,width,height,near,far);
        }else {
            self.projection = Camera::create_orto_projection(width,height);
        }
    }

    pub fn update_aspect_with_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        if !self.orto {
            self.projection = nalgebra_glm::perspective_fov(self.fov,self.width,self.height,self.near,self.far);
        }else {
            self.projection = Camera::create_orto_projection(width,height);
        }
    }

    pub fn update_aspect(&mut self) {
        if !self.orto {
            self.projection = nalgebra_glm::perspective_fov(self.fov as f32,self.width as f32,self.height as f32,self.near,self.far);
        }else {
            self.projection = Camera::create_orto_projection(self.width,self.height);
        }
    }

    pub fn update(&mut self) {
        self.view = self.create_view_matrix();
    }
    
    fn create_view_matrix(&self) -> Mat4{
        let mut view = nalgebra_glm::rotation(self.transform.to_radians(self.transform.pitch) as f32,&nalgebra_glm::vec3(1.0,0.0,0.0));
        view = nalgebra_glm::rotate(&view,self.transform.to_radians(self.transform.yaw) as f32,&nalgebra_glm::vec3(0.0,1.0,0.0));
        view = nalgebra_glm::rotate(&view,self.transform.to_radians(self.transform.roll) as f32,&nalgebra_glm::vec3(0.0,0.0,1.0));

        let neg_pos = nalgebra_glm::vec3(-self.transform.pos.x as f32,-self.transform.pos.y as f32,-self.transform.pos.z as f32);

        view = nalgebra_glm::translate(&view,&neg_pos);

        return view;
    }

    pub fn add_buffer(buffers:&mut Vec<Buffer>,device:&Device,data:&[u8]) {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        buffers.push(buffer);
    }

    pub fn add_dynamic_buffer(buffers:&mut Vec<Buffer>,device:&Device) {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: (device.limits().min_storage_buffer_offset_alignment * 5000) as u64/*5000*/,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        buffers.push(buffer);
    }

    pub fn bind_group_layout(device:&Device) -> BindGroupLayout {
        return device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry{
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform, min_binding_size: BufferSize::new(64), has_dynamic_offset: false },
            count: None
        },BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform,has_dynamic_offset: false, min_binding_size: BufferSize::new(64) },
            count: None
        }] });
    }

    pub fn transform_bind_group(device:&Device) -> BindGroupLayout {
        return device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer { ty: BufferBindingType::Storage {
                read_only: true
            },has_dynamic_offset: true, min_binding_size: BufferSize::new(64) },
            count: None
        }] });
    }

    pub fn clone(&self, device:&Device) -> Camera {
        if self.orto {
            return Camera::new_orto(self.width as i32,self.height as i32,self.transform.pos.clone(),device);
        } else {
            return Camera::new_perspective(self.fov,self.width,self.height,self.near,self.far,self.transform.pos.clone(),device);
        }
    }
}