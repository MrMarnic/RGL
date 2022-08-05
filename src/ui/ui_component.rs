use std::any::Any;
use std::collections::HashMap;
use wgpu::{CommandEncoder, RenderPass, TextureView};
use crate::animation::animator::Animator;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::render::render_phase::RenderPhase;
use crate::ui::bounds::{Bounds};
use crate::ui::ui_button::UIButton;
use crate::ui::ui_constraint::{ConstraintSettings, UIConstraint};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum UIComponentType {
    PANE,
    COLOR_BUTTON,
    TEXT,
    IMAGE,
    RECT
}

pub trait UIComponent {
    fn get_type(&self) -> UIComponentType;
    fn get_constraint_settings(&self) -> &ConstraintSettings;
    fn get_bounds(&self) -> &Bounds;
    fn get_bounds_mut(&mut self) -> &mut Bounds;
    fn update(&mut self, engine:&mut GameEngine, camera:&Camera) {}
    fn process_input(&mut self, engine:&mut GameEngine, camera:&Camera) {}
    fn render<'a>(&'a self,engine: &'a GameEngine, camera:&'a Camera, pass: &mut RenderPass<'a>, state: RenderPhase);
    fn constrain(&mut self,parent:&Bounds) {
        self.default_constrain(parent);
    }
    fn should_be_removed(&self) -> bool;
    fn remove(&self, engine:&mut GameEngine);
    fn mark_for_remove(&mut self);
    fn hide(&mut self,value:bool);
    fn is_hidden(&self) -> bool;
    fn default_constrain(&mut self,parent:&Bounds) {
        let mut x = self.get_constraint_settings().x_constraint.calculate(self.get_bounds().x,parent.x);
        let mut y = self.get_constraint_settings().y_constraint.calculate(self.get_bounds().y,parent.y);
        let mut width = self.get_constraint_settings().width_constraint.calculate(self.get_bounds().width,parent.width);
        let mut height = self.get_constraint_settings().height_constraint.calculate(self.get_bounds().height,parent.height);

        let mut left = 0.0;
        if !self.get_constraint_settings().left_border.is_none() {
            left = self.get_constraint_settings().left_border.calculate(self.get_bounds().left(),parent.left());
        }
        if !self.get_constraint_settings().left_constraint.is_none() {
            left = self.get_constraint_settings().left_constraint.calculate(self.get_bounds().left(),parent.left());
        }
        let mut right = 0.0;
        if !self.get_constraint_settings().right_border.is_none() {
            right = self.get_constraint_settings().right_border.calculate(self.get_bounds().right(),parent.right());
        }
        if !self.get_constraint_settings().right_constraint.is_none() {
            right = self.get_constraint_settings().right_constraint.calculate(self.get_bounds().right(),parent.right());
        }
        let mut top = 0.0;
        if !self.get_constraint_settings().top_border.is_none() {
            top = self.get_constraint_settings().top_border.calculate(self.get_bounds().top(),parent.top());
        }
        if !self.get_constraint_settings().top_constraint.is_none() {
            top = self.get_constraint_settings().top_constraint.calculate(self.get_bounds().top(),parent.top());
        }
        let mut bottom = 0.0;
        if !self.get_constraint_settings().bottom_border.is_none() {
            bottom = self.get_constraint_settings().bottom_border.calculate(self.get_bounds().bottom(),parent.bottom());
        }
        if !self.get_constraint_settings().bottom_constraint.is_none() {
            bottom = self.get_constraint_settings().bottom_constraint.calculate(self.get_bounds().bottom(),parent.bottom());
        }

        if (!self.get_constraint_settings().left_border.is_none() && !self.get_constraint_settings().right_border.is_none()) | (!self.get_constraint_settings().left_border.is_none() && !self.get_constraint_settings().right_constraint.is_none()) | (!self.get_constraint_settings().left_constraint.is_none() && !self.get_constraint_settings().right_border.is_none()){
            let dis = right - left;
            let w = dis/2.0;

            x = left + w;
            width = w;
        } else {
            if !self.get_constraint_settings().left_border.is_none() {
                let dis = x - width - left;

                width += dis;
            }
            if !self.get_constraint_settings().right_border.is_none() {
                let dis = right - x - width;

                width += dis;
            }
            if !self.get_constraint_settings().left_constraint.is_none(){
                let dis = x - width - left;

                x -= dis;
            }
            if !self.get_constraint_settings().right_constraint.is_none(){
                let dis = right - x - width;
                x += dis;
            }
        }


        if (!self.get_constraint_settings().bottom_border.is_none() && !self.get_constraint_settings().top_border.is_none()) | (!self.get_constraint_settings().bottom_border.is_none() && !self.get_constraint_settings().top_constraint.is_none()) | (!self.get_constraint_settings().bottom_constraint.is_none() && !self.get_constraint_settings().top_border.is_none()){
            let dis = top - bottom;
            let w = dis/2.0;
            y = bottom + w;
            height = w;
        } else {
            if !self.get_constraint_settings().bottom_border.is_none() {
                let dis = y - height - bottom;

                height += dis;
            }
            if !self.get_constraint_settings().top_border.is_none() {
                let dis = top - y - height;

                height += dis;
            }
            if !self.get_constraint_settings().bottom_constraint.is_none(){
                let dis = y - height - bottom;

                y -= dis;
            }
            if !self.get_constraint_settings().top_constraint.is_none(){
                let dis = top - y - height;
                y += dis;
            }
        }


        let mut b = self.get_bounds_mut();

        b.x = x;
        b.y = y;
        b.width = width;
        b.height = height;

        b.update();
    }
    fn get_animator_mut(&mut self) -> &mut Animator;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait UIParent {
    fn get_components(&self) -> &Vec<Box<dyn UIComponent>>;
    fn get_components_mut(&mut self) -> &mut Vec<Box<dyn UIComponent>>;
    fn get_buttons(&self) -> &HashMap<u32,Box<dyn UIButton>>;
    fn get_buttons_mut(&mut self) -> &mut HashMap<u32,Box<dyn UIButton>>;
    fn get_button(&self,id:u32) -> &Box<dyn UIButton> {
        return &self.get_buttons()[&id];
    }
    fn get_button_mut(&mut self,id:u32) -> &mut Box<dyn UIButton> {
        return self.get_buttons_mut().get_mut(&id).unwrap();
    }
    fn add_button(&mut self,id:u32,button:Box<dyn UIButton>) {
        self.get_buttons_mut().insert(id,button);
    }
    fn update_children(&mut self, engine: &mut GameEngine, camera: &Camera, capture_input:&mut bool) {

        for c in self.get_components_mut().iter_mut() {

            if c.get_type() == UIComponentType::PANE {
                if !c.is_hidden() {
                    if engine.input_handler.mouse_aabb.collision_test_no_z(&c.get_bounds().aabb) {
                        *capture_input = true;
                    }
                }
            }

            c.update(engine,camera);
        }
        for (id,c) in self.get_buttons_mut().iter_mut() {
            c.update(engine,camera);
        }

        self.get_components_mut().retain(|c|{
            if c.should_be_removed() {
                c.remove(engine);
                return false
            }
            return true
        });
        self.get_buttons_mut().retain(|id,c|{
            if c.should_be_removed() {
                c.remove(engine);
                return false
            }
            return true
        });
    }
    fn process_input_children(&mut self, engine: &mut GameEngine, camera: &Camera) {
        for c in self.get_components_mut().iter_mut() {
            c.process_input(engine,camera);
        }
        for (id,c) in self.get_buttons_mut().iter_mut() {
            c.process_input(engine,camera);
        }
    }
    fn render_children<'a>(&'a self,engine: &'a GameEngine, camera:&'a Camera, pass: &mut RenderPass<'a>, state: RenderPhase) {
        for c in self.get_components().iter() {
            c.render(engine,camera,pass, state);
        }
        for (id,c) in self.get_buttons().iter() {
            c.render(engine,camera,pass, state);
        }
    }
    fn constrain_children(&mut self);
}