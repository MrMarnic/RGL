use std::any::Any;
use std::collections::HashMap;
use wgpu::{CommandEncoder, RenderPass, TextureView};
use crate::animation::animator::Animator;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use crate::render::render_phase::RenderPhase;
use crate::ui::bounds::Bounds;
use crate::ui::ui_button::UIButton;
use crate::ui::ui_component::{UIComponent, UIComponentType, UIParent};
use crate::ui::ui_constraint::{ConstraintSettings, UIConstraint};

pub struct UIPane {
    pub components: Vec<Box<dyn UIComponent>>,
    pub buttons: HashMap<u32,Box<dyn UIButton>>,
    pub bounds: Bounds,
    pub settings: ConstraintSettings,
    pub color: Color,
    pub render: bool,
    pub animator: Animator,
    pub remove: bool,
    pub delay:f32,
    pub action:Option<Box<dyn FnMut(&mut UIPane)>>,
    pub hidden: bool,
    pub capture_input: bool
}

impl UIPane {
    pub fn new(x:f32,y:f32,width:f32,height:f32) -> UIPane {
        let bounds = Bounds::new(x,y,width,height);
        let cs = ConstraintSettings::default();

        UIPane { components: vec![], buttons: HashMap::new(), bounds, settings: cs, color: Color::new(0, 0, 0), render: false, animator: Animator::new(), remove: false, delay: 0.0, action: None, hidden: false, capture_input: false }
    }

    pub fn activate_render(&mut self,color:Color) {
        self.render = true;
        self.color = color;
    }

    fn load(&mut self,engine:&mut GameEngine) {

    }

    pub fn constrain_pane(&mut self) {
        self.constrain_children();
    }
}

impl UIComponent for UIPane {
    fn get_type(&self) -> UIComponentType {
        UIComponentType::PANE
    }

    fn get_constraint_settings(&self) -> &ConstraintSettings {
        &self.settings
    }

    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn get_bounds_mut(&mut self) -> &mut Bounds {
        &mut self.bounds
    }

    fn update(&mut self, engine: &mut GameEngine, camera: &Camera) {
        if !self.hidden {
            if self.render {
                engine.color_renderer.render_color_queue_matrix(&mut engine.offset_handler.camera_offset,&self.color,&self.bounds.mat4,camera,&engine.queue);
            }
            let mut capture_input = false;
            self.update_children(engine,camera,&mut capture_input);
            self.capture_input = capture_input;

            if self.render && engine.input_handler.mouse_aabb.collision_test_no_z(&self.bounds.aabb) {
                self.capture_input = true;
            }

            if self.action.is_some() {
                self.delay -= engine.delta_time;

                if self.delay <= 0.0 {
                    (self.action.take().unwrap())(self);
                }
            }
        }
    }

    fn constrain(&mut self, parent: &Bounds) {
        self.default_constrain(parent);
        self.constrain_pane();
    }

    fn process_input(&mut self, engine: &mut GameEngine, camera: &Camera) {
        if !self.hidden {
            self.process_input_children(engine,camera);
        }
    }
    
    fn render<'a>(&'a self,engine: &'a GameEngine, camera:&'a Camera, pass: &mut RenderPass<'a>, state: RenderPhase) {
        if !self.hidden {
            self.render_children(engine,camera,pass,state);
        }
    }

    fn should_be_removed(&self) -> bool {
        self.remove
    }

    fn remove(&self, engine: &mut GameEngine) {
        for x in 0..self.components.len() {
            self.components[x].remove(engine);
        }
        for (id,c) in self.buttons.iter() {
            c.remove(engine);
        }
    }

    fn mark_for_remove(&mut self) {
        self.remove = true;
    }

    fn hide(&mut self, value: bool) {
        self.hidden = value;
    }

    fn is_hidden(&self) -> bool {
        self.hidden
    }

    fn get_animator_mut(&mut self) -> &mut Animator {
        &mut self.animator
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl UIParent for UIPane {
    fn get_components(&self) -> &Vec<Box<dyn UIComponent>> {
        &self.components
    }

    fn get_components_mut(&mut self) -> &mut Vec<Box<dyn UIComponent>> {
        &mut self.components
    }

    fn get_buttons(&self) -> &HashMap<u32, Box<dyn UIButton>> {
        &self.buttons
    }

    fn get_buttons_mut(&mut self) -> &mut HashMap<u32, Box<dyn UIButton>> {
        &mut self.buttons
    }

    fn constrain_children(&mut self) {
        for x in 0..self.components.len() {
            self.components[x].constrain(&self.bounds);
        }
        for (id,c) in self.buttons.iter_mut() {
            c.constrain(&self.bounds);
        }
    }
}