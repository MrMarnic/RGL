use wgpu::PresentMode;
use winit::dpi::{LogicalSize, PhysicalPosition, Position, Size};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::objects::color::Color;

pub struct GameWindow {
    pub title: String,
    pub window: Window,
    pub event_loop: Option<EventLoop<()>>,
    pub clear_color: Color,
    pub vsync: bool,
    pub should_close: bool
}

impl GameWindow {
    pub fn new(title: String, mut width: i32, mut height: i32,x:i32,y:i32,clear_color:Color,vsync:bool) -> GameWindow {

        let event_loop = EventLoop::new();

        let w = WindowBuilder::new().with_title(title.clone()).with_inner_size(Size::Logical(LogicalSize::new(width as f64,height as f64)))
            .build(&event_loop).unwrap();
        w.set_outer_position(Position::Physical(PhysicalPosition::new(x,y)));

        return GameWindow {
            title,
            window: w,
            event_loop: Some(event_loop),
            clear_color,
            vsync,
            should_close: false
        }
    }

    pub fn update(&mut self,control_flow:&mut ControlFlow) -> bool{
        if self.should_close {
            *control_flow = ControlFlow::Exit;
            return true;
        }
        return false;
    }

    pub fn get_present_mode(&self) -> PresentMode {
        return if self.vsync {
            PresentMode::Fifo
        } else {
            PresentMode::Immediate
        }
    }
}