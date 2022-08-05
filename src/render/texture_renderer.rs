use crate::objects::vertex_buffer::VertexBuffer;
use wgpu::{Device, RenderPass, Queue, CommandEncoderDescriptor, PushConstantRange, BufferAddress, Surface, SurfaceConfiguration};
use crate::objects::texture_object::TextureObject;
use std::rc::Rc;
use nalgebra_glm::{TVec3, vec2};
use crate::objects::camera::Camera;
use crate::objects::tex_coord::TexCoord;
use std::ops::{Range, RangeBounds};
use crate::pipeline::texture_render_pipeline::TextureRenderPipelineGroup;

pub struct TextureRenderer {
    pub basic_2d_shader: TextureRenderPipelineGroup,
    pub mesh: VertexBuffer,
    pub to_render: Vec<(Rc<TextureObject>,u64)>
}

impl TextureRenderer {
    pub fn new(working_dir:String,device:&Device,config:&SurfaceConfiguration) -> TextureRenderer {
        unsafe {
            let shader = TextureRenderPipelineGroup::new(format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\2d\\vertex.shader"),format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\2d\\fragment.shader"),config,device);
            let mesh = VertexBuffer::default_vertex_buffer(device,true);

            return TextureRenderer { basic_2d_shader: shader, mesh, to_render: vec![] }
        }
    }

    pub fn finish<'a>(&'a self,render_pass:&mut RenderPass<'a>,camera:&'a Camera,range:Range<usize>) {
        render_pass.set_pipeline(&self.basic_2d_shader.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
        for (i,(tex,offset)) in self.to_render[range.clone()].iter().enumerate() {
            render_pass.set_bind_group(0,&tex.bind_group,&[]);
            render_pass.set_bind_group(2,&camera.transform_bind_group,&[*offset as u32]);
            render_pass.set_bind_group(3,&self.basic_2d_shader.group.bind_groups[0],&[((i as u32 + range.start as u32) * 256 as u32) as u32]);
            self.mesh.render(render_pass);
        }
    }

    pub fn render_texture_queue(&mut self, texture:Rc<TextureObject>, pos:&TVec3<f32>, scale:&TVec3<f32>, camera:&Camera, queue:&Queue, offset:&mut BufferAddress) {
        queue.write_buffer(&camera.buffers[2], *offset, &*crate::objects::matrix_helper::get_bytes(&nalgebra_glm::scale(&nalgebra_glm::translation(pos), scale)));
        queue.write_buffer(&self.basic_2d_shader.offset_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec_ref(&TexCoord::default().array));
        self.to_render.push((texture.clone(),*offset));
        *offset += 256;
    }

    pub fn render_texture_with_tex_coords_queue(&mut self, texture:Rc<TextureObject>, pos:&TVec3<f32>, scale:&TVec3<f32>, camera:&Camera, tex_coord:&TexCoord, queue:&Queue, offset:&mut BufferAddress) {
        queue.write_buffer(&camera.buffers[2],*offset,&*crate::objects::matrix_helper::get_bytes(&nalgebra_glm::scale(&nalgebra_glm::translation(pos),scale)));
        queue.write_buffer(&self.basic_2d_shader.offset_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec_ref(&tex_coord.array));
        self.to_render.push((texture.clone(),*offset));
        *offset += 256;
    }

    pub fn render_texture_with_tex_coords_instant<'a>(&'a self, texture:&'a Rc<TextureObject>, pos:&TVec3<f32>, scale:&TVec3<f32>, camera:&'a Camera, tex_coord:&'a TexCoord, queue:&Queue,render_pass:&mut RenderPass<'a>) {
        queue.write_buffer(&camera.buffers[2],0,&*crate::objects::matrix_helper::get_bytes(&nalgebra_glm::scale(&nalgebra_glm::translation(pos),scale)));
        queue.write_buffer(&self.basic_2d_shader.offset_buffer,0,&*crate::objects::matrix_helper::get_bytes_from_vec_ref(&tex_coord.array));
        render_pass.set_pipeline(&self.basic_2d_shader.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
        render_pass.set_bind_group(0,&texture.bind_group,&[]);
        render_pass.set_bind_group(2,&camera.transform_bind_group,&[0]);
        render_pass.set_bind_group(3,&self.basic_2d_shader.group.bind_groups[0],&[0]);
        self.mesh.render(render_pass);
    }

    /*
    pub fn render_sprite_queue(&mut self, sprite:&Sprite, camera:&Camera, queue:&Queue, offset:&mut BufferAddress) {
        queue.write_buffer(&camera.buffers[2],*offset,&*crate::objects::matrix_helper::get_bytes(&sprite.transform.matrix));
        if !sprite.tex_coord.default {
            queue.write_buffer(&self.basic_2d_shader.offset_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec_ref(&sprite.tex_coord.array));
        } else {
            queue.write_buffer(&self.basic_2d_shader.offset_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec_ref(&TexCoord::default().array));
        }
        self.to_render.push((sprite.texture.clone(),*offset));
        *offset += 256;
    }
     */
}