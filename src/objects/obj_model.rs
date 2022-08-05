use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use std::u32;
use nalgebra_glm::{TVec3, vec3, vec4};
use wgpu::{Buffer, Device, Queue, RenderPass};
use crate::engine::game_engine::GameEngine;
use crate::engine::resource_loader::ResourceLoader;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use crate::objects::texture_object::TextureObject;
use crate::objects::vertex::{NormalVertex, Vertex};
use crate::objects::vertex_buffer::{NormalVertexBuffer, VertexBuffer};
use crate::objects::vertex_buffer_builder::VertexBufferBuilder;

pub struct ObjModel {
    pub meshes: HashMap<String,Vec<NormalVertexBuffer>>,
    pub materials: HashMap<String,Material>
}

impl ObjModel {
    pub fn new(parent:String,file:String,device:&Device) -> ObjModel {
        let data = std::fs::read(format!("{}//{}", parent, file)).unwrap();
        let txt = String::from_utf8(data).unwrap();

        let mut lines = txt.split("\n").collect::<Vec<&str>>();

        let mut positions = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];

        let mut vertecies = vec![];
        let mut i_map = HashMap::new();
        let mut indecies = vec![];
        let mut current_index = 0;
        let mut materials = HashMap::new();

        let mut current_mat = String::new();
        let mut meshes = HashMap::new();

        for mut l in lines {
            l = l.trim();
            if l.starts_with("mtllib ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();
                let name = v_data[1];

                materials = Material::map_from_mat_file(parent.clone(),name.to_string(),device);
            }else if l.starts_with("v ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();
                positions.push((ObjModel::read_f32(v_data[1]),ObjModel::read_f32(v_data[2]),ObjModel::read_f32(v_data[3])));
            } else if l.starts_with("vt ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();
                uvs.push((ObjModel::read_f32(v_data[1]),ObjModel::read_f32(v_data[2])));
            }else if l.starts_with("vn ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();
                normals.push((ObjModel::read_f32(v_data[1]),ObjModel::read_f32(v_data[2]),ObjModel::read_f32(v_data[3])));
            }else if l.starts_with("usemtl ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();
                let name = v_data[1];

                if !current_mat.is_empty() {
                    let mesh = NormalVertexBuffer::new(device,vertecies,indecies,false);
                    vertecies = vec![];
                    indecies = vec![];
                    i_map.clear();
                    current_index = 0;

                    if !meshes.contains_key(&current_mat) {
                        meshes.insert(current_mat.clone(), vec![mesh]);
                    } else {
                        meshes.get_mut(&current_mat).unwrap().push(mesh);
                    }
                }

                current_mat = name.to_string();
            }else if l.starts_with("f ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();
                for i in 1..4 {
                    let f_data = v_data[i].split("/").collect::<Vec<&str>>();

                    if !i_map.contains_key(v_data[i]) {
                        let index_pos = ObjModel::read_i32(f_data[0]) - 1;

                        let mut index_uv = -1;
                        let mut u = 0.0;
                        let mut v = 0.0;
                        if !f_data[1].is_empty() {
                            index_uv = ObjModel::read_i32(f_data[1]) - 1;
                            u = uvs[index_uv as usize].0;
                            v = uvs[index_uv as usize].1;
                        }
                        let index_n = ObjModel::read_i32(f_data[2]) - 1;

                        vertecies.push(NormalVertex::new(positions[index_pos as usize].0,positions[index_pos as usize].1,positions[index_pos as usize].2,
                                                         u,v,normals[index_n as usize].0,normals[index_n as usize].1,normals[index_n as usize].2));
                        i_map.insert(v_data[i],current_index);

                        indecies.push(current_index);
                        current_index+=1;
                    } else {
                        indecies.push(i_map[&v_data[i]]);
                    }
                }
            }
        }

        if !vertecies.is_empty() {
            if !meshes.contains_key(&current_mat) {
                meshes.insert(current_mat.clone(), vec![NormalVertexBuffer::new(device,vertecies,indecies,false)]);
            } else {
                meshes.get_mut(&current_mat).unwrap().push(NormalVertexBuffer::new(device,vertecies,indecies,false));
            }
        }

        if materials.is_empty() {
            materials.insert("".to_string(),Material::default());
        }

        return ObjModel { meshes, materials }
    }

    pub fn read_f32(str:&str) -> f32 {
        return f32::from_str(str).unwrap();
    }

    pub fn read_i32(str:&str) -> i32 {
        return i32::from_str(str).unwrap();
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera, offset: u32, engine:&'a GameEngine) {
        for (id,m) in self.meshes.iter() {
            let material = &self.materials[id];
            for model in m {
                if let Some(texture) = &material.texture_obj {
                    engine.vertex_renderer.render(render_pass,camera,texture,offset,model,material.offset);
                } else {
                    engine.vertex_renderer.render(render_pass,camera,&engine.vertex_renderer.default_tex,offset,model,material.offset);
                }
            }
        }
    }
}

pub struct Material {
    pub name: String,
    pub ambient_color: TVec3<f32>,
    pub diffuse_color: TVec3<f32>,
    pub specular_color: TVec3<f32>,
    pub specular_highlights: f32,
    pub optical_density: f32,
    pub dissolve: f32,
    pub illumination_model: i32,
    pub texture_name: Option<String>,
    pub texture_obj: Option<Rc<TextureObject>>,
    pub data: Vec<u8>,
    pub offset: u32
}

impl Material {
    pub fn default() -> Material {
        let mut m =  Material {
            name: "".to_string(),
            ambient_color: vec3(1.0,1.0,1.0),
            diffuse_color: vec3(1.0,1.0,1.0),
            specular_color: vec3(1.0,1.0,1.0),
            specular_highlights: 0.0,
            optical_density: 0.0,
            dissolve: 0.0,
            illumination_model: 0,
            texture_name: None,
            texture_obj: None,
            data: vec![],
            offset: 0
        };

        m.data = m.get_data();

        return m;
    }

    pub fn map_from_mat_file(parent:String,file:String,device:&Device) -> HashMap<String,Material> {
        let data = std::fs::read(format!("{}//{}", parent, file)).unwrap();
        let txt = String::from_utf8(data).unwrap();

        let mut lines = txt.split("\n").collect::<Vec<&str>>();

        let mut current_mat = Material::default();
        let mut mats = HashMap::new();

        for mut l in lines {
            l = l.trim();

            if l.starts_with("newmtl ") {
                if !current_mat.name.is_empty() {
                    current_mat.data = current_mat.get_data();
                    mats.insert(current_mat.name.clone(),current_mat);
                    current_mat = Material::default();
                }

                let v_data = l.split(" ").collect::<Vec<&str>>();
                current_mat.name = v_data[1].to_string();
            }else if l.starts_with("Ka ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                let vec = vec3(ObjModel::read_f32(v_data[1]),ObjModel::read_f32(v_data[2]),ObjModel::read_f32(v_data[3]));

                current_mat.ambient_color = vec;
            }else if l.starts_with("Kd ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                let vec = vec3(ObjModel::read_f32(v_data[1]),ObjModel::read_f32(v_data[2]),ObjModel::read_f32(v_data[3]));

                current_mat.diffuse_color = vec;
            }else if l.starts_with("Ks ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                let vec = vec3(ObjModel::read_f32(v_data[1]),ObjModel::read_f32(v_data[2]),ObjModel::read_f32(v_data[3]));

                current_mat.specular_color = vec;
            }else if l.starts_with("Ns ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                current_mat.specular_highlights = ObjModel::read_f32(v_data[1]);
            }else if l.starts_with("Ni ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                current_mat.optical_density = ObjModel::read_f32(v_data[1]);
            }else if l.starts_with("d ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                current_mat.dissolve = ObjModel::read_f32(v_data[1]);
            }else if l.starts_with("illum ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                current_mat.illumination_model = ObjModel::read_i32(v_data[1]);
            }else if l.starts_with("map_Kd ") {
                let v_data = l.split(" ").collect::<Vec<&str>>();

                current_mat.texture_name = Some(v_data[1].to_string());
            }
        }

        if !current_mat.name.is_empty() {
            current_mat.data = current_mat.get_data();
            mats.insert(current_mat.name.clone(),current_mat);
        }

        return mats;
    }

    pub fn register(&mut self, queue:&Queue, buffer:&Buffer, rsc:&ResourceLoader) {
        queue.write_buffer(buffer, self.offset as u64, &self.data);
        if let Some(name) = &self.texture_name{
            let tex = rsc.textures[name].clone();
            self.texture_obj = Some(tex);
        }
    }

    fn get_data(&self) -> Vec<u8> {
        let mut data2 = vec![];

        data2.extend(crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(self.ambient_color.x,self.ambient_color.y,self.ambient_color.z,0.0)));
        data2.extend(crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(self.diffuse_color.x,self.diffuse_color.y,self.diffuse_color.z,0.0)));
        data2.extend(crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(self.specular_color.x,self.specular_color.y,self.specular_color.z,0.0)));
        data2.extend(crate::objects::matrix_helper::get_bytes_from_f32(self.specular_highlights));
        data2.extend(crate::objects::matrix_helper::get_bytes_from_f32(self.optical_density));
        data2.extend(crate::objects::matrix_helper::get_bytes_from_f32(self.dissolve));
        data2.extend(crate::objects::matrix_helper::get_bytes_from_f32(0.0));
        return data2;
    }
}