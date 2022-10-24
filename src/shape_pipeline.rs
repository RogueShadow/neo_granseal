use std::num::NonZeroU32;
use crate::*;
use bytemuck::{Pod, Zeroable};
use rand::{Rng, SeedableRng};
use std::ops::Range;
use wgpu::{BindGroup, BindingType, BufferAddress, BufferBindingType, BufferSize, BufferUsages, DynamicOffset, IndexFormat, MultisampleState, ShaderStages, VertexStepMode};
use wgpu::BufferBindingType::Storage;
use wgpu::PolygonMode;
use crate::util::{Color, Point};

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
    pub fn pt(mut self, p: Point) -> Self {
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
    pub fn new(pos: Point, r: f32) -> Self {
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
    fn from(c: Color) -> Self {
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
    pub fn oval(mut self) -> Self {
        self.kind = 1;
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SSRObjectInfo {
    start_vertice: u32,
    end_vertice: u32,
    start_index: u32,
    end_index: u32,
}

pub struct SimpleShapeRenderPipeline {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
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
        let index_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Index Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false
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
            index_buffer,
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
        let data = self.data.as_ref().expect("Get data");

        self.vertex_buffer =
            core.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("SSRP Vertex Buffer"),
                contents: bytemuck::cast_slice(data.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        self.index_buffer =
            core.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("SSRP Index Buffer"),
                contents: bytemuck::cast_slice(data.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });

        core.queue.write_buffer(&self.trans_buffer, 0, bytemuck::cast_slice(data.transforms.as_slice()));
        core.queue.write_buffer(&self.mats_buffer, 0, bytemuck::cast_slice(data.materials.as_slice()));

        let output =
            core.surface.get_current_texture().expect("Get Surface Texture");
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
        render_pass.set_index_buffer(self.index_buffer.slice(..),IndexFormat::Uint32);
        render_pass.set_bind_group(0, &self.globals.bind_group, &[]);
        render_pass.set_bind_group(1, &self.data_bind_group,&[]);

        for (i,obj) in data.object_info.iter().enumerate() {
            let index = i as u32..i as u32 + 1;
            render_pass.draw_indexed(obj.start_index..obj.end_index,obj.start_vertice as i32,index);
        }

        drop(render_pass);
        core.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn set_data(&mut self, data: Box<dyn std::any::Any>) {
        let rd = data
            .downcast::<SSRRenderData>()
            .expect("Get Render Data");
        self.data = Some(*rd);
    }

    fn set_globals(&mut self, globals: GlobalUniforms) {
        self.globals = globals;
    }
}

#[derive(Clone, Debug)]
pub struct SSRRenderData {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    transforms: Vec<SSRTransform>,
    materials: Vec<SSRMaterial>,
    object_info: Vec<SSRObjectInfo>,
}

impl SSRRenderData {
    pub(crate) fn clear(&mut self) {
        self.transforms.clear();
        self.vertices.clear();
        self.indices.clear();
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
    pub pos: Point,
    pub rotation: f32,
    pub kind: i32,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl <'draw>SSRGraphics<'draw> {
    pub fn clear(&mut self) {
        self.fill = true;
        self.color = Color::WHITE;
        self.thickness = 1.0;
        self.data.clear();
        self.pos = Point::new(0.0, 0.0);
        self.rotation = 0.0;
        self.kind = 0;
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
                indices: vec![],
                transforms: vec![],
                materials: vec![],
                object_info: vec![],
            },
            pos: Point::new(0.0, 0.0),
            rotation: 0.0,
            kind: 0,
        }
    }
    fn pt_quad(p1: Point, p2: Point, p3: Point, p4: Point) -> Mesh {
        Mesh {
            vertices: vec![
            Vertex::new().pt(p1).uv(0.0,0.0),
            Vertex::new().pt(p2).uv(0.0,1.0),
            Vertex::new().pt(p3).uv(1.0,1.0),
            Vertex::new().pt(p4).uv(1.0,0.0)],
            indices: vec![0,1,2,2,3,0]
        }
    }
    fn pt_line_quad(start: Point, end: Point, width: f32) -> Mesh {
        let (start,end) = if start.y < end.y {
            (end,start)
        } else {(start,end)};
        let pi = std::f32::consts::PI;
        let dx = (start.x - end.x) * 2.0; // width of line
        let dy = (start.y - end.y) * 2.0; // height of line
        let a = (dx/dy).atan(); // line slope in radians
        let sa = a - (pi/2.0);
        let sa2 = a + (pi/2.0);

        let bump2 = Point::new(width * sa.sin(), width * sa.cos());
        let bump = Point::new(width * sa2.sin(), width * sa2.cos());

        let p1 = Point::new(start.x + bump2.x, start.y + bump2.y);
        let p2 =  Point::new(start.x + bump.x, start.y + bump.y);
        let p3 =  Point::new(end.x + bump.x, end.y + bump.y);
        let p4 = Point::new(end.x + bump2.x, end.y + bump2.y);

        Self::pt_quad(p1,p2,p3,p4)
    }
    pub fn line(&mut self, start: Point, end: Point) {
        self.draw_raw_vertices(Self::pt_line_quad(start,end,self.thickness),None,None,None);
    }
    pub fn rect(&mut self, pos: Point, size: Point) {
        if self.fill {
            let hx = size.x / 2.0;
            let hy = size.y / 2.0;
            let p1 = Point::new(-hx, -hy);
            let p2 = Point::new(-hx, hy);
            let p3 = Point::new(hx, hy);
            let p4 = Point::new(hx, -hy);
            let mesh = SSRGraphics::pt_quad(p1,p2,p3,p4);
            self.draw_raw_vertices(mesh,Some(pos),None,None);
        } else {
            let hx = size.x / 2.0;
            let hy = size.y / 2.0;
            let p1 = Point::new(pos.x-hx, pos.y-hy);
            let p2 = Point::new(pos.x-hx, pos.y+hy);
            let p3 = Point::new(pos.x+hx, pos.y+hy);
            let p4 = Point::new(pos.x+hx, pos.y-hy);
            self.poly(&vec![p1,p2,p3,p4,p1]);
        }
    }
    pub fn poly(&mut self, points: &Vec<Point>) {
        if self.fill == false {
            if points.len() > 2 {
                let mut i = 0;
                while i < points.len() - 1 {
                    self.line(points[i],points[i+1]);
                    i += 1;
                }
            } else if points.len() == 2 {
                self.line(points[0],points[1]);
            }
        } else {
            todo!("Filled Polygons not yet supported.")
        }
    }
    pub fn draw_raw_vertices(
        &mut self,
        mesh: Mesh,
        pos: Option<Point>,
        rot: Option<f32>,
        kind: Option<i32>,
    ) {
        let start_vertex = self.data.vertices.len();
        let start_index = self.data.indices.len();
        for v in mesh.vertices { self.data.vertices.push(v); }
        for i in mesh.indices { self.data.indices.push(i); }
        let end_vertex = self.data.vertices.len();
        let end_index = self.data.indices.len();
        let info = SSRObjectInfo {
            start_vertice: start_vertex as u32,
            end_vertice: end_vertex as u32,
            start_index: start_index as u32,
            end_index: end_index as u32,
        };
        let rotation = match rot {
            Some(r) => r,
            None => 0.0,
        };
        let transform = match pos {
            Some(p) => SSRTransform {
            x: self.pos.x + p.x,
            y: self.pos.y + p.y,
            r: self.rotation + rotation,
        },
            None => SSRTransform::new(self.pos,self.rotation + rotation)
        };
        let material = match kind {
            Some(1) => SSRMaterial::from(self.color).oval(),
            None => SSRMaterial::from(self.color),
            Some(_) => SSRMaterial::from(self.color),
        };
        self.data.transforms.push(transform);
        self.data.materials.push(material);
        self.data.object_info.push(info);
    }
    pub fn oval(&mut self, pos: Point, size: Point) {
        if self.fill {
            let hx = size.x / 2.0;
            let hy = size.y / 2.0;
            let p1 = Point::new(-hx, -hy);
            let p2 = Point::new(-hx, hy);
            let p3 = Point::new(hx, hy);
            let p4 = Point::new(hx, -hy);
            let verts = SSRGraphics::pt_quad(p1,p2,p3,p4);
            self.draw_raw_vertices(verts,Some(pos),None, Some(1));
        } else {
            todo!("un-filled ovals, not yet supported.")
        }
    }
    pub fn finish(&mut self) {
        self.core.cmd(core::NGCommand::Render(0, Box::new(self.data.to_owned())))
    }
}

