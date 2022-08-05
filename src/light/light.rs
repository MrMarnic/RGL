use nalgebra_glm::{TVec3, TVec4, vec3, vec3_to_vec4, vec4};
use crate::engine::game_engine::GameEngine;
use crate::objects::color::Color;

pub struct Lights {
    pub lights: [Light;4],
    pub data: Vec<u8>,
    pub num_lights: i32
}

impl Lights {
    pub fn new() -> Self {
        let mut ls = Lights { lights: [Light::empty(),Light::empty(),Light::empty(),Light::empty()], data: vec![], num_lights: 0 };
        ls.data = ls.get_data();

        return ls;
    }

    pub fn set_num_lights(&mut self,num:i32) {
        self.num_lights = num;
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut data = vec![];

        for l in self.lights.iter() {
            data.extend_from_slice(&l.get_data());
        }

        data.extend_from_slice(&crate::objects::matrix_helper::get_bytes_from_i32(self.num_lights));
        data.extend_from_slice(&crate::objects::matrix_helper::get_bytes_from_vec3(&vec3(0.0,0.0,0.0)));


        return data;
    }

    pub fn update(&mut self, engine:&GameEngine) {
        self.data = self.get_data();
        engine.queue.write_buffer(&engine.vertex_renderer.shader.light_buffer,0,&self.data);
    }
}

pub struct Light {
    pub pos:TVec3<f32>,
    pub color:Color
}

impl Light {

    pub fn empty() -> Self {
        Self { pos: vec3(0.0,0.0,0.0), color: Color::new(255,255,255) }
    }

    pub fn new(pos:TVec3<f32>,color:Color) -> Self {
        Self { pos, color }
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend(crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(self.pos.x,self.pos.y,self.pos.z,0.0)));
        data.extend(crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(self.color.r,self.color.g,self.color.b,0.0)));

        return data;
    }
}