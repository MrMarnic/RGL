use nalgebra_glm::{Mat4, mat4, TMat, TMat4, TVec3, vec3};
use crate::engine::game_engine::GameEngine;

pub struct Transform {
    pub matrix: Mat4,
    pub pos: TVec3<f32>,
    pub yaw : f32,
    pub pitch : f32,
    pub roll : f32,
    pub scale: TVec3<f32>,
    pub interpolation: bool,
    pub last_transform: Option<TransformData>,
    pub new_transform: Option<TransformData>,
}

impl Transform {

    pub fn new( x : f32, y : f32, z : f32, scale : TVec3<f32>) -> Transform {
        let pos = nalgebra_glm::vec3(x,y,z);
        let matrix = nalgebra_glm::translation(&vec3(x,y,z));
        let mut trans = Transform {
            matrix,
            pos,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            scale,
            interpolation: false,
            last_transform: None,
            new_transform: None
        };
        trans.matrix = trans.create_matrix();
        
        return trans;
    }

    pub fn new_interpolation( x : f32, y : f32, z : f32, scale : TVec3<f32>) -> Transform {
        let pos = nalgebra_glm::vec3(x,y,z);
        let matrix = nalgebra_glm::translation(&vec3(x,y,z));
        let mut trans = Transform {
            matrix,
            pos: pos.clone(),
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            scale,
            interpolation: true,
            last_transform: Some(TransformData::new(pos.clone())),
            new_transform: Some(TransformData::new(pos.clone()))
        };
        trans.matrix = trans.create_matrix();

        return trans;
    }

    pub fn activate_interpolation(&mut self) {
        self.last_transform = Some(TransformData::new(self.pos.clone()));
        self.new_transform = Some(TransformData::new(self.pos.clone()));
        self.interpolation = true;
    }

    pub fn interpolate(&mut self,engine:&GameEngine) {
        if self.interpolation {
            //self.pos = self.last_transform.as_ref().unwrap().pos.lerp(&self.new_transform.as_ref().unwrap().pos,engine.tick_alpha);
            self.matrix = self.create_matrix();
        }
    }

    pub fn handle_tick(&mut self) {
        if self.interpolation {
            self.last_transform.as_mut().unwrap().pos = self.new_transform.as_ref().unwrap().pos.clone();
            self.pos = self.new_transform.as_ref().unwrap().pos.clone();
            self.matrix = self.create_matrix();
        }
    }

    pub fn set_rotation(&mut self, yaw : f32, pitch : f32, roll: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.roll = roll;
        self.matrix = self.create_matrix();
    }

    pub fn rotate(&mut self, yaw : f32, pitch : f32, roll: f32) {
        self.yaw += yaw;
        self.pitch += pitch;
        self.roll += roll;
        self.matrix = self.create_matrix();
    }

    pub fn set_translation(&mut self, movement : TVec3<f32>) {
        if !self.interpolation {
            self.pos = movement;
            self.matrix = self.create_matrix();
        } else {
            self.new_transform.as_mut().unwrap().pos = movement;
        }
    }

    pub fn set_x(&mut self,x:f32) {
        if !self.interpolation {
            self.pos = vec3(x,self.pos.y,self.pos.z);
            self.matrix = self.create_matrix();
        } else {
            let p = self.new_transform.as_mut().unwrap().pos.clone();
            self.new_transform.as_mut().unwrap().pos = vec3(x,p.y,p.z);
        }
    }

    pub fn set_y(&mut self,y:f32) {
        if !self.interpolation {
            self.pos = vec3(self.pos.x,y,self.pos.z);
            self.matrix = self.create_matrix();
        } else {
            let p = self.new_transform.as_mut().unwrap().pos.clone();
            self.new_transform.as_mut().unwrap().pos = vec3(p.x,y,p.z);
        }
    }

    pub fn set_z(&mut self,z:f32) {
        if !self.interpolation {
            self.pos = vec3(self.pos.x,self.pos.y,z);
            self.matrix = self.create_matrix();
        } else {
            let p = self.new_transform.as_mut().unwrap().pos.clone();
            self.new_transform.as_mut().unwrap().pos = vec3(p.x,p.y,z);
        }
    }

    pub fn translate(&mut self, movement : TVec3<f32>) {
        if !self.interpolation {
            self.pos += movement;
            self.matrix = self.create_matrix();
        } else {
            self.new_transform.as_mut().unwrap().pos += movement;
        }
    }

    pub fn set_scale(&mut self, scale : TVec3<f32>) {
        self.scale = scale;
        self.matrix = self.create_matrix();
    }

    pub fn scale(&mut self, scale : TVec3<f32>) {
        self.scale = vec3(self.scale.x * scale.x,self.scale.y * scale.y,self.scale.z * scale.z);
        self.matrix = self.create_matrix();
    }

    fn create_matrix(&self) -> TMat4<f32> {
        let mut matrix : TMat4<f32> = nalgebra_glm::identity() as TMat4<f32>;
        matrix = nalgebra_glm::translate(&mut matrix,&vec3(self.pos.x,self.pos.y,self.pos.z));
        matrix = nalgebra_glm::rotate(&matrix, self.to_radians(self.pitch),&nalgebra_glm::vec3(1.0,0.0,0.0));
        matrix = nalgebra_glm::rotate(&matrix, self.to_radians(self.yaw),&nalgebra_glm::vec3(0.0,1.0,0.0));
        matrix = nalgebra_glm::rotate(&matrix, self.to_radians(self.roll),&nalgebra_glm::vec3(0.0,0.0,1.0));
        matrix = nalgebra_glm::scale(&matrix,&nalgebra_glm::vec3(self.scale.x,self.scale.y,self.scale.z));
        return matrix;
    }

    pub fn to_radians(&self, number : f32) -> f32 {
        return nalgebra_glm::radians(&nalgebra_glm::vec1(number)).x;
    }

    pub fn to_degrees(&self, number : f32) -> f32 {
        return nalgebra_glm::degrees(&nalgebra_glm::vec1(number)).x;
    }

    pub fn cos(&self,number: f32) -> f32 {
        return nalgebra_glm::cos(&nalgebra_glm::vec1(self.to_radians(number))).x;
    }

    pub fn sin(&self,number: f32) -> f32 {
        return nalgebra_glm::sin(&nalgebra_glm::vec1(self.to_radians(number))).x;
    }

    pub fn tan(&self,number: f32) -> f32 {
        return nalgebra_glm::tan(&nalgebra_glm::vec1(self.to_radians(number))).x;
    }
}

pub struct TransformData {
    pub pos: TVec3<f32>
}

impl TransformData {
    pub fn new(pos: TVec3<f32>) -> TransformData {
        return TransformData { pos }
    }
}