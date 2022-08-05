use std::any::Any;
use std::rc::Rc;
use nalgebra_glm::vec3;
use wgpu::{CommandEncoder, RenderPass, TextureView};
use crate::animation::animator::Animator;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use crate::objects::texture_object::TextureObject;
use crate::objects::transform::Transform;
use crate::objects::vertex_buffer_builder::VertexBufferBuilder;
use crate::objects::vertex_buffer_obj::VertexBufferObject;
use crate::render::render_phase::RenderPhase;
use crate::ui::bounds::Bounds;
use crate::ui::ui_component::{UIComponent, UIComponentType};
use crate::ui::ui_constraint::ConstraintSettings;

pub struct UIImage {
    pub bounds: Bounds,
    pub settings: ConstraintSettings,
    pub animator: Animator,
    pub remove: bool,
    pub hidden: bool,
    pub texture: Rc<TextureObject>,
    pub vbo: VertexBufferObject,
    pub write_transform: bool
}

impl UIImage {
    pub fn new(x:f32,y:f32,width:f32,height:f32, texture:Rc<TextureObject>, engine:&mut GameEngine) -> UIImage {
        let bounds = Bounds::new(x,y,width,height);
        let cs = ConstraintSettings::default();
        let mesh = VertexBufferBuilder::new().square(0.0,0.0,1.0).build(&engine.device);
        let vbo = VertexBufferObject::new(mesh,Transform::new(x,y,0.0,vec3(width,height,0.0)),engine);

        UIImage { bounds, settings: cs, animator: Animator::new(), remove: false, hidden: false, texture, vbo, write_transform: true }
    }
}

impl UIComponent for UIImage {
    fn get_type(&self) -> UIComponentType {
        UIComponentType::IMAGE
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
            if self.write_transform {
                self.vbo.write(engine,camera);
                self.write_transform = false;
            }
        }
    }

    fn render<'a>(&'a self,engine: &'a GameEngine, camera:&'a Camera, pass: &mut RenderPass<'a>, state: RenderPhase) {
        if state == RenderPhase::VERTEX {
            if! self.hidden {
                engine.custom_vertex_renderer.render(pass,camera,&self.texture,self.vbo.offset,&self.vbo.mesh);
            }
        }
    }

    fn constrain(&mut self, parent: &Bounds) {
        self.default_constrain(parent);
        self.vbo.transform.set_x(self.bounds.x);
        self.vbo.transform.set_y(self.bounds.y);
        self.vbo.transform.set_scale(vec3(self.bounds.width,self.bounds.height,0.0));
        self.write_transform = true;
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