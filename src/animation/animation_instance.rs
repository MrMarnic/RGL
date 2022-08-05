use std::rc::Rc;
use nalgebra_glm::{TVec3, vec3};
use crate::animation::animation::Animation;
use crate::animation::key_frame::KeyFrame;
use crate::engine::game_engine::GameEngine;
use crate::objects::transform::{Transform, TransformData};

pub struct AnimationInstance {
    pub animation:Rc<Animation>,
    pub time: f32,
    pub pos:TVec3<f32>,
    pub scale:TVec3<f32>,
    pub rotation:TVec3<f32>,
    pub current_key_frame: usize,
    pub finished: bool,
    pub pos_per_sec:TVec3<f32>,
    pub scale_per_sec:TVec3<f32>,
    pub rotation_per_sec:TVec3<f32>,
    pub activated: bool,
    pub changed_rotation:TVec3<f32>,
    pub changed_pos:TVec3<f32>,
    pub changed_scale:TVec3<f32>
}

impl AnimationInstance {

    pub fn new(animation:Rc<Animation>, transform:&Transform) -> Self {
        let mut a = AnimationInstance {
            animation: animation.clone(),
            time: 0.0,
            pos: transform.pos,
            scale: vec3(1.0,1.0,1.0),
            rotation: vec3(transform.yaw,transform.roll,transform.pitch),
            current_key_frame: 0,
            finished: false,
            pos_per_sec: vec3(0.0,0.0,0.0),
            scale_per_sec: vec3(0.0,0.0,0.0),
            rotation_per_sec: vec3(0.0,0.0,0.0),
            activated: false,
            changed_rotation: vec3(0.0,0.0,0.0),
            changed_pos: vec3(0.0,0.0,0.0),
            changed_scale: vec3(0.0,0.0,0.0)

        };

        a.init_key_frame(animation.key_frames[0].clone());

        return a;
    }

    pub fn update(&mut self, engine:&GameEngine) {
        if self.activated && !self.finished{
            let frame = &self.animation.key_frames[self.current_key_frame];
            let d = engine.delta_time;
            self.time += engine.delta_time;

            if self.time < frame.time {
                self.rotation += vec3(d * self.rotation_per_sec.x,d * self.rotation_per_sec.y,d * self.rotation_per_sec.z);
                self.changed_rotation = vec3(d * self.rotation_per_sec.x,d * self.rotation_per_sec.y,d * self.rotation_per_sec.z);

                self.pos += vec3(d * self.pos_per_sec.x,d * self.pos_per_sec.y,d * self.pos_per_sec.z);
                self.changed_pos = vec3(d * self.pos_per_sec.x,d * self.pos_per_sec.y,d * self.pos_per_sec.z);

                self.scale += vec3(d * self.scale_per_sec.x,d * self.scale_per_sec.y,d * self.scale_per_sec.z);
                self.changed_scale = vec3(d * self.scale_per_sec.x,d * self.scale_per_sec.y,d * self.scale_per_sec.z);
            } else {
                self.changed_rotation = vec3(self.rotation.x - frame.rotation.x,self.rotation.y - frame.rotation.y,self.rotation.z - frame.rotation.z);
                self.rotation = vec3(frame.rotation.x,frame.rotation.y,frame.rotation.z);

                self.changed_pos = vec3(self.pos.x - frame.pos.x,self.pos.y - frame.pos.y,self.pos.z - frame.pos.z);
                self.pos = vec3(frame.pos.x,frame.pos.y,frame.pos.z);

                self.changed_scale = vec3(self.scale.x - frame.scale.x,self.scale.y - frame.scale.y,self.scale.z - frame.scale.z);
                self.scale = vec3(frame.scale.x,frame.scale.y,frame.scale.z);

                if let Some(next) = self.animation.get_next_key_frame(self.current_key_frame) {
                    self.init_key_frame(next.clone());
                    self.current_key_frame += 1;
                } else {
                    self.finished = true;
                    self.activated = false;
                    self.changed_rotation = vec3(0.0,0.0,0.0);
                    self.changed_pos = vec3(0.0,0.0,0.0);
                    self.changed_scale = vec3(0.0,0.0,0.0);
                }
            }
        }
    }

    pub fn update_self(&mut self) {
        self.init_key_frame(self.animation.key_frames[self.current_key_frame].clone());
    }

    pub fn init_key_frame(&mut self, frame:KeyFrame) {
        self.time = 0.0;
        self.rotation_per_sec = vec3(frame.rotation.x/(frame.time - self.time),frame.rotation.y/(frame.time - self.time),frame.rotation.z/(frame.time - self.time));
        self.scale_per_sec = vec3(frame.scale.x/(frame.time - self.time),frame.scale.y/(frame.time - self.time),frame.scale.z/(frame.time - self.time));
        self.pos_per_sec = vec3(frame.pos.x/(frame.time - self.time),frame.pos.y/(frame.time - self.time),frame.pos.z/(frame.time - self.time));
        self.changed_rotation = vec3(0.0,0.0,0.0);
        self.changed_pos = vec3(0.0,0.0,0.0);
        self.changed_scale = vec3(0.0,0.0,0.0);
    }

    fn check(&mut self, engine:&GameEngine, key_frame:&KeyFrame) {

    }
}