use crate::events::Key;
use crate::math::{vec2, Vec2};
use crate::mesh::Mesh;
use crate::shape_pipeline::{BufferedObjectID, MeshBuffer, SSRObjectInfo};
use crate::{map_present_modes, GransealGameConfig, NGRenderPipeline};
use image::EncodableLayout;
use pollster::FutureExt;
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use wgpu::util::DeviceExt;
use winit::dpi::{LogicalSize, PhysicalSize};
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
    TextureOverload,
}
impl From<wgpu::CreateSurfaceError> for NGError {
    fn from(value: wgpu::CreateSurfaceError) -> Self {
        NGError::CreateSurfaceError(value)
    }
}
impl From<image::ImageError> for NGError {
    fn from(value: image::ImageError) -> Self {
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
    CustomEvent(Box<dyn Any>),
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
        }
    }
}

pub struct TextureInfo {
    pub(crate) texture: wgpu::Texture,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Image {
    pub texture: usize,
    pub atlas: Option<(usize, Vec2)>,
    size: Vec2,
    pub sub_image: Option<(Vec2, Vec2)>,
}
pub const TEXTURE_SIZE: u32 = 8192;
impl Image {
    pub fn atlas_id(&self) -> Option<usize> {
        if let Some(atlas) = self.atlas {
            Some(atlas.0)
        } else {
            None
        }
    }
    pub fn texture_id(&self) -> Option<usize> {
        return if let Some(atlas_id) = self.atlas_id() {
            Some(atlas_id)
        } else {
            Some(self.texture)
        };
    }
    pub fn sub_image(mut self, start: Vec2, size: Vec2) -> Self {
        self.sub_image = Some((start, start + size));
        self
    }
    pub fn get_uv(&self) -> (Vec2, Vec2) {
        match (self.atlas, self.sub_image) {
            (Some((_, atlas_pos)), Some((sub_start, sub_end))) => (
                (atlas_pos + sub_start) / vec2(TEXTURE_SIZE, TEXTURE_SIZE),
                (atlas_pos + sub_end) / vec2(TEXTURE_SIZE, TEXTURE_SIZE),
            ),
            (Some((_, atlas_pos)), None) => (
                atlas_pos / vec2(TEXTURE_SIZE, TEXTURE_SIZE),
                (atlas_pos + self.size) / vec2(TEXTURE_SIZE, TEXTURE_SIZE),
            ),
            (None, Some((sub_start, sub_end))) => (sub_start / self.size, sub_end / self.size),
            (None, None) => (vec2(0, 0), vec2(1, 1)),
        }
    }
    pub fn size(&self) -> Vec2 {
        if let Some(sub_image) = self.sub_image {
            sub_image.1 - sub_image.0
        } else {
            self.size
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
    pub(crate) textures: Vec<TextureInfo>,
}

impl NGCore {
    fn initialize_texture(&mut self) {
        let mut image = image::RgbaImage::new(16, 16);
        image.fill(u8::MAX);
        let data = image.as_bytes();
        self.load_image_data(image.width(), image.height(), data, true);
    }
    pub fn load_image_from_memory(&mut self, data: &[u8], nearest: bool) -> Result<Image, NGError> {
        let image = image::load_from_memory(data)?.to_rgba8();
        Ok(self.load_image_data(image.width(), image.height(), image.as_bytes(), nearest))
    }
    pub fn load_image_data(
        &mut self,
        width: u32,
        height: u32,
        data: &[u8],
        nearest: bool,
    ) -> Image {
        let filter_mode = if nearest {
            wgpu::FilterMode::Nearest
        } else {
            wgpu::FilterMode::Linear
        };
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
            mag_filter: filter_mode,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: filter_mode,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
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
            atlas: None,
            sub_image: None,
        }
    }
    //TODO Make this an NGCommand so it does it later, after other things are done.
    pub fn destroy_image(&mut self, image: &Image) {
        self.textures[image.texture].texture.destroy();
    }
    pub fn load_image<P>(&mut self, file: P, nearest: bool) -> Result<Image, NGError>
    where
        P: AsRef<Path>,
    {
        let image = match image::open(file) {
            Ok(image) => image.to_rgba8(),
            Err(err) => return Err(NGError::ImageError(err)),
        };
        let data = image.as_raw().as_slice();
        Ok(self.load_image_data(image.width(), image.height(), data, nearest))
    }
    pub fn create_image(&mut self, width: u32, height: u32, nearest: bool) -> Image {
        let mut image = image::RgbaImage::new(width, height);
        image.fill(u8::MAX);
        self.load_image_data(
            image.width(),
            image.height(),
            image.as_raw().as_slice(),
            nearest,
        )
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
            texture: mesh.get_texture_id(),
        };
        self.mesh_buffers.push(MeshBuffer {
            vertex_buffer,
            index_buffer,
            texture: mesh.get_texture_id(),
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
    pub fn event<T>(&mut self, event: T)
    where
        T: 'static,
    {
        self.cmd_queue.push(NGCommand::CustomEvent(Box::new(event)));
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
            .with_resizable(true)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build(event_loop)?;
        if config.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        } else {
            if let Some(monitor) = window.primary_monitor() {
                let mut position = vec2(0, 0);
                let screen_size = vec2(monitor.size().width, monitor.size().height);
                let mut window_size = vec2(config.width, config.height);
                if screen_size.x <= window_size.x {
                    window_size.x = screen_size.x;
                } else {
                    position.x = (screen_size.x - window_size.x) / 2.0;
                }
                if screen_size.y <= window_size.y {
                    window_size.y = screen_size.y;
                } else {
                    position.y = (screen_size.y - window_size.y) / 2.0;
                    position.y -= position.y / 2.0; // prefer up a bit more, because taskbars exist.
                }
                window.set_outer_position(position);
                if let Some(size) =
                    window.request_inner_size(LogicalSize::new(window_size.x, window_size.y))
                {
                    config.width = size.width as i32;
                    config.height = size.height as i32;
                }
            }
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
                        | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                        | wgpu::Features::DEPTH32FLOAT_STENCIL8,
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
        println!("{:?}", &adapter.get_info());

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
