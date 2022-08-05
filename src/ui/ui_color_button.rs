use std::any::Any;
use std::rc::Rc;
use nalgebra_glm::vec3;
use wgpu::{CommandEncoder, RenderPass, TextureView};
use crate::animation::animator::{AnimationAttribute, Animator};
use crate::engine::game_engine::GameEngine;
use crate::objects::aabb::AABB;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use crate::render::render_phase::RenderPhase;
use crate::text::render_font::RenderFont;
use crate::ui::bounds::Bounds;
use crate::ui::ui_button::{ButtonState, UIButton};
use crate::ui::ui_color_button::ButtonState::NORMAL;
use crate::ui::ui_component::{UIComponent, UIComponentType};
use crate::ui::ui_constraint::ConstraintSettings;
use crate::ui::ui_text::UIText;

pub struct UIColorButton {
    pub bounds: Bounds,
    pub settings: ConstraintSettings,
    pub text: UIText,
    pub button_settings: ButtonSettings,
    pub aabb: AABB,
    pub state: ButtonState,
    write_transform: bool,
    pub animator: Animator,
    pub color:Color,
    pub remove: bool,
    pub hidden: bool
}

impl UIColorButton {
    pub fn new(x:f32,y:f32,width:f32,height:f32, color:Color, text_str: String, font:Rc<RenderFont>, engine:&mut GameEngine) -> UIColorButton {
        let bounds = Bounds::new(x,y,width,height);
        let cs = ConstraintSettings::default();
        let w = font.get_width(&text_str);
        let h = font.get_height(&text_str);
        let mut text = UIText::new_new(text_str,font,color,engine,x - w / 2.0,y-h/2.0,width,height);
        text.inside_button = true;
        let b_settings = ButtonSettings { normal: Color::new(15,140,0), hover: Color::new(19,194,0), click: Color::new(26,255,0) };

        let color = b_settings.normal.clone();

        UIColorButton { bounds, settings: cs, text, button_settings: b_settings, aabb: AABB::new(vec3(x-width, y-height, 0.0),
                                                                                                 vec3(x+width, y+height, 0.0)),
            state: ButtonState::NORMAL,
            write_transform: false,
            animator: Animator::new(),
            color,
            remove: false,
            hidden: false
        }
    }

    pub fn update_color(&mut self) {
        self.color = self.button_settings.normal.clone();
    }

    pub fn create_aabb(&mut self) {
        self.aabb = AABB::new(vec3(self.bounds.x-self.bounds.width, self.bounds.y-self.bounds.height, 0.0),
                              vec3(self.bounds.x+self.bounds.width, self.bounds.y+self.bounds.height, 0.0));
    }
}

impl UIComponent for UIColorButton {
    fn get_type(&self) -> UIComponentType {
        UIComponentType::COLOR_BUTTON
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
            match self.state {
                ButtonState::HOVER => {
                    engine.color_renderer.render_color_queue_matrix(&mut engine.offset_handler.camera_offset,&self.color,&self.bounds.mat4,&camera,&engine.queue);
                },
                ButtonState::CLICK | ButtonState::PRESSED => {
                    engine.color_renderer.render_color_queue_matrix(&mut engine.offset_handler.camera_offset,&self.color,&self.bounds.mat4,&camera,&engine.queue);
                },
                ButtonState::NORMAL => {
                    engine.color_renderer.render_color_queue_matrix(&mut engine.offset_handler.camera_offset,&self.color,&self.bounds.mat4,&camera,&engine.queue);
                }
            }

            if self.write_transform {
                self.write_transform = false;
                self.text.write(camera,engine);
            }

            self.text.update(engine,camera);

            self.animator.update(engine);
            self.animator.apply(&mut self.bounds.x,&mut self.bounds.y,&mut self.bounds.width,&mut self.bounds.height
                                ,&mut self.color.r,&mut self.color.g,&mut self.color.b,&mut self.color.a);
            self.text.constrain(&self.bounds);
            self.bounds.update();
        }
    }

    fn process_input(&mut self, engine: &mut GameEngine, camera: &Camera) {
        if !self.hidden {
            if self.aabb.collision_test_no_z(&engine.input_handler.mouse_aabb) {
                if engine.input_handler.is_mouse_pressed(0) {
                    if self.state != ButtonState::CLICK && self.state != ButtonState::PRESSED{
                        self.state = ButtonState::CLICK;
                        if !self.animator.is_changing(AnimationAttribute::COLOR) {
                            self.color.copy(&self.button_settings.click);
                        }
                    } else {
                        self.state = ButtonState::PRESSED;
                        if !self.animator.is_changing(AnimationAttribute::COLOR) {
                            self.color.copy(&self.button_settings.click);
                        }
                    }
                } else {
                    self.state = ButtonState::HOVER;
                    if !self.animator.is_changing(AnimationAttribute::COLOR) {
                        self.color.copy(&self.button_settings.hover);
                    }
                }
            } else {
                if self.state != ButtonState::NORMAL {
                    self.state = ButtonState::NORMAL;
                    if !self.animator.is_changing(AnimationAttribute::COLOR) {
                        self.color.copy(&self.button_settings.normal);
                    }
                }
            }
        }
    }

    fn render<'a>(&'a self,engine: &'a GameEngine, camera:&'a Camera, pass: &mut RenderPass<'a>, state: RenderPhase) {
        if !self.hidden {
            if state == RenderPhase::TEXT {
                engine.text_renderer.render_ui(&self.text,pass,camera);
            }
        }
    }

    fn constrain(&mut self, parent: &Bounds) {
        self.default_constrain(parent);
        self.text.constrain(&self.bounds);
        self.create_aabb();
    }

    fn should_be_removed(&self) -> bool {
        self.remove
    }

    fn remove(&self, engine: &mut GameEngine) {
        self.text.remove(engine);
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

impl UIButton for UIColorButton{
    fn get_state(&self) -> ButtonState {
        self.state
    }

    fn get_text(&self) -> &UIText {
        &self.text
    }

    fn get_text_mut(&mut self) -> &mut UIText {
        &mut self.text
    }
}

pub struct ButtonSettings {
    pub normal: Color,
    pub hover: Color,
    pub click: Color
}