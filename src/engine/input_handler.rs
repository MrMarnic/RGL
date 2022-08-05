use nalgebra_glm::{normalize, TVec3, vec1, vec3, vec4};
use crate::objects::aabb::AABB;
use crate::engine::game_engine::GameEngine;
use winit::event::{KeyboardInput, ElementState, DeviceId, MouseButton, VirtualKeyCode};
use winit::dpi::{PhysicalSize, PhysicalPosition};

pub struct InputHandler {
    pressed_keys : [bool;348],
    clicked_key: i32,
    pub last_x : f32,
    pub last_y : f32,
    pub delta_x : f32,
    pub delta_y : f32,
    pub current_x: f32,
    pub current_y: f32,
    pub mouse_pos: TVec3<f32>,
    pub fixed_mouse_pos: TVec3<f32>,
    is_new : bool,
    clicked_button : [bool;7],
    pressed_button : [bool;7],
    pub scroll_x:f32,
    pub scroll_y:f32,
    pub mouse_aabb: AABB,
    pub current_key:Option<VirtualKeyCode>
}

impl InputHandler {

    pub fn new() -> InputHandler{
        let handler = InputHandler {
            pressed_keys: [false; 348],
            clicked_key: -1,
            last_x: 0.0,
            last_y: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            mouse_pos: vec3(0.0,0.0,0.0),
            fixed_mouse_pos: vec3(0.0,0.0,0.0),
            is_new: true,
            clicked_button: [false;7],
            pressed_button: [false;7],
            scroll_x:0.0,
            scroll_y:0.0,
            mouse_aabb: AABB::new(vec3(0.0,0.0,0.0),vec3(0.0,0.0,0.0)),
            current_key: None
        };

        return handler;
    }
    pub fn handle_key(&mut self, key:&KeyboardInput) {
        match key.state {
            ElementState::Pressed => {
                self.current_key = key.virtual_keycode;
                self.pressed_keys[key.virtual_keycode.unwrap() as usize] = true;
                self.clicked_key = key.virtual_keycode.unwrap() as i32;
            },
            ElementState::Released => {
                let code = key.virtual_keycode;
                if code.is_some() {
                    self.pressed_keys[code.unwrap() as usize] = false;
                }
            }
        }
    }

    pub fn handle_mouse(&mut self,device_id: &DeviceId, state: &ElementState, button: &MouseButton) {
        match button {
            MouseButton::Left => {
                if state == &ElementState::Pressed {
                    self.clicked_button[0] = true;
                    self.pressed_button[0] = true;
                }else {
                    self.pressed_button[0] = false;
                }
            },
            MouseButton::Right => {
                if state == &ElementState::Pressed {
                    self.clicked_button[1] = true;
                    self.pressed_button[1] = true;
                }else {
                    self.pressed_button[1] = false;
                }
            },
            MouseButton::Middle => {
                if state == &ElementState::Pressed {
                    self.clicked_button[2] = true;
                    self.pressed_button[2] = true;
                }else {
                    self.pressed_button[2] = false;
                }
            },
            _ => {}
        }
    }

    pub fn handle_cursor_move(&mut self,device_id:&DeviceId,position:&PhysicalPosition<f64>,size:&PhysicalSize<u32>) {
        let x = position.x as f32;
        let y = position.y as f32;

        self.mouse_pos = vec3(x,y,0.0);
        self.fixed_mouse_pos = vec3(x,size.height as f32 - y,0.0);

        self.mouse_aabb = AABB::new(vec3(x - 1.0,size.height as f32 - y - 1.0,0.0),vec3(x + 1.0,size.height as f32 - y + 1.0,0.0));
    }

    pub fn handle_cursor_move_delta(&mut self,delta:(f64,f64)) {
        self.delta_x = delta.0 as f32;
        self.delta_y = delta.1 as f32;
    }

    pub fn reset(&mut self) {
        for i in 0..self.clicked_button.len() {
            self.clicked_button[i] = false;
        }
        self.clicked_key = -1;
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
        self.current_key = None;
        self.delta_x = 0.0;
        self.delta_y = 0.0;
    }

    pub fn handle_scroll(&mut self,x:f32,y:f32) {
        self.scroll_x = x;
        self.scroll_y = y;
    }

    pub fn is_key_pressed(&self,key : VirtualKeyCode) -> bool{
        return self.pressed_keys[key as usize];
    }

    pub fn is_key_clicked(&self, key: VirtualKeyCode) -> bool {
        return self.clicked_key == key as i32;
    }

    pub fn is_mouse_pressed(&self,button: i32) -> bool{
        return self.pressed_button[button as usize];
    }

    pub fn is_mouse_clicked(&self,button: i32) -> bool{
        return self.clicked_button[button as usize];
    }

    pub fn is_scroll_y(&self) -> bool {
        return self.scroll_y != 0.0;
    }
}