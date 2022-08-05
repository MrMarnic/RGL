use std::any::Any;
use std::rc::Rc;
use nalgebra_glm::{TVec3, vec3, vec4};
use wgpu::{Device, RenderPass};
use crate::animation::animator::Animator;
use crate::engine::game_engine::GameEngine;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use crate::objects::transform::Transform;
use crate::objects::vertex_buffer::VertexBuffer;
use crate::render::render_phase::RenderPhase;
use crate::text::render_font::RenderFont;
use crate::ui::bounds::Bounds;
use crate::ui::ui_component::UIComponent;
use crate::ui::ui_constraint::ConstraintSettings;

pub struct SimpleText {
    pub lines:Vec<String>,
    pub font: Rc<RenderFont>,
    pub color: Color,
    pub camera_offset: u32,
    pub color_offset: u32,
    pub mesh: VertexBuffer,
    pub transform: Transform,
    write_transform: bool,
    pub wrap_lines: bool,
    pub remove: bool,
    pub hidden: bool
}

impl SimpleText {
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

    pub fn new(mut text:String,font:Rc<RenderFont>,color:Color,engine:&mut GameEngine,x:f32,y:f32,width:f32, height:f32) -> SimpleText {
        let mut paragraphs : Vec<String> = text.split("\n\r").map(|s| s.to_string()).collect();

        let mut lines = vec![];

        for l in paragraphs {
            let mut ls : Vec<String> = l.split("\n").map(|s| s.to_string()).collect();
            lines.extend_from_slice(&ls);
            lines.push("".to_string());
        }

        let buffer = crate::ui::ui_text::create_buffer(&lines,engine,&font,width);

        return SimpleText {
            lines,
            font,
            color,
            camera_offset: engine.static_offset_handler.get_offset() as u32,
            color_offset: engine.text_renderer.static_offset_handler.get_offset() as u32,
            mesh: buffer,
            transform: Transform::new(x,y,0.0,vec3(1.0,1.0,1.0)),
            write_transform: false,
            wrap_lines: false,
            remove: false,
            hidden: false
        };
    }

    fn update(&mut self, engine: &mut GameEngine, camera: &Camera) {
        if !self.hidden {
            if self.write_transform {
                self.write_transform = false;
                self.write(camera,engine);
                if self.wrap_lines {
                    self.mesh = create_buffer(&self.lines,&engine.device,&self.font,1000.0);
                }
            }
        }
    }

    fn render<'a>(&'a self, engine: &'a GameEngine, camera: &'a Camera, pass: &mut RenderPass<'a>) {
        if !self.hidden {
            engine.text_renderer.render_ui_simple(self,pass,camera);
        }
    }
}

pub fn create_buffer(lines:&Vec<String>,device:&Device, font:&Rc<RenderFont>,width:f32) -> VertexBuffer{
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

    let buffer = VertexBuffer::new(device,vertecies,indecies,false);

    return buffer
}