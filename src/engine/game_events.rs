use std::time::Instant;
use env_logger::Builder;
use log::LevelFilter;
use wgpu::{Backends, TextureViewDescriptor};
use winit::event::{DeviceEvent, Event, MouseScrollDelta, WindowEvent};
use winit::event_loop::ControlFlow;
use crate::engine::game_engine::GameEngine;
use crate::engine::game_window::GameWindow;
use crate::scene::scene_handler::SceneHandler;
use chrono::Local;
use std::io::Write;


pub struct GameEvents {

}

impl GameEvents {

    pub fn new() -> GameEvents {
        return GameEvents{};
    }

    pub async fn run(&self, mut scene_handler:SceneHandler, mut game_window:GameWindow, backend:Backends) {

        if cfg!(debug_assertions) {
            Builder::new()
                .format(|buf, record| {
                    writeln!(buf,
                             "[{}] [{}] - {}",
                             Local::now().format("%Y-%m-%dT%H:%M:%S"),
                             record.level(),
                             record.args()
                    )
                })
                .filter(None, LevelFilter::Warn)
                .filter(None, LevelFilter::Info)
                .init();
        }

        //env_logger::init();

        let event_loop = game_window.event_loop.take().unwrap();

        let mut delta_time_calc = DeltaTimeCalc::new();
        //let mut tick_handler = TickHandler::new((1000.0 / ticks_per_second as f64) / 1000.0,ticks_per_second);
        let mut fps_calc = FpsCalc::new();

        let mut engine = GameEngine::new(game_window,backend).await;
        //let mut gui_handler = GuiHandler::new();

        scene_handler.opened_scene.loaded(&mut engine);

        engine.audio_handler.start();

        event_loop.run(move |event, _, control_flow|{
            match event {

                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == engine.game_window.window.id() => match event {
                    WindowEvent::CloseRequested => {
                        engine.audio_handler.destroy();
                        //scene_handler.opened_scene.destroy();
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        engine.input_handler.handle_key(input);
                    },WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        engine.input_handler.handle_mouse(device_id,state,button);
                    },WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => {
                        engine.input_handler.handle_cursor_move(device_id,position,&engine.size);
                    }
                    ,WindowEvent::Resized(physical_size) => {
                        if physical_size.width * physical_size.height > 0 {
                            engine.size = physical_size.clone();
                            engine.config.width = physical_size.width;
                            engine.config.height = physical_size.height;
                            engine.surface.configure(&engine.device,&engine.config);
                            scene_handler.opened_scene.window_resized(&mut engine,physical_size);
                        }
                    }
                    _ => {}
                },Event::DeviceEvent {
                    device_id, event
                } => match event {
                    DeviceEvent::MouseMotion {
                        delta
                    } => {
                        engine.input_handler.handle_cursor_move_delta(delta);
                    },
                    DeviceEvent::MouseWheel {
                        delta
                    } => {
                        match delta {
                            MouseScrollDelta::PixelDelta(pos) => {
                                engine.input_handler.handle_scroll(pos.x as f32,pos.y as f32);
                            },
                            MouseScrollDelta::LineDelta(x,y) => {
                                engine.input_handler.handle_scroll(x,y);
                            }
                        }
                    },
                    _ => {}
                }
                ,Event::RedrawRequested(_) => {
                    engine.static_offset_handler.update();
                    delta_time_calc.update(&mut engine);
                    engine.delta_time = delta_time_calc.delta;

                    if engine.game_window.update(control_flow) {
                        engine.audio_handler.destroy();
                        //scene_handler.opened_scene.destroy();
                    }
                    if engine.scene_to_open.is_some() {
                        scene_handler.opened_scene.close(&mut engine);
                        engine.delta_time = 0.0;
                        scene_handler.opened_scene = engine.scene_to_open.take().unwrap();
                        scene_handler.opened_scene.loaded(&mut engine);
                    }

                    scene_handler.opened_scene.process_input(&mut engine);

                    /*
                    If add returns true a second has passed!
                     */
                    if fps_calc.add(&mut engine) {
                        scene_handler.opened_scene.handle_second(&mut engine);
                    }

                    scene_handler.opened_scene.update(&mut engine);

                    let frame_result = engine.surface.get_current_texture();

                    if frame_result.is_err() {
                        match frame_result.unwrap_err() {
                            wgpu::SurfaceError::Lost => {
                                let size = engine.size.clone();
                                scene_handler.opened_scene.window_resized(&mut engine,&size)
                            },
                            // The system is out of memory, we should probably quit
                            wgpu::SurfaceError::OutOfMemory => *control_flow = ControlFlow::Exit,
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            e => eprintln!("{:?}", e),
                        }
                    } else {

                        let frame = frame_result.unwrap();

                        {
                            let mut encoder = engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            });
                            {
                                let view = frame.texture.create_view(&TextureViewDescriptor::default());

                                {
                                    //gui_handler.render(&mut engine,&mut scene_handler.opened_scene,&queue);
                                    scene_handler.opened_scene.render(&engine,&mut encoder,&view);
                                }
                            }
                            engine.queue.submit(std::iter::once(encoder.finish()));
                            frame.present();
                        }
                    }

                    engine.input_handler.reset();
                },Event::MainEventsCleared => {
                    engine.game_window.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}

struct DeltaTimeCalc {
    delta: f32
}

impl DeltaTimeCalc {

    pub fn new() -> DeltaTimeCalc{
        return DeltaTimeCalc {
            delta: 0.0
        }
    }

    pub fn update(&mut self,engine:&mut GameEngine) {
        self.delta = engine.time.elapsed().as_secs_f32();
        engine.time = Instant::now();
    }
}

struct FpsCalc {
    current_fps: i32,
    time: f32
}

impl FpsCalc {
    //Returns true if a second has passed in the game
    pub fn add(&mut self, engine:&mut GameEngine) -> bool{
        self.current_fps += 1;
        self.time += engine.delta_time;
        if self.time >= 1.0 {
            engine.fps = self.current_fps;
            self.time = 0.0;
            self.reset();
            return true
        }
        return false
    }

    pub fn reset(&mut self) {
        self.current_fps = 0;
    }

    pub fn new() -> FpsCalc {
        return FpsCalc { current_fps: 0, time: 0.0 }
    }
}