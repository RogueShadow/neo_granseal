use crate::events::{map_events, map_keys};
use crate::shape_pipeline::SimpleShapeRenderPipeline;
use crate::{
    core::{NGCommand, NGCore},
    events, GlobalUniforms,
};
use log::error;
use std::time::Duration;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::{
    event::{Event, WindowEvent},
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
    let one_second = Duration::from_secs(1);
    let mut frame_timer = std::time::Instant::now();
    let mut pipelines: Vec<Box<dyn crate::NGRenderPipeline>> = vec![];
    if core.config.simple_pipeline {
        pipelines.push(Box::new(SimpleShapeRenderPipeline::new(&core)));
    }

    h.event(&mut core, events::Event::Load);

    e_loop
        .run(move |event, window| {
            window.set_control_flow(event_loop::ControlFlow::Poll);
            while !core.cmd_queue.is_empty() {
                match core.cmd_queue.pop().unwrap() {
                    NGCommand::AddPipeline(p) => {
                        pipelines.push(p);
                    }
                    NGCommand::Render(index, data) => {
                        let size = {
                            let d = core.window.inner_size();
                            (d.width as f32, d.height as f32)
                        };
                        if index < pipelines.len() {
                            if !pipelines.is_empty() {
                                let renderer = pipelines.get_mut(index).unwrap();
                                renderer.set_globals(GlobalUniforms::new(&core, size));
                                renderer.set_data(data);
                                renderer.render(&mut core).expect("Render");
                            }
                        } else {
                            error!("Index out of bounds for pipeline.")
                        }
                        if frame_timer.elapsed() >= one_second {
                            frame_timer = std::time::Instant::now();
                            core.state.fps = frames;
                            frames = 0;
                        }
                        frames += 1;
                    }
                    NGCommand::RenderImage(index, data, img, replace) => {
                        if index < pipelines.len() {
                            if !pipelines.is_empty() {
                                let renderer = pipelines.get_mut(index).unwrap();
                                renderer.set_globals(GlobalUniforms::new(
                                    &core,
                                    (img.size.x, img.size.y),
                                ));
                                renderer.set_data(data);
                                renderer.render_image(&mut core, img, replace);
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
                    NGCommand::CustomEvent(event) => {
                        h.event(&mut core, events::Event::Custom(event));
                    }
                };
            }
            match event {
                Event::WindowEvent { event, window_id } if window_id == core.window.id() => {
                    if let Some(nge) = map_events(&event) {
                        h.event(&mut core, nge);
                    }
                    match event {
                        WindowEvent::RedrawRequested => {
                            if !core.window.is_minimized().unwrap() {
                                let elapsed = delta.elapsed();
                                delta = std::time::Instant::now();
                                h.event(&mut core, events::Event::Update(elapsed));
                                h.event(&mut core, events::Event::Draw);
                            }
                            core.window.request_redraw();
                        }
                        WindowEvent::CloseRequested => {
                            window.exit();
                        }
                        WindowEvent::KeyboardInput { event, .. } => match event {
                            KeyEvent {
                                physical_key,
                                state,
                                ..
                            } => match physical_key {
                                key => {
                                    let ng_key = map_keys(&key);
                                    core.state
                                        .keys
                                        .insert(ng_key, state == ElementState::Pressed);
                                    if key == winit::keyboard::KeyCode::Escape {
                                        window.exit();
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
                            MouseButton::Back => {}
                            MouseButton::Forward => {}
                        },
                        _ => {}
                    }
                }
                _ => (),
            }
        })
        .expect("Looop");
}
