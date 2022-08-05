use std::any::Any;
use wgpu::{CommandEncoder, RenderPass, TextureView};
use crate::animation::animator::Animator;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use crate::render::render_phase::RenderPhase;
use crate::ui::bounds::Bounds;
use crate::ui::ui_component::{UIComponent, UIComponentType};
use crate::ui::ui_constraint::ConstraintSettings;

pub struct UIRect {
    pub bounds: Bounds,
    pub settings: ConstraintSettings,
    pub color: Color,
    pub animator: Animator,
    pub remove: bool,
    pub hidden: bool
}

impl UIRect {
    pub fn new(x:f32,y:f32,width:f32,height:f32, color:Color) -> UIRect {
        let bounds = Bounds::new(x,y,width,height);
        let cs = ConstraintSettings::default();

        UIRect { bounds, settings: cs, color, animator: Animator::new(), remove: false, hidden: false }
    }
}

impl UIComponent for UIRect {

    fn get_type(&self) -> UIComponentType {
        UIComponentType::RECT
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

    fn update(&mut self, engine: &mut GameEngine, camera:&Camera) {
        if !self.hidden {
            engine.color_renderer.render_color_queue_matrix(&mut engine.offset_handler.camera_offset,&self.color,&self.bounds.mat4, camera,&engine.queue);
            self.animator.update(engine);
            self.animator.apply(&mut self.bounds.x,&mut self.bounds.y,&mut self.bounds.width,&mut self.bounds.height
                                ,&mut self.color.r,&mut self.color.g,&mut self.color.b,&mut self.color.a);
        }
    }

    fn render<'a>(&'a self,engine: &'a GameEngine, camera:&'a Camera, pass: &mut RenderPass<'a>, state: RenderPhase) {

    }

    fn should_be_removed(&self) -> bool {
        self.remove
    }

    fn remove(&self, engine: &mut GameEngine) {

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