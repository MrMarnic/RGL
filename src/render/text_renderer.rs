use nalgebra_glm::{TVec3, vec3, vec4};
use crate::pipeline::text_render_pipeline::TextRenderPipelineGroup;
use crate::objects::vertex_buffer::{OnlyCoordsVertexBuffer, VertexBuffer};
use wgpu::{Device, RenderPass, Queue, BufferAddress, Surface, SurfaceConfiguration};
use std::rc::Rc;
use crate::objects::vertex::Vertex;
use crate::objects::camera::Camera;
use crate::objects::color::Color;
use std::ops::Range;
use crate::objects::offset_handler::{OffsetHandler, StaticOffsetHandler};
use crate::objects::simple_text::SimpleText;
use crate::text::render_font::RenderFont;
use crate::ui::ui_text::UIText;

pub struct TextRenderer {
    pub shader_program: TextRenderPipelineGroup,
    pub char_mesh: VertexBuffer,
    pub color_offset: u64,
    pub static_offset_handler: StaticOffsetHandler,
    pub meshes_to_render: Vec<(u64,u64,VertexBuffer)>
}

impl TextRenderer {

    pub fn new(working_dir:String,device:&Device,config:&SurfaceConfiguration) -> TextRenderer {
        unsafe {
            let vertecies_char = vec![Vertex::new(-1.0,1.0,0.0,0.0,0.0),Vertex::new(-1.0,-1.0,0.0,0.0,1.0),Vertex::new(1.0,-1.0,0.0,1.0,1.0),Vertex::new(1.0,1.0,0.0,1.0,0.0)];
            let indecies_char = vec![0,1,3,3,1,2];

            let shader = TextRenderPipelineGroup::new(format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\text\\vertex.shader"),format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\text\\fragment.shader"),config,device,false);

            return TextRenderer {
                shader_program: shader,
                char_mesh: VertexBuffer::new(device, vertecies_char, indecies_char, false),
                color_offset: 0,
                static_offset_handler: StaticOffsetHandler::new(200, 20),
                meshes_to_render: vec![]
            }
        }
    }

    pub fn render_text(&mut self,lines:&Vec<String>,pos:&TVec3<f32>, color:Color, queue:&Queue, camera_offset:&mut BufferAddress, camera:&Camera, device:&Device, font:&Rc<RenderFont>, width:f32) {
        let offset = self.color_offset;
        queue.write_buffer(&self.shader_program.color_buffer,offset,&crate::objects::matrix_helper::get_bytes_from_vec4(&vec4(color.r,color.g,color.b,color.a)));
        queue.write_buffer(&camera.buffers[2],*camera_offset,&*crate::objects::matrix_helper::get_bytes(&nalgebra_glm::scale(&nalgebra_glm::translation(pos),&vec3(1.0,1.0,0.0))));
        let mesh = crate::objects::simple_text::create_buffer(lines,device,font,width);

        self.meshes_to_render.push((offset,*camera_offset,mesh));

        *camera_offset += 256;
        self.color_offset += 256;
    }

    pub fn finish<'a>(&'a self,render_pass:&mut RenderPass<'a>,camera:&'a Camera,range:Range<usize>, font:&'a Rc<RenderFont>){
        render_pass.set_bind_group(0,&font.texture.bind_group,&[]);
        for (c,cam,mesh) in self.meshes_to_render[range.clone()].iter() {
            render_pass.set_bind_group(2,&camera.transform_bind_group,&[*cam as u32]);
            render_pass.set_bind_group(3,&self.shader_program.color_group,&[*c as u32]);
            mesh.render(render_pass);
        }
    }

    pub fn begin<'a>(&'a self, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        render_pass.set_pipeline(&self.shader_program.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
    }

    pub fn render_ui<'a>(&'a self, gui_text:&'a UIText, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        render_pass.set_bind_group(0,&gui_text.font.texture.bind_group,&[]);
        render_pass.set_bind_group(2,&camera.transform_bind_group,&[gui_text.camera_offset as u32]);
        render_pass.set_bind_group(3,&self.shader_program.color_group,&[gui_text.color_offset as u32]);
        gui_text.mesh.render(render_pass);
    }

    pub fn render_ui_simple<'a>(&'a self, gui_text:&'a SimpleText, render_pass:&mut RenderPass<'a>, camera:&'a Camera) {
        render_pass.set_bind_group(0,&gui_text.font.texture.bind_group,&[]);
        render_pass.set_bind_group(2,&camera.transform_bind_group,&[gui_text.camera_offset as u32]);
        render_pass.set_bind_group(3,&self.shader_program.color_group,&[gui_text.color_offset as u32]);
        gui_text.mesh.render(render_pass);
    }
}