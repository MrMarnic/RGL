use std::any::Any;
use std::rc::Rc;
use nalgebra_glm::{TVec3, vec3, vec4};
use wgpu::RenderPass;
use crate::animation::animator::Animator;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use crate::objects::transform::Transform;
use crate::objects::vertex_buffer::VertexBuffer;
use crate::render::render_phase::RenderPhase;
use crate::text::render_font::RenderFont;
use crate::ui::bounds::Bounds;
use crate::ui::ui_component::{UIComponent, UIComponentType};
use crate::ui::ui_constraint::ConstraintSettings;

pub struct UIText {
    pub lines:Vec<String>,
    pub bounds: Bounds,
    pub settings: ConstraintSettings,
    pub font: Rc<RenderFont>,
    pub color: Color,
    pub camera_offset: u32,
    pub color_offset: u32,
    pub mesh: VertexBuffer,
    pub transform: Transform,
    write_transform: bool,
    pub wrap_lines: bool,
    pub inside_button:bool,
    pub animator: Animator,
    pub animated: bool,
    pub remove: bool,
    pub hidden: bool
}

impl UIText {
    pub fn change_text(&mut self,lines:Vec<String>,engine:&GameEngine) {
        self.mesh = crate::ui::ui_text::create_buffer(&lines,engine,&self.font,1000.0);
        self.lines = lines;
    }

    pub fn write(&self,camera:&Camera,engine:&GameEngine) {
        engine.queue.write_buffer(&camera.buffers[2],self.camera_offset as u64,&*crate::objects::matrix_helper::get_bytes(&self.transform.matrix));
        self.write_color(engine);
    }

    pub fn write_color(&self,engine:&GameEngine) {
        engine.queue.write_buffer(&engine.text_renderer.shader_program.color_buffer,self.color_offset as u64,&*crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(self.color.r,self.color.g,self.color.b,self.color.a)));
    }

    pub fn free_up(&self,engine:&mut GameEngine) {
        engine.static_offset_handler.remove(self.camera_offset as u64);
        engine.text_renderer.static_offset_handler.remove(self.color_offset as u64);
    }

    pub fn new_new(mut text:String,font:Rc<RenderFont>,color:Color,engine:&mut GameEngine,x:f32,y:f32,width:f32, height:f32) -> UIText {
        let mut paragraphs : Vec<String> = text.split("\n\r").map(|s| s.to_string()).collect();

        let mut lines = vec![];

        for l in paragraphs {
            let mut ls : Vec<String> = l.split("\n").map(|s| s.to_string()).collect();
            lines.extend_from_slice(&ls);
            lines.push("".to_string());
        }

        let buffer = crate::ui::ui_text::create_buffer(&lines,engine,&font,width);

        let mut bounds = Bounds::new(x,y,width,height);

        return UIText {
            lines,
            bounds,
            settings: ConstraintSettings::default(),
            font,
            color,
            camera_offset: engine.static_offset_handler.get_offset() as u32,
            color_offset: engine.text_renderer.static_offset_handler.get_offset() as u32,
            mesh: buffer,
            transform: Transform::new(x,y,0.0,vec3(1.0,1.0,1.0)),
            write_transform: false,
            wrap_lines: false,
            inside_button: false,
            animator: Animator::new(),
            animated: false,
            remove: false,
            hidden: false
        };
    }

    fn fix_bounds_pos(&mut self) {
        let w = self.font.get_width(&self.lines[0]);
        let h = self.font.get_height(&self.lines[0]);

        self.transform.set_x(((self.bounds.x - w / 2.0) as i32) as f32);
        self.transform.set_y(((self.bounds.y - h / 2.0) as i32) as f32);

        self.write_transform = true;
    }
}

pub fn create_buffer(lines:&Vec<String>,engine:&GameEngine, font:&Rc<RenderFont>,width:f32) -> VertexBuffer{
    let mut vertecies = vec![];
    let mut indecies = vec![];
    let mut number = 0;

    let mut chars : Vec<Vec<char>> = vec![];

    for l in lines {
        chars.push(l.chars().collect::<Vec<char>>())
    }

    let mut unedited_chars = chars.clone();
    let mut remapped_lines : Vec<String> = vec![];

    while unedited_chars.len() > 0 {
        let mut cs = unedited_chars.remove(0);

        if let Some(index) = font.base.get_chars_test(width,&cs) {
            if index == cs.len() - 1 {
                remapped_lines.push(cs.iter().collect());
            } else {
                let a = cs.drain(0..index).collect::<String>();
                unedited_chars.insert(0,cs);
                remapped_lines.push(a);
            }
        }
    }

    for (line_number,text) in remapped_lines.iter().enumerate() {
        let mut last_pos = 0.0;

        for (id,c) in text.chars().enumerate() {
            if !c.is_whitespace() {
                if id > 0 {
                    let char = &font.characters[&c];
                    let mut vexs = vec![];
                    for v in &font.characters[&c].vertecies {
                        let mut vv = v.clone();
                        vv.x = vv.x + last_pos + char.x_offset as f32;
                        vv.y = vv.y - line_number as f32 * font.base.pixel_height as f32 + char.y_offset as f32;
                        vexs.push(vv);
                    }
                    vertecies.extend_from_slice(&vexs);

                    indecies.extend_from_slice(&vec![0 + 4 * number as u32,1 + 4 * number as u32,3 + 4 * number as u32,3 + 4 * number as u32,1 + 4 * number as u32,2 + 4 * number as u32]);
                    last_pos += char.x_advance as f32;
                } else {
                    let char = &font.characters[&c];
                    let mut vexs = vec![];
                    for v in &font.characters[&c].vertecies {
                        let mut vv = v.clone();
                        vv.x = vv.x + char.x_offset as f32;
                        vv.y = vv.y - line_number as f32 * font.base.pixel_height as f32 + char.y_offset as f32;
                        vexs.push(vv);
                    }
                    vertecies.extend_from_slice(&vexs);
                    indecies.extend_from_slice(&vec![0 + 4 * number as u32,1 + 4 * number as u32,3 + 4 * number as u32,3 + 4 * number as u32,1 + 4 * number as u32,2 + 4 * number as u32]);
                    last_pos += char.x_advance as f32;
                }
                number += 1;
            } else {
                last_pos += font.characters[&c].x_advance as f32;
            }
        }
    }

    let buffer = VertexBuffer::new(&engine.device,vertecies,indecies,false);

    return buffer
}

impl UIComponent for UIText {

    fn get_type(&self) -> UIComponentType {
        UIComponentType::TEXT
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
            if self.write_transform {
                self.write_transform = false;
                self.write(camera,engine);
                if self.wrap_lines {
                    self.mesh = create_buffer(&self.lines,engine,&self.font,self.bounds.width * 2.0);
                }
            }

            if self.animated {
                self.animator.update(engine);
                self.animator.apply(&mut self.bounds.x,&mut self.bounds.y,&mut self.bounds.width,&mut self.bounds.height
                                    ,&mut self.color.r,&mut self.color.g,&mut self.color.b,&mut self.color.a);
                self.write_color(engine);
                self.bounds.update();
                self.fix_bounds_pos();
            }
        }
    }

    fn render<'a>(&'a self, engine: &'a GameEngine, camera: &'a Camera, pass: &mut RenderPass<'a>, state: RenderPhase) {
        if !self.hidden {
            if state == RenderPhase::TEXT {
                engine.text_renderer.render_ui(self,pass,camera);
            }
        }
    }

    fn constrain(&mut self, parent: &Bounds) {
        if self.inside_button {
            self.bounds.set(parent.x,parent.y,parent.width,parent.height);
            self.bounds.update();

            let w = self.font.get_width(&self.lines[0]);
            let h = self.font.get_height(&self.lines[0]);

            self.transform.set_x(((self.bounds.x - w / 2.0) as i32) as f32);
            self.transform.set_y(((self.bounds.y - h / 2.0) as i32) as f32);

            self.write_transform = true;
        } else {
            self.default_constrain(parent);

            let w = self.font.get_width(&self.lines[0]);

            let dis = self.bounds.height;

            self.transform.set_x(((self.bounds.x - self.bounds.width) as i32) as f32);
            self.transform.set_y(((self.bounds.y + dis - self.font.base.pixel_height as f32) as i32 ) as f32);

            self.write_transform = true;
        }
    }

    fn should_be_removed(&self) -> bool {
        self.remove
    }

    fn remove(&self, engine: &mut GameEngine) {
        self.free_up(engine);
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