use std::ops::Index;
use std::rc::Rc;
use env_logger::init;
use wgpu::util::DeviceExt;
use crate::{core::{NGCommand, NGCore}, shape_pipeline, events, GlobalUniforms, SSRRenderData};
use winit::{
    event_loop,
    event::{Event,WindowEvent,VirtualKeyCode}
};
use crate::shape_pipeline::SSRGraphics;

pub(crate) fn main_loop(
    e_loop: event_loop::EventLoop<()>,
    mut core: NGCore,
    mut h: Box<dyn crate::NeoGransealEventHandler>,
) {
    let mut delta = std::time::Instant::now();
    let mut frames = 0;
    let mut fps = 0;
    let mut frame_timer = std::time::Instant::now();
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
                            println!("Added pipeline.");
                            pipelines.push(p);
                        },
                        NGCommand::GetFps => h.event(&mut core,events::Event::Fps(fps as u32)),
                        NGCommand::Render(index,data) => {
                            if !pipelines.is_empty() {
                                pipelines.get_mut(index).expect("Couldn't get render pipeline")
                                    .set_data(data);
                            }
                        },
                    };
                }
                core.window.request_redraw()
            },
            Event::RedrawRequested(_) => {
                h.event(&mut core, events::Event::Update(delta.elapsed()));
                h.event(&mut core, events::Event::Draw);
                delta = std::time::Instant::now();

                for p in pipelines.iter_mut() {
                    p.set_globals(GlobalUniforms::new(&core));
                }

                for p in pipelines.iter_mut() {
                    p.render(&mut core);
                }
                if frame_timer.elapsed().as_secs_f64() > 1.0 {
                    frame_timer = std::time::Instant::now();
                    fps = frames;
                    frames = 0;
                    println!("Fps: {}",fps);
                }
                frames += 1;
            }
            _ => (),
        }
    });
}
