use crate::events::{map_events, map_keys};
use crate::shape_pipeline::SimpleShapeRenderPipeline;
use crate::{
    core::{NGCommand, NGCore},
    events, GlobalUniforms,
};
use log::error;
use winit::event::{ElementState, KeyboardInput, MouseButton};
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop,
};


pub(crate) fn main_loop(
    e_loop: event_loop::EventLoop<()>,
    mut core: NGCore,
    mut h: Box<dyn crate::NeoGransealEventHandler>,
) {
    env_logger::init();
    let mut delta = std::time::Instant::now();
    let mut frames = 0;
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
                NGCommand::Render(index, data) => {
                    if index < pipelines.len() {
                        if !pipelines.is_empty() {
                            pipelines.get_mut(index).unwrap().set_data(data);
                        }
                    } else {
                        error!("Index out of bounds for pipeline.")
                    }
                }
                NGCommand::SetCursorVisibility(v) => core.window.set_cursor_visible(v),
                NGCommand::SetTitle(title) => {
                    core.config.title = title;
                    core.window.set_title(core.config.title.as_str());
                }
            };
        }
        match event {
            Event::WindowEvent { event, window_id } if window_id == core.window.id() => {
                if let Some(nge) = map_events(&event) {
                    h.event(&mut core, nge);
                }
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        } => match virtual_keycode {
                            None => {}
                            Some(key) => {
                                let ng_key = map_keys(&key);
                                core.state
                                    .keys
                                    .insert(ng_key, state == ElementState::Pressed);
                                if key == VirtualKeyCode::Escape {
                                    *control_flow = event_loop::ControlFlow::Exit;
                                }
                            }
                        },
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        core.state.mouse.pos.x = position.x as f32;
                        core.state.mouse.pos.y = position.y as f32;
                    }
                    WindowEvent::MouseInput { button, state, .. } => match button {
                        MouseButton::Left => {
                            core.state.mouse.left = match state {
                                ElementState::Pressed => true,
                                ElementState::Released => false,
                            }
                        }
                        MouseButton::Right => {
                            core.state.mouse.right = match state {
                                ElementState::Pressed => true,
                                ElementState::Released => false,
                            }
                        }
                        MouseButton::Middle => {
                            core.state.mouse.middle = match state {
                                ElementState::Pressed => true,
                                ElementState::Released => false,
                            }
                        }
                        MouseButton::Other(_) => {}
                    },
                    _ => {}
                }
            }
            Event::MainEventsCleared => core.window.request_redraw(),
            Event::RedrawRequested(_) => {
                h.event(&mut core, events::Event::Update(delta.elapsed()));
                h.event(&mut core, events::Event::Draw);
                delta = std::time::Instant::now();
                pipelines.iter_mut().for_each(|p| {
                    p.set_globals(GlobalUniforms::new(&core));
                    p.render(&mut core).expect("Render");
                });
                if frame_timer.elapsed().as_secs_f64() > 1.0 {
                    frame_timer = std::time::Instant::now();
                    core.state.fps = frames;
                    frames = 0;
                }
                frames += 1;
            }
            _ => (),
        }
    });
}
