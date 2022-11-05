use crate::{map_present_modes, GransealGameConfig, NGRenderPipeline};
use pollster::FutureExt;
use wgpu::{BufferUsages, Features};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use crate::mesh::Mesh;
use crate::shape_pipeline::{BufferedObjectID, SRBufferedObject, SSRObjectInfo};

#[derive(Debug)]
pub enum NGError {
    OsError(winit::error::OsError),
    WgpuError(wgpu::RequestDeviceError),
    NoAdapterFound,
    NoFormatFound,
    NoPipeline,
    NoCommand,
    SurfaceError(wgpu::SurfaceError)
}
impl From<winit::error::OsError> for NGError {
    fn from(e: winit::error::OsError) -> Self {
        NGError::OsError(e)
    }
}
impl From<wgpu::RequestDeviceError> for NGError {
    fn from(e: wgpu::RequestDeviceError) -> Self {
        NGError::WgpuError(e)
    }
}
impl From<wgpu::SurfaceError> for NGError {
    fn from(e: wgpu::SurfaceError) -> Self {
        NGError::SurfaceError(e)
    }
}
pub enum NGCommand {
    AddPipeline(Box<dyn NGRenderPipeline>),
    Render(usize, Box<dyn std::any::Any>),
    SetCursorVisibility(bool),
    SetTitle(String),
}

pub struct MouseState {
    pub pos: crate::util::Point,
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}
// It's so common to check if buttons are held down, let's add that right in.
pub struct EngineState {
    pub mouse: MouseState,
    pub fps: i32,
}
impl EngineState {
    pub fn new() -> Self {
        Self {
            mouse: MouseState {
                pos: crate::util::Point::new(0.0,0.0),
                left: false,
                right: false,
                middle: false,
            },
            fps: 0
        }
    }
}

pub struct NGCore {
    pub config: GransealGameConfig,
    pub timer: std::time::Instant,
    pub window: winit::window::Window,
    pub instance: wgpu::Instance,
    pub surface_configuration: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub cmd_queue: Vec<NGCommand>,
    pub state: EngineState,
    pub buffered_objects: Vec<SRBufferedObject>,
}

impl NGCore {
    pub fn buffer_object(&mut self, mesh: Mesh) -> BufferedObjectID {
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.indices.as_slice()),
            usage: BufferUsages::INDEX
        });
        let index = self.buffered_objects.len();
        let bo = SRBufferedObject {
            vertex_buffer,
            index_buffer,
            object_info: SSRObjectInfo {
                buffered_object: Some(index),
                start_vertice: 0,
                start_index: 0,
                end_index: mesh.indices.len() as u32,
            }
        };
        self.buffered_objects.push(bo);
        index
    }

    pub fn cmd(&mut self, cmd: NGCommand) {
        self.cmd_queue.push(cmd);
    }
    pub fn new(event_loop: &EventLoop<()>, config: GransealGameConfig) -> Result<Self,NGError> {
        let timer = std::time::Instant::now();
        let window = winit::window::WindowBuilder::new()
            .with_title(&config.title)
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build(&event_loop)?;
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .block_on().ok_or(NGError::NoAdapterFound)?;
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_supported_formats(&adapter)
                .pop().ok_or(NGError::NoFormatFound)?,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: map_present_modes(config.vsync),
            alpha_mode: wgpu::CompositeAlphaMode::Auto
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: Features::STORAGE_RESOURCE_BINDING_ARRAY | Features::BUFFER_BINDING_ARRAY,
                    limits: Default::default(),
                },
                None,
            )
            .block_on()?;
        surface.configure(&device, &surface_configuration);
        let state = EngineState::new();
        Ok(Self {
            config,
            timer,
            window,
            instance,
            surface_configuration,
            surface,
            adapter,
            device,
            queue,
            cmd_queue: vec![],
            state,
            buffered_objects: vec![],
        })
    }
}
