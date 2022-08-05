use crate::pipeline::line_render_pipeline::LineRenderPipelineGroup;
use crate::objects::vertex_buffer::OnlyCoordsVertexBuffer;
use wgpu::{Device, Queue, RenderPass, CommandEncoderDescriptor, BufferAddress, Surface, SurfaceConfiguration};
use crate::objects::vertex::OnlyCoordsVertex;
use crate::objects::color::Color;
use crate::objects::camera::Camera;
use nalgebra_glm::{TVec3, vec3,vec4, TMat4};
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use std::convert::TryInto;
use std::ops::Range;

pub struct LineRenderer {
    pub line_shader: LineRenderPipelineGroup,
    pub line_mesh: OnlyCoordsVertexBuffer,
    pub to_render: Vec<u64>
}

impl LineRenderer {
    pub fn new(working_dir:String,device:&Device,config:&SurfaceConfiguration) -> LineRenderer {
        let shader = LineRenderPipelineGroup::new(format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\line\\vertex.shader"),format!("{}\\{}",&working_dir.to_string(),"assets\\shader\\line\\fragment.shader"),config,device);
        let mesh = OnlyCoordsVertexBuffer::new(device,vec![OnlyCoordsVertex::new(0.0,0.0,0.0),OnlyCoordsVertex::new(0.0,0.0,0.0)],vec![0,1],true);

        return LineRenderer { line_shader: shader, line_mesh: mesh, to_render: vec![] }
    }

    pub fn render_line_queue(&mut self, offset:&mut BufferAddress, start:TVec3<f32>, end:TVec3<f32>, color:Color, queue:&Queue) {

        let mut bytes = crate::objects::matrix_helper::get_bytes(&nalgebra_glm::translation(&start));
        bytes.extend(crate::objects::matrix_helper::get_bytes(&nalgebra_glm::translation(&end)));
        queue.write_buffer(&self.line_shader.transform_buffer,*offset,&*bytes);
        queue.write_buffer(&self.line_shader.color_buffer,self.to_render.len() as u64 * 256,&*crate::objects::matrix_helper::get_bytes_from_vec(vec![color.r,color.g,color.b,color.a]));
        self.to_render.push(*offset);
        *offset += 256;
    }

    pub fn finish<'a>(&'a self,render_pass:&mut RenderPass<'a>,camera:&'a Camera,range:Range<usize>) {
        render_pass.set_pipeline(&self.line_shader.group.pipeline);
        render_pass.set_bind_group(1,&camera.bind_group,&[]);
        for (i,off) in self.to_render[range.clone()].iter().enumerate() {
            render_pass.set_bind_group(0,&self.line_shader.group.bind_groups[0],&[((i as u32 + range.start as u32) * 256 as u32) as u32]);
            render_pass.set_bind_group(2,&self.line_shader.group.bind_groups[1],&[*off as u32]);
            self.line_mesh.render(render_pass);
        }
    }
}