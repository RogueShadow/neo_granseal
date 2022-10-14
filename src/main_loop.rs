use std::ops::Index;
use env_logger::init;
use wgpu::util::DeviceExt;
use crate::{core::{NGCommand, NGCore}, shape_pipeline, events, GlobalUniforms};
use winit::{
    event_loop,
    event::{Event,WindowEvent}
};
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent::KeyboardInput;

pub(crate) fn main_loop(
    e_loop: event_loop::EventLoop<()>,
    mut core: NGCore,
    mut h: Box<dyn crate::NeoGransealEventHandler>,
) {
    let mut delta = std::time::Instant::now();
    let mut pipelines: Vec<Box<dyn crate::NGRenderPipeline>> = vec![];

    h.event(&mut core, events::Event::Load);

    e_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == core.window.id() => *control_flow = event_loop::ControlFlow::Exit,
            Event::WindowEvent {
                window_id,
                event: WindowEvent::KeyboardInput {
                    input,
                    ..
                }
            } if window_id == core.window.id() =>
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    *control_flow = event_loop::ControlFlow::Exit
                },
            Event::MainEventsCleared => {
                while !core.cmd_queue.is_empty() {
                    match core.cmd_queue.pop().expect("Couldn't get command.") {
                        NGCommand::AddPipeline(p) => {
                            pipelines.push(p);
                        }
                    };
                }
                core.window.request_redraw()
            },
            Event::RedrawRequested(id) => {
                h.event(&mut core, events::Event::Update(delta.elapsed()));
                delta = std::time::Instant::now();

                for p in pipelines.iter_mut() {
                    p.set_globals(GlobalUniforms::new(&core));
                }

                for p in pipelines.iter_mut() {
                    p.render(&mut core);
                }
            }
            _ => (),
        }
    });
}
