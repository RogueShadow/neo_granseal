use crate::shape_pipeline::SimpleShapeRenderPipeline;
use crate::{map_present_modes, GransealGameConfig, NGRenderPipeline, SSRRenderData};
use pollster::FutureExt;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;

pub enum NGCommand {
    AddPipeline(Box<dyn NGRenderPipeline>),
    Render(usize, Box<dyn std::any::Any>),
    GetFps,
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
}

impl NGCore {
    pub fn cmd(&mut self, cmd: NGCommand) {
        self.cmd_queue.push(cmd);
    }
    pub fn new(event_loop: &EventLoop<()>, config: GransealGameConfig) -> Self {
        let timer = std::time::Instant::now();
        let window = winit::window::WindowBuilder::new()
            .with_title(&config.title)
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .build(&event_loop)
            .expect("Failed to build window");
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .block_on()
            .expect("Failed to create adapter");
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_supported_formats(&adapter)
                .pop()
                .expect("Failed to get surface format."),
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: map_present_modes(config.vsync),
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: Default::default(),
                    limits: Default::default(),
                },
                None,
            )
            .block_on()
            .expect("Failed to create device");
        surface.configure(&device, &surface_configuration);
        Self {
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
        }
    }
}
