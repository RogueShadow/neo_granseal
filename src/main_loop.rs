use crate::shape_pipeline::{SSRGraphics, SimpleShapeRenderPipeline, SSRTransform, SSRMaterial};
use crate::{
    core::{NGCommand, NGCore},
    events, shape_pipeline, GlobalUniforms, SSRRenderData,
};
use std::ops::{Deref, Index};
use std::rc::Rc;
use std::sync::Arc;
use log::{error, info};
use wgpu::util::DeviceExt;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop,
};
use winit::event::KeyboardInput;
use crate::core::NGError;
use crate::events::map_events;

pub(crate) fn main_loop(
    e_loop: event_loop::EventLoop<()>,
    mut core: NGCore,
    mut h: Box<dyn crate::NeoGransealEventHandler>,
) {
    env_logger::init();
    info!("Teest starting up.");
    let mut delta = std::time::Instant::now();
    let mut frames = 0;
    let mut fps = 0;
    let mut frame_timer = std::time::Instant::now();
    let mut pipelines: Vec<Box<dyn crate::NGRenderPipeline>> = vec![];
    if core.config.simple_pipeline {
        pipelines.push(Box::new(SimpleShapeRenderPipeline::new(&core)));
    }

    h.event(&mut core, events::Event::Load);

    e_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Poll;
        while !core.cmd_queue.is_empty() {
            match core.cmd_queue.pop().unwrap() {
                NGCommand::AddPipeline(p) => {
                    pipelines.push(p);
                }
                NGCommand::GetFps => h.event(&mut core, events::Event::Fps(fps as u32)),
                NGCommand::Render(index, data) => {
                    if index >= 0 && index <= pipelines.len() {
                        if !pipelines.is_empty() {
                            pipelines
                                .get_mut(index)
                                .unwrap()
                                .set_data(data);
                        }
                    } else {
                        error!("Index out of bounds for pipeline.")
                    }
                }
                NGCommand::SetCursorVisibility(v) => {core.window.set_cursor_visible(v)}
                NGCommand::SetTitle(title) => {
                    core.config.title = title;
                    core.window.set_title(core.config.title.as_str());
                }
            };
        }
        match event {
            Event::WindowEvent { event, window_id}
            if window_id == core.window.id() => {
                if let Some(nge) = map_events(&event) {
                    h.event(&mut core,nge);
                }
                match event {
                    WindowEvent::Resized(_) => {}
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested => {*control_flow = event_loop::ControlFlow::Exit;}
                    WindowEvent::Destroyed => {}
                    WindowEvent::DroppedFile(_) => {}
                    WindowEvent::HoveredFile(_) => {}
                    WindowEvent::HoveredFileCancelled => {}
                    WindowEvent::ReceivedCharacter(_) => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::KeyboardInput {input, ..} => {
                        match input {
                            KeyboardInput {virtual_keycode, state, ..} => {
                                match virtual_keycode {
                                    None => {}
                                    Some(key) => {
                                        match key {
                                            VirtualKeyCode::Escape => {*control_flow = event_loop::ControlFlow::Exit}
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    WindowEvent::ModifiersChanged(state) => {}
                    WindowEvent::Ime(_) => {}
                    WindowEvent::CursorMoved { position, .. } => {
                        h.event(
                            &mut core,events::Event::MouseMoved { position: [position.x,position.y]}
                        )
                    }
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {}
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { .. } => {}
                    WindowEvent::TouchpadPressure { .. } => {}
                    WindowEvent::AxisMotion { .. } => {}
                    WindowEvent::Touch(_) => {}
                    WindowEvent::ScaleFactorChanged { .. } => {}
                    WindowEvent::ThemeChanged(_) => {}
                    WindowEvent::Occluded(_) => {}
                }
            }
            Event::MainEventsCleared => core.window.request_redraw(),
            Event::RedrawRequested(_) => {
                h.event(&mut core, events::Event::Update(delta.elapsed()));
                h.event(&mut core, events::Event::Draw);
                delta = std::time::Instant::now();
                pipelines.iter_mut().for_each(|p| {
                    p.set_globals(GlobalUniforms::new(&core));
                    p.render(&mut core);
                });
                if frame_timer.elapsed().as_secs_f64() > 1.0 {
                    frame_timer = std::time::Instant::now();
                    fps = frames;
                    frames = 0;
                    println!("Fps: {}", fps);
                }
                frames += 1;
            }
            _ => (),
        }
    });
}
