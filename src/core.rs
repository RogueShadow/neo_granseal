use crate::events::Key;
use crate::math::Vec2;
use crate::mesh::Mesh;
use crate::shape_pipeline::{BufferedObjectID, MeshBuffer, SSRObjectInfo};
use crate::{map_present_modes, GransealGameConfig, NGRenderPipeline};
use image::{EncodableLayout, ImageError};
use pollster::FutureExt;
use std::any::Any;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::CreateSurfaceError;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::Fullscreen;

#[derive(Debug)]
pub enum NGError {
    OsError(winit::error::OsError),
    WgpuError(wgpu::RequestDeviceError),
    NoAdapterFound,
    NoFormatFound,
    NoPipeline,
    NoCommand,
    CreateSurfaceError(wgpu::CreateSurfaceError),
    SurfaceError(wgpu::SurfaceError),
    ImageError(image::ImageError),
}
impl From<wgpu::CreateSurfaceError> for NGError {
    fn from(value: CreateSurfaceError) -> Self {
        NGError::CreateSurfaceError(value)
    }
}
impl From<image::ImageError> for NGError {
    fn from(value: ImageError) -> Self {
        NGError::ImageError(value)
    }
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
    CustomEvent(String),
    RenderImage(usize, Box<dyn Any>, Image, bool),
}

pub struct MouseState {
    pub pos: Vec2,
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
impl Default for EngineState {
    fn default() -> Self {
        Self {
            mouse: MouseState {
                pos: Vec2::new(0, 0),
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

pub struct TextureInfo {
    pub(crate) texture: wgpu::Texture,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

#[derive(Copy, Clone, Debug)]
pub struct Image {
    pub(crate) texture: usize,
    pub size: Vec2,
    pub sub_image: Option<(Vec2, Vec2)>,
}
impl Image {
    pub fn sub_image(mut self, start: Vec2, size: Vec2) -> Self {
        self.sub_image = Some((start, start + size));
        self
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
    pub(crate) textures: Vec<TextureInfo>,
}

impl NGCore {
    fn initialize_texture(&mut self) {
        let mut image = image::RgbaImage::new(16, 16);
        image.fill(u8::MAX);
        let data = image.as_bytes();
        self.load_image_data(image.width(), image.height(), data);
    }
    pub fn load_image_from_memory(&mut self, data: &[u8]) -> Result<Image, NGError> {
        let image = image::load_from_memory(data)?.to_rgba8();
        Ok(self.load_image_data(image.width(), image.height(), image.as_bytes()))
    }
    pub fn load_image_data(&mut self, width: u32, height: u32, data: &[u8]) -> Image {
        let tex = wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let texture = self
            .device
            .create_texture_with_data(&self.queue, &tex, data);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        self.textures.push(TextureInfo {
            texture,
            bind_group_layout,
            bind_group,
        });
        Image {
            texture: self.textures.len() - 1,
            size: Vec2::new(width, height),
            sub_image: None,
        }
    }
    pub fn load_image(&mut self, file: &str) -> Result<Image, NGError> {
        let image = match image::open(file) {
            Ok(image) => image.to_rgba8(),
            Err(err) => return Err(NGError::ImageError(err)),
        };
        let data = image.as_raw().as_slice();
        Ok(self.load_image_data(image.width(), image.height(), data))
    }
    pub fn create_image(&mut self, width: u32, height: u32) -> Image {
        let mut image = image::RgbaImage::new(width, height);
        image.fill(u8::MAX);
        self.load_image_data(image.width(), image.height(), image.as_raw().as_slice())
    }
    pub fn update_buffer_object(&mut self, slot: usize, mesh: &Mesh) -> bool {
        if self.mesh_buffers.get(slot).is_some() {
            let bo = &mut self.mesh_buffers[slot];
            bo.vertex_buffer.destroy();
            bo.index_buffer.destroy();

            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Buffered Object Vertices"),
                    contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Buffered Object Indices"),
                    contents: bytemuck::cast_slice(mesh.indices.as_slice()),
                    usage: wgpu::BufferUsages::INDEX,
                });
            bo.vertex_buffer = vertex_buffer;
            bo.index_buffer = index_buffer;
            bo.texture = match mesh.image {
                Some(image) => Some(image.texture),
                None => None,
            };
            true
        } else {
            false
        }
    }
    pub fn buffer_object(&mut self, mesh: &Mesh) -> BufferedObjectID {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(mesh.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });
        let bo_slot = self.mesh_buffers.len();
        let object_info = SSRObjectInfo {
            bo_slot: Some(bo_slot),
            start_vertice: 0,
            start_index: 0,
            end_index: 0,
            texture: match mesh.image {
                Some(image) => Some(image.texture),
                None => None,
            },
        };
        self.mesh_buffers.push(MeshBuffer {
            vertex_buffer,
            index_buffer,
            texture: match mesh.image {
                Some(image) => Some(image.texture),
                None => None,
            },
        });
        self.buffered_objects.push(object_info);
        self.buffered_objects.len() - 1
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
    pub fn render_image(
        &mut self,
        pipeline: usize,
        data: Box<dyn Any>,
        image: &Image,
        replace: bool,
    ) {
        self.cmd_queue.push(NGCommand::RenderImage(
            pipeline,
            data,
            image.clone(),
            replace,
        ));
    }
    pub fn event(&mut self, event: String) {
        self.cmd_queue.push(NGCommand::CustomEvent(event));
    }
    pub fn set_title(&mut self, title: String) {
        self.cmd_queue.push(NGCommand::SetTitle(title));
    }
    pub fn set_cursor_visibility(&mut self, visible: bool) {
        self.cmd_queue.push(NGCommand::SetCursorVisibility(visible))
    }
    pub fn new(
        event_loop: &EventLoop<()>,
        mut config: GransealGameConfig,
    ) -> Result<Self, NGError> {
        let timer = std::time::Instant::now();
        let window = winit::window::WindowBuilder::new()
            .with_title(&config.title)
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build(event_loop)?;
        if config.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window)? };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .ok_or(NGError::NoAdapterFound)?;
        let caps = surface.get_capabilities(&adapter);
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: map_present_modes(config.vsync),
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![caps.formats[0]],
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY
                        | wgpu::Features::BUFFER_BINDING_ARRAY
                        | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    limits: Default::default(),
                },
                None,
            )
            .block_on()?;
        surface.configure(&device, &surface_configuration);
        let state = EngineState::default();

        // change config to whatever size we actually ended up with.
        let size = window.inner_size();
        config.width = size.width as i32;
        config.height = size.height as i32;

        let mut core = Self {
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
            textures: vec![],
        };

        core.initialize_texture();
        Ok(core)
    }
}
