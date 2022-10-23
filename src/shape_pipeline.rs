use std::num::NonZeroU32;
use crate::events::Key::V;
use crate::*;
use bytemuck::{Pod, Zeroable};
use rand::{Rng, SeedableRng};
use std::ops::Range;
use wgpu::{BindGroup, BindingType, BufferAddress, BufferBindingType, BufferSize, BufferUsages, DynamicOffset, MultisampleState, ShaderStages, VertexStepMode};
use wgpu::BufferBindingType::Storage;
use wgpu::PolygonMode::Point;
use crate::util::{Color, Point2d};

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
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn xy(mut self,x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn uv(mut self,u: f32, v: f32) -> Self {
        self.u = u;
        self.v = v;
        self
    }
    pub fn pt(mut self, p: Point2d) -> Self {
        self.x = p.x;
        self.y = p.y;
        self
    }
}
pub struct MeshBuilder {
    verts: Vec<Vertex>
}
impl  MeshBuilder {
    pub fn new() -> Self {
        Self {
            verts: vec![]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug,Zeroable,Pod)]
pub struct SSRTransform {
    x: f32,
    y: f32,
    r: f32,
}
impl SSRTransform {
    pub fn new(pos: Point2d, r: f32) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            r,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug,Zeroable,Pod)]
pub struct SSRMaterial {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    pub kind: i32,
}

impl SSRMaterial {
    fn from(c: &Color) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
            kind: 0,
        }
    }
    pub fn rgba(r: f32, g: f32, b: f32,a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a,
            kind: 0,
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct SSRObjectInfo {
    start_vertice: u32,
    end_vertice: u32,
}

pub struct SimpleShapeRenderPipeline {
    vertex_buffer: wgpu::Buffer,
    trans_buffer: wgpu::Buffer,
    mats_buffer: wgpu::Buffer,
    data_bind_group: wgpu::BindGroup,
    clear_color: [f64; 4],
    globals: GlobalUniforms,
    data: Option<SSRRenderData>,
    pipeline: wgpu::RenderPipeline,
    update_buffers: bool,
}
impl SimpleShapeRenderPipeline {
    const MAX: usize = 1_000_000;
    pub fn new(core: &NGCore) -> Self {
        let max = SimpleShapeRenderPipeline::MAX as usize;
        let vertex_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Vertex Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        let trans_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Trans Buffer"),
            size: (max * std::mem::size_of::<SSRTransform>()) as BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let mats_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Materials Buffer"),
            size: (max * std::mem::size_of::<SSRMaterial>()) as BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let data_bgl = core.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Transforms Materials BGL"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage {
                            read_only: true
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage {
                            read_only: true
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                }
            ]
        });
        let data_bind_group = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transforms Materials BG"),
            layout: &data_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: trans_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: mats_buffer.as_entire_binding()
                }
            ]
        });
        let globals = crate::GlobalUniforms::new(&core);
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
                    &data_bgl,
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
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
                            step_mode: VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2],
                        }
                    ],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: Some(wgpu::Face::Back),
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
            trans_buffer,
            mats_buffer,
            data_bind_group,
            clear_color,
            globals,
            data: None,
            pipeline,
            update_buffers: true,
        }
    }
}
impl NGRenderPipeline for SimpleShapeRenderPipeline {
    fn render(&mut self, core: &mut NGCore) {
        if self.data.is_none() {
            return;
        };
        let data = self.data.as_ref().expect("Couldn't get data");

        self.vertex_buffer =
            core.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("SSRP Vertex Buffer"),
                contents: bytemuck::cast_slice(data.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        core.queue.write_buffer(&self.trans_buffer, 0, bytemuck::cast_slice(data.transforms.as_slice()));
        core.queue.write_buffer(&self.mats_buffer, 0, bytemuck::cast_slice(data.materials.as_slice()));

        let output =
            core.surface.get_current_texture().expect("Couldn't get Surface Texture.");
        let view =
            output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            core.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("SimpleShapeRenderPipeline Command Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("SimpleShapeRenderPipeline Render Pass"),
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

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.globals.bind_group, &[]);
        render_pass.set_bind_group(1, &self.data_bind_group,&[]);

        for (i,obj) in data.object_info.iter().enumerate() {
            let index = i as u32..i as u32 + 1;
            render_pass.draw(obj.start_vertice..obj.end_vertice, index);
        }

        drop(render_pass);
        core.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn set_data(&mut self, data: Box<dyn std::any::Any>) {
        let rd = data
            .downcast::<SSRRenderData>()
            .expect("Wrong type of data!");
        self.data = Some(*rd);
    }

    fn set_globals(&mut self, globals: GlobalUniforms) {
        self.globals = globals;
    }
}

#[derive(Clone, Debug)]
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


pub struct SSRGraphics<'draw> {
    core: &'draw mut NGCore,
    pub fill: bool,
    pub color: Color,
    pub thickness: f32,
    pub data: SSRRenderData,
    pub pos: Point2d,
    pub rotation: f32,
}

impl <'draw>SSRGraphics<'draw> {
    pub fn clear(&mut self) {
        self.fill = true;
        self.color = Color::WHITE;
        self.thickness = 1.0;
        self.data.clear();
        self.pos = Point2d::new(0.0,0.0);
        self.rotation = 0.0;
    }
    pub fn data(&self) -> &SSRRenderData {
        self.data.as_ref()
    }
    pub fn new(core: &'draw mut NGCore) -> Self {
        Self {
            core,
            fill: true,
            color: Color::WHITE,
            thickness:  1.0,
            data: SSRRenderData {
                vertices: vec![],
                transforms: vec![],
                materials: vec![],
                object_info: vec![],
            },
            pos: Point2d::new(0.0,0.0),
            rotation: 0.0,
        }
    }
    fn v_quad(x1: f32, y1: f32, x2: f32, y2: f32) -> [Vertex; 6] {
        [
            Vertex::new().xy(x1, y1),
            Vertex::new().xy(x1, y2),
            Vertex::new().xy(x2, y2),
            Vertex::new().xy(x2, y2),
            Vertex::new().xy(x2, y1),
            Vertex::new().xy(x1, y1),
        ]
    }
    fn pt_quad(p1: Point2d, p2: Point2d, p3: Point2d, p4: Point2d) -> [Vertex; 6] {
        [
            Vertex::new().pt(p1),
            Vertex::new().pt(p2),
            Vertex::new().pt(p3),
            Vertex::new().pt(p3),
            Vertex::new().pt(p4),
            Vertex::new().pt(p1),
        ]
    }
    pub fn line2(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.line(Point2d::new(x1,y1),Point2d::new(x2,y2));
    }
    pub fn line(&mut self, start: Point2d, end: Point2d) {
        let (start,end) = if start.y < end.y {
            (end,start)
        } else {(start,end)};
        let pi = std::f32::consts::PI;
        let dx = (start.x - end.x) * 2.0; // width of line
        let dy = (start.y - end.y) * 2.0; // height of line
        let a = (dx/dy).atan(); // line slope in radians
        let sa = a - (pi/2.0);
        let sa2 = a + (pi/2.0);

        let bump2 = Point2d::new(self.thickness * sa.sin(),self.thickness * sa.cos());
        let bump = Point2d::new(self.thickness * sa2.sin(), self.thickness * sa2.cos());

        let p1 = Point2d::new(start.x + bump2.x,start.y + bump2.y);
        let p2 =  Point2d::new(start.x + bump.x, start.y + bump.y);
        let p3 =  Point2d::new(end.x + bump.x, end.y + bump.y);
        let p4 = Point2d::new(end.x + bump2.x,end.y + bump2.y);

        let start_vertice = self.data.vertices.len() as u32;
        SSRGraphics::pt_quad(p1,p2,p3,p4).iter()
            .for_each(|v| self.data.vertices.push(*v));

        let end_vertice = self.data.vertices.len() as u32;
        let info = SSRObjectInfo {
            start_vertice,
            end_vertice
        };
        self.data.transforms.push(SSRTransform::new(self.pos,self.rotation));
        self.data.materials.push(SSRMaterial::from(&self.color));
        self.data.object_info.push(info);
    }
    pub fn rect2(&mut self, pos: Point2d, size: Point2d) {
        self.rect(pos.x,pos.y,size.x,size.y);
    }
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        if self.fill {
            let start_vertex = self.data.vertices.len();
            let hw = w / 2.0;
            let hh = h / 2.0;

            for v in SSRGraphics::v_quad(-hw, -hh, hw, hh) {
                self.data.vertices.push(v);
            }

            let end_vertex = self.data.vertices.len();

            let info = SSRObjectInfo {
                start_vertice: start_vertex as u32,
                end_vertice: end_vertex as u32,
            };

            self.data.transforms.push(SSRTransform::new(Point2d::new(x,y),self.rotation));
            self.data.materials.push(SSRMaterial::from(&self.color));
            self.data.object_info.push(info);
        } else {

        }
    }
    pub fn poly(&mut self, points: &Vec<Point2d>) {
        if self.fill == false {
            if points.len() > 2 {
                let mut i = 0;
                while i < (points.len() - 1) {
                    self.line(points[i], points[i + 1]);
                    i += 1;
                }
            } else if points.len() == 2 {
                self.line(points[0], points[1])
            }
        } else {

        }
    }
    pub fn oval(&mut self, x: f32, y: f32, w: f32, h: f32) {
        if self.fill {
            let start_vertex = self.data.vertices.len();
            let hw = w / 2.0;
            let hh = h / 2.0;

            for v in SSRGraphics::v_quad(-hw, -hh, hw, hh) {
                self.data.vertices.push(v);
            }

            let end_vertex = self.data.vertices.len();

            let info = SSRObjectInfo {
                start_vertice: start_vertex as u32,
                end_vertice: end_vertex as u32,
            };

            self.data.transforms.push(SSRTransform::new(self.pos,self.rotation));
            let mut material = SSRMaterial::from(self.color.as_ref());
            material.kind = 1;
            self.data.materials.push(material);
            self.data.object_info.push(info);
        } else {

        }
    }
    pub fn finish(&mut self) {
        self.core.cmd(core::NGCommand::Render(0, Box::new(self.data.to_owned())))
    }
}

