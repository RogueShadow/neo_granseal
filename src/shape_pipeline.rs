use crate::events::Key::V;
use crate::*;
use bytemuck::{Pod, Zeroable};
use std::ops::Range;
use wgpu::{BufferAddress, DynamicOffset, MultisampleState};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
}
impl Vertex {
    pub fn new_xy(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            z: 0.0,
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C, align(256))]
#[derive(Copy, Clone, Debug)]
pub struct SSRTransform {
    x: f32,
    y: f32,
    rotation: f32,
}
impl SSRTransform {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            rotation: 0.0,
        }
    }
}
#[repr(C, align(256))]
#[derive(Copy, Clone, Debug)]
pub struct SSRMaterial {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    kind: i32,
}
impl SSRMaterial {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: 1.0,
            kind: 0,
        }
    }
}

pub struct SSRLocals {
    transform_buffer: wgpu::Buffer,
    material_buffer: wgpu::Buffer,
    transform_bgl: wgpu::BindGroupLayout,
    transform_bg: wgpu::BindGroup,
    material_bgl: wgpu::BindGroupLayout,
    material_bg: wgpu::BindGroup,
}
impl SSRLocals {
    const MAX_OBJECTS: BufferAddress = 1 << 20;
    pub fn new(core: &NGCore) -> Self {
        let uniform_alignment =
            core.device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
        let transform_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Transform Buffer"),
            size: SSRLocals::MAX_OBJECTS * uniform_alignment,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        let transform_bgl =
            core.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("SSR Transform Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<SSRTransform>() as _,
                            ),
                        },
                        count: None,
                    }],
                });
        let transform_bg = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SSR Transform Bind Group"),
            layout: &transform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &transform_buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(std::mem::size_of::<SSRTransform>() as _),
                }),
            }],
        });
        let material_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Material Buffer"),
            size: SSRLocals::MAX_OBJECTS * uniform_alignment,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        let material_bgl = core
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("SSR Material Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<SSRMaterial>() as _
                        ),
                    },
                    count: None,
                }],
            });
        let material_bg = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SSR Material Bind Group"),
            layout: &material_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &material_buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(std::mem::size_of::<SSRMaterial>() as _),
                }),
            }],
        });
        Self {
            transform_buffer,
            material_buffer,
            transform_bgl,
            transform_bg,
            material_bgl,
            material_bg,
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct SSRObjectInfo {
    start_vertice: u32,
    end_vertice: u32,
}

pub struct SimpleShapeRenderPipeline {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    clear_color: [f64; 4],
    globals: GlobalUniforms,
    locals: SSRLocals,
    data: Option<SSRRenderData>,
    pipeline: wgpu::RenderPipeline,
}
impl SimpleShapeRenderPipeline {
    pub fn new(core: &NGCore) -> Self {
        let vertex_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Vertex Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        let index_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Index Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });

        let globals = crate::GlobalUniforms::new(&core);
        let locals = SSRLocals::new(&core);
        let clear_color = core.config.clear_color;

        let shader = core
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("SSR Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shape_shader.wgsl").into()),
            });

        let pipeline_layout = core
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("SSR Pipeline Layout"),
                bind_group_layouts: &[
                    &globals.bind_group_layout,
                    &locals.transform_bgl,
                    &locals.material_bgl,
                ],
                push_constant_ranges: &[],
            });
        let pipeline = core
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("SSR Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: core.surface_configuration.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });
        Self {
            vertex_buffer,
            index_buffer,
            clear_color,
            globals,
            locals,
            data: None,
            pipeline,
        }
    }
}
impl NGRenderPipeline for SimpleShapeRenderPipeline {
    fn render(&mut self, core: &mut NGCore) {
        if self.data.is_none() { return; };
        let data = self.data.as_ref().expect("Couldn't get data");

        let uniform_alignment =
            core.device.limits().min_uniform_buffer_offset_alignment as DynamicOffset;

        let output = core
            .surface
            .get_current_texture()
            .expect("Couldn't get Surface Texture.");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = core
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("SimpleShapeRenderPipeline Command Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("SimpeShapeRenderPipeline Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: core.config.clear_color[0],
                            g: core.config.clear_color[1],
                            b: core.config.clear_color[2],
                            a: core.config.clear_color[3],
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            self.vertex_buffer =
                core.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("SSRP Vertex Buffer"),
                        contents: bytemuck::cast_slice(data.vertices.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
            core.queue
                .write_buffer(&self.locals.transform_buffer, 0, unsafe {
                    std::slice::from_raw_parts(
                        data.transforms.as_ptr() as *const u8,
                        data.transforms.len() * uniform_alignment as usize,
                    )
                });
            core.queue
                .write_buffer(&self.locals.material_buffer, 0, unsafe {
                    std::slice::from_raw_parts(
                        data.materials.as_ptr() as *const u8,
                        data.materials.len() * uniform_alignment as usize,
                    )
                });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.globals.bind_group, &[]);

            for (i, obj) in data.object_info.iter().enumerate() {
                let offset = (i as DynamicOffset) * uniform_alignment;
                render_pass.set_bind_group(1, &self.locals.transform_bg, &[offset]);
                render_pass.set_bind_group(2, &self.locals.material_bg, &[offset]);
                render_pass.draw(obj.start_vertice..obj.end_vertice, 0..1);
            }
        }
        core.queue.submit(Some(encoder.finish()));
        output.present();
    }

    fn set_data(&mut self, data: Box<dyn std::any::Any>) {
        let rd = data.downcast::<SSRRenderData>().expect("");
        self.data = Some(*rd);
    }

    fn set_globals(&mut self, globals: GlobalUniforms) {
        self.globals = globals;
    }
}


pub trait RenderData {}

#[derive(Clone,Debug)]
pub struct SSRRenderData {
    vertices: Vec<Vertex>,
    transforms: Vec<SSRTransform>,
    materials: Vec<SSRMaterial>,
    object_info: Vec<SSRObjectInfo>,
}

impl SSRRenderData {
    pub(crate) fn clear(&mut self) {
        self.transforms.clear();
        self.vertices.clear();
        self.materials.clear();
        self.object_info.clear();
    }
}

impl AsRef<SSRRenderData> for SSRRenderData {
    fn as_ref(&self) -> &SSRRenderData {
        self
    }
}

impl RenderData for  SSRRenderData {}

pub struct SSRGraphics {
    pub data: SSRRenderData,
    current_transform: SSRTransform,
    current_material: SSRMaterial,
}
impl SSRGraphics {
    pub fn data(&self) -> &SSRRenderData {
        self.data.as_ref()
    }
    pub fn new() -> Self {
        Self {
            data: SSRRenderData {
                vertices: vec![],
                transforms: vec![],
                materials: vec![],
                object_info: vec![]
            },
            current_transform: SSRTransform {
                x: 0.0,
                y: 0.0,
                rotation: 0.0,
            },
            current_material: SSRMaterial {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
                kind: 0
            }
        }
    }
    fn v_quad(x1: f32, y1: f32, x2: f32, y2: f32) -> [Vertex; 6] {
        [
            Vertex::new_xy(x1,y1),
            Vertex::new_xy(x1,y2),
            Vertex::new_xy(x2,y2),
            Vertex::new_xy(x2,y2),
            Vertex::new_xy(x2,y1),
            Vertex::new_xy(x1,y1),
        ]
    }
    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let start_vertex = self.data.vertices.len();
        let hw = w / 2.0;
        let hh = h / 2.0;

        for v in SSRGraphics::v_quad(-hw,-hh,hw,hh) {
            self.data.vertices.push(v);
        }

        let end_vertex = self.data.vertices.len();

        let info = SSRObjectInfo {
            start_vertice: start_vertex as u32,
            end_vertice: end_vertex as u32,
        };

        let transform = SSRTransform {
            x: self.current_transform.x + x,
            y: self.current_transform.y + y,
            rotation: self.current_transform.rotation
        };
        self.data.transforms.push(transform);
        self.data.materials.push(self.current_material);
        self.data.object_info.push(info);
    }
    pub fn fill_rgba(&mut self, r: f32, g: f32, b: f32, a: f32)  {
        self.current_material = SSRMaterial {
            r,
            g,
            b,
            a,
            kind: 0
        }
    }
}