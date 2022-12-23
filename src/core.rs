use crate::events::Key;
use crate::mesh::Mesh;
use crate::shape_pipeline::{BufferedObjectID, MeshBuffer, SSRObjectInfo, Vertex};
use crate::{map_present_modes, GransealGameConfig, NGRenderPipeline};
use pollster::FutureExt;
use std::any::Any;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::{BufferUsages, Features};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;

#[derive(Debug)]
pub enum NGError {
    OsError(winit::error::OsError),
    WgpuError(wgpu::RequestDeviceError),
    NoAdapterFound,
    NoFormatFound,
    NoPipeline,
    NoCommand,
    SurfaceError(wgpu::SurfaceError),
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
    Render(usize, Box<dyn Any>),
    SetCursorVisibility(bool),
    SetTitle(String),
}

pub struct MouseState {
    pub pos: crate::math::Vec2,
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}
// It's so common to check if buttons are held down, let's add that right in.
pub struct EngineState {
    pub mouse: MouseState,
    pub fps: i32,
    pub(crate) keys: HashMap<Key, bool>,
    pub rotation: f32,
    pub scale: f32,
    pub xpos: f32,
    pub ypos: f32,
}
impl EngineState {
    pub fn new() -> Self {
        Self {
            mouse: MouseState {
                pos: crate::math::Vec2::new(0, 0),
                left: false,
                right: false,
                middle: false,
            },
            fps: 0,
            keys: HashMap::new(),
            rotation: 0.0,
            scale: 1.0,
            xpos: 0.0,
            ypos: 0.0,
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
    pub(crate) cmd_queue: Vec<NGCommand>,
    pub state: EngineState,
    pub(crate) mesh_buffers: Vec<MeshBuffer>,
    pub(crate) buffered_objects: Vec<SSRObjectInfo>,
}

impl NGCore {
    pub fn buffer_object(&mut self, slot: usize, mesh: Mesh) -> BufferedObjectID {
        if self.mesh_buffers.get(slot).is_some() {
            let mut bo = &mut self.mesh_buffers[slot];
            let mut vert_data: Vec<Vertex> = vec![];
            let mut i_data: Vec<u32> = vec![];
            bo.meshes.iter().for_each(|m| {
                vert_data.extend(&m.vertices);
                i_data.extend(&m.indices);
            });
            let start_vertice = vert_data.len() as u32;
            let start_index = i_data.len() as u32;
            let end_index = start_index + mesh.indices.len() as u32;
            vert_data.extend(&mesh.vertices);
            i_data.extend(&mesh.indices);
            bo.meshes.push(mesh);
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(vert_data.as_slice()),
                    usage: BufferUsages::VERTEX,
                });
            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(i_data.as_slice()),
                    usage: BufferUsages::INDEX,
                });
            bo.vertex_buffer.destroy();
            bo.vertex_buffer = vertex_buffer;
            bo.index_buffer.destroy();
            bo.index_buffer = index_buffer;
            let bo_slot = Some(slot);
            let object_info = SSRObjectInfo {
                bo_slot,
                start_vertice,
                start_index,
                end_index,
            };
            self.buffered_objects.push(object_info);
            self.buffered_objects.len() - 1
        } else {
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                });
            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(mesh.indices.as_slice()),
                    usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
                });
            let end_index = mesh.indices.len() as u32;
            let meshes = vec![mesh];
            let bo_slot = self.mesh_buffers.len();
            let object_info = SSRObjectInfo {
                bo_slot: Some(bo_slot),
                start_vertice: 0,
                start_index: 0,
                end_index,
            };
            self.mesh_buffers.push(MeshBuffer {
                vertex_buffer,
                index_buffer,
                meshes,
            });
            self.buffered_objects.push(object_info);
            self.buffered_objects.len() - 1
        }
    }
    pub fn key_held(&self, key: Key) -> bool {
        if !self.state.keys.contains_key(&key) {
            false
        } else {
            self.state.keys[&key]
        }
    }
    pub fn render(&mut self, pipeline: usize, data: Box<dyn Any>) {
        self.cmd_queue.push(NGCommand::Render(pipeline, data));
    }
    pub fn set_title(&mut self, title: String) {
        self.cmd_queue.push(NGCommand::SetTitle(title));
    }
    pub fn set_cursor_visibility(&mut self, visible: bool) {
        self.cmd_queue.push(NGCommand::SetCursorVisibility(visible))
    }
    pub fn new(event_loop: &EventLoop<()>, config: GransealGameConfig) -> Result<Self, NGError> {
        let timer = std::time::Instant::now();
        let window = winit::window::WindowBuilder::new()
            .with_title(&config.title)
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build(event_loop)?;
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .block_on()
            .ok_or(NGError::NoAdapterFound)?;
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_supported_formats(&adapter)
                .pop()
                .ok_or(NGError::NoFormatFound)?,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: map_present_modes(config.vsync),
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: Features::STORAGE_RESOURCE_BINDING_ARRAY
                        | Features::BUFFER_BINDING_ARRAY,
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
            mesh_buffers: vec![],
            buffered_objects: vec![],
        })
    }
}
