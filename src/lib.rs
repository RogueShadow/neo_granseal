pub mod core;
pub mod events;
pub mod main_loop;
pub mod shape_pipeline;
pub mod util;

use crate::core::{NGCore, NGError};
use crate::main_loop::main_loop;
use image::EncodableLayout;
use wgpu::util::DeviceExt;
use winit::event_loop::{ EventLoopBuilder};
use crate::util::Color;

#[derive(Clone, Debug)]
pub struct GransealGameConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub vsync: VSyncMode,
    pub clear_color: Color,
    pub simple_pipeline: bool,
    pub msaa: MSAA,
}
impl GransealGameConfig {
    pub fn new() -> Self {
        Self {
            title: "Neo Granseal Engine".to_string(),
            width: 800,
            height: 600,
            vsync: VSyncMode::AutoVsync,
            clear_color: Color::DARK_ORANGE,
            simple_pipeline: true,
            msaa: MSAA::Enable4x,
        }
    }
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
    pub fn vsync(mut self, mode: bool) -> Self {
        self.vsync = match mode {true => VSyncMode::AutoVsync,false => VSyncMode::AutoNoVsync};
        self
    }
    pub fn clear_color(mut self, color: Color) -> Self {
        self.clear_color = color;
        self
    }
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn msaa(mut self,v: MSAA) -> Self {
        self.msaa = v;
        self
    }
}
#[derive(Clone, Debug, Copy)]
pub enum MSAA {
    Disabled,
    Enable4x,
    // Enable8x,
    // Enable16x,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VSyncMode {
    AutoVsync,
    AutoNoVsync,
    // VSyncOn,
    // AdaptiveVSync,
    // VSyncOff,
    // FastVSync,
}

pub fn map_present_modes(mode: VSyncMode) -> wgpu::PresentMode {
    match mode {
        VSyncMode::AutoVsync => wgpu::PresentMode::AutoVsync,
        VSyncMode::AutoNoVsync => wgpu::PresentMode::AutoNoVsync,
        // VSyncMode::VSyncOn => wgpu::PresentMode::Fifo,
        // VSyncMode::AdaptiveVSync => wgpu::PresentMode::FifoRelaxed,
        // VSyncMode::VSyncOff => wgpu::PresentMode::Immediate,
        // VSyncMode::FastVSync => wgpu::PresentMode::Mailbox,
    }
}

pub trait NeoGransealEventHandler {
    fn event(&mut self, core: &mut NGCore, e: events::Event);
}

pub trait NGRenderPipeline {
    fn render(&mut self, core: &mut NGCore) -> Result<(),NGError>;
    fn set_data(&mut self, data: Box<dyn std::any::Any>);
    fn set_globals(&mut self, globals: GlobalUniforms);
}

pub struct GlobalUniforms {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}
impl GlobalUniforms {
    fn new(core: &NGCore) -> Self {
        let screen_uniform_buffer =
            core.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Screen Uniform Buffer"),
                    contents: bytemuck::cast_slice(
                        [
                            core.window.inner_size().width as f32,
                            core.window.inner_size().height as f32,
                        ]
                        .as_bytes(),
                    ),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        let time_uniform_buffer =
            core.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Time Uniform Buffer"),
                    contents: bytemuck::cast_slice(
                        core.timer.elapsed().as_secs_f32().to_ne_bytes().as_slice(),
                    ),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        let bind_group_layout =
            core.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Global Uniforms Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
        let bind_group = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Global Uniforms Bind Group Layout"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: time_uniform_buffer.as_entire_binding(),
                },
            ],
        });
        Self {
            bind_group_layout,
            bind_group,
        }
    }
}

pub fn start<T>(handler: T, config: GransealGameConfig)
where
    T: 'static + NeoGransealEventHandler,
{
    let event_loop = EventLoopBuilder::new().build();
    let core = NGCore::new(&event_loop, config).expect("Initializing Core");
    main_loop(event_loop, core, Box::new(handler));
}

#[cfg(test)]
mod tests {
    use super::*;
}
