use std::collections::HashMap;

use std::rc::Rc;
use crate::audio::audio_source::AudioSource;
use crate::audio::mp3_file::MP3File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use nalgebra_glm::pi;
use rgl_font::{BaseFont, FontLoader, ScaledFont};
use crate::objects::texture_object::TextureObject;
use wgpu::{Device, Queue, BindGroupLayout, Buffer};
use crate::engine::game_engine::GameEngine;
use crate::engine::material_manager::MaterialManager;
use crate::objects::obj_model::{Material, ObjModel};
use crate::pipeline::pipeline::RenderPipelineGroupBuilder;
use crate::text::render_font::RenderFont;

pub struct ResourceLoader {
    pub textures: HashMap<String,Rc<TextureObject>>,
    pub base_fonts: HashMap<String,Rc<BaseFont>>,
    pub scaled_fonts: HashMap<String,Rc<ScaledFont>>,
    pub render_fonts: HashMap<String,Rc<RenderFont>>,
    pub objs: HashMap<String,Rc<ObjModel>>,
    pub base_layout: Arc<BindGroupLayout>,
    font_loader: FontLoader
}

impl ResourceLoader {
    pub fn new(device:&Device) -> ResourceLoader{
        let loader = ResourceLoader {textures: HashMap::new(), base_fonts: HashMap::new(), scaled_fonts: HashMap::new(),render_fonts: HashMap::new(), objs: HashMap::new(), base_layout: Arc::new(RenderPipelineGroupBuilder::empty().create_texture_bind_group_layout(device)), font_loader: FontLoader::new() };
        return loader;
    }

    pub fn load_all_textures_in_folder(&mut self,folder: String, device:&Device, queue:&Queue, working_dir:String) {
        let paths = std::fs::read_dir(format!("{}//{}",working_dir,folder)).unwrap();

        for path in paths {
            let file = path.unwrap();
            if file.path().is_file() {
                if file.path().extension().unwrap().to_str().unwrap().eq("png") {
                    let file_name = String::from(file.file_name().to_str().unwrap());
                    unsafe {
                        self.textures.insert(file_name.clone(),Rc::new(TextureObject::new(format!("{}//{}//{}",working_dir,&folder,&file_name),file_name.to_string(),device,queue,&self.base_layout)));
                    }
                }
            }
        }
    }

    pub fn load_all_objs_in_folder(&mut self,folder: String, working_dir:String, device:&Device, m_manager:&mut MaterialManager, buffer:&Buffer, queue:&Queue) {
        let paths = std::fs::read_dir(format!("{}//{}",working_dir,folder)).unwrap();

        for path in paths {
            let file = path.unwrap();
            if file.path().is_file() {
                if file.path().extension().unwrap().to_str().unwrap().eq("obj") {
                    let file_name = String::from(file.file_name().to_str().unwrap());
                    unsafe {
                        let mut obj = ObjModel::new(format!("{}//{}",working_dir,&folder),file_name.clone(),device);
                        m_manager.register(&mut obj.materials, buffer, queue,self);

                        self.objs.insert(file_name.clone(),Rc::new(obj));
                    }
                }
            }
        }
    }

    pub fn load_font(&mut self, path: String, name: String, working_dir:String) {
        self.base_fonts.insert(name.clone(),Rc::new(self.font_loader.load_font(name.as_str(),format!("{}\\{}",working_dir,path).as_str())));
    }

    pub fn load_scaled_font(&mut self,name:String, base:Rc<BaseFont>, pixel_height:i32, device:&Device,queue:&Queue) {

        let s = Rc::new(base.scaled(pixel_height));

        let render_font = RenderFont::new(s.clone(),device,queue,&self.base_layout);

        self.scaled_fonts.insert(name.clone(),s);
        self.render_fonts.insert(name.clone(),Rc::new(render_font));
    }

    pub fn get_texture(&self, id: String) -> Rc<TextureObject>{
        return self.textures.get(&id).unwrap().clone();
    }
}