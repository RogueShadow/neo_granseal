use std::num::NonZeroU32;
use crate::*;
use bytemuck::{Pod, Zeroable};
use rand::{Rng, SeedableRng};
use std::ops::Range;
use crate::core::NGError;
use crate::util::{Color, Point};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Vertex {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            u: 0.0,
            v: 0.0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
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
    pub fn rgba(mut self, c: Color) -> Self {
        self.r = c.r;
        self.g = c.g;
        self.b = c.b;
        self.a = c.a;
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
    pub kind: i32,
}

impl SSRMaterial {
    fn new() -> Self {
        Self {
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
            size: (max * std::mem::size_of::<SSRTransform>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let mats_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Materials Buffer"),
            size: (max * std::mem::size_of::<SSRMaterial>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let data_bgl = core.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Transforms Materials BGL"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: true
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
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
        let globals = GlobalUniforms::new(&core);
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
                            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2,  2 => Float32x4],
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
                multisample: wgpu::MultisampleState::default(),
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
    fn render(&mut self, core: &mut NGCore) -> Result<(),NGError>{
        let data = match self.data.as_ref() {
            Some(d) => d,
            None => return Ok(()),
        };

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
            core.surface.get_current_texture()?;
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
        render_pass.set_index_buffer(self.index_buffer.slice(..),wgpu::IndexFormat::Uint32);
        render_pass.set_bind_group(0, &self.globals.bind_group, &[]);
        render_pass.set_bind_group(1, &self.data_bind_group,&[]);

        for (i,obj) in data.object_info.iter().enumerate() {
            let index = i as u32..i as u32 + 1;
            render_pass.draw_indexed(obj.start_index..obj.end_index,obj.start_vertice as i32,index);
        }

        drop(render_pass);
        core.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
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

#[derive(Copy,Clone,Debug)]
pub enum FillStyle {
    Solid(Color),
    FadeDown(Color,Color),
    FadeLeft(Color,Color),
    Corners(Color,Color,Color,Color),
}
#[derive(Copy,Clone,Debug)]
pub enum LineStyle {
    Center,
    Left,
    Right,
}

pub struct SSRGraphics<'draw> {
    core: &'draw mut NGCore,
    pub fill: bool,
    pub color: FillStyle,
    pub thickness: f32,
    pub line_style: LineStyle,
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
        self.color = FillStyle::Solid(Color::WHITE);
        self.thickness = 1.0;
        self.line_style = LineStyle::Center;
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
            color: FillStyle::Solid(Color::WHITE),
            thickness:  1.0,
            line_style: LineStyle::Center,
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
    fn pt_quad(&self, p1: Point, p2: Point, p3: Point, p4: Point) -> Mesh {
        let (c1,c2,c3,c4) = match self.color {
            FillStyle::Solid(color) => {(color,color,color,color)}
            FillStyle::FadeDown(color1, color2) => {(color2,color1,color1,color2)}
            FillStyle::FadeLeft(color1, color2) => {(color1,color1,color2,color2)}
            FillStyle::Corners(c1, c2, c3, c4) => {(c1,c2,c3,c4)}
        };
        Mesh {
            vertices: vec![
            Vertex::new().pt(p1).uv(0.0,0.0).rgba(c1),
            Vertex::new().pt(p2).uv(0.0,1.0).rgba(c2),
            Vertex::new().pt(p3).uv(1.0,1.0).rgba(c3),
            Vertex::new().pt(p4).uv(1.0,0.0).rgba(c4)],
            indices: vec![0,1,2,2,3,0]
        }
    }
    fn pt_line_quad(&self, start: Point, end: Point, width: f32) -> Mesh {
        let mut swap = if start.y <= end.y {-1.0} else {1.0};
        let pi = std::f32::consts::PI;
        let dx = (start.x - end.x) * 2.0; // width of line
        let dy = (start.y - end.y) * 2.0; // height of line
        let a = (dx/dy).atan(); // line slope in radians
        let sa = a - (pi/2.0);
        let sa2 = a + (pi/2.0);

        let bump2 = Point::new(width * sa.sin(), width * sa.cos());
        let bump = Point::new(width * sa2.sin(), width * sa2.cos());

        let (p1,p2,p3,p4) = match self.line_style {
            LineStyle::Center => {
            (
                Point::new(start.x + bump2.x, start.y + bump2.y),
                Point::new(start.x + bump.x, start.y + bump.y),
                Point::new(end.x + bump.x, end.y + bump.y),
                Point::new(end.x + bump2.x, end.y + bump2.y),
            )}
            LineStyle::Left => {
            (
                Point::new(start.x + bump2.x * 2.0 * swap, start.y + bump2.y * 2.0 * swap),
                Point::new(start.x, start.y),
                Point::new(end.x, end.y),
                Point::new(end.x + bump2.x * 2.0 * swap, end.y + bump2.y * 2.0 * swap),
            )}
            LineStyle::Right => {
            (
                Point::new(start.x, start.y),
                Point::new(start.x + bump.x * 2.0 * swap, start.y + bump.y * 2.0 * swap),
                Point::new(end.x + bump.x * 2.0 * swap, end.y + bump.y * 2.0 * swap),
                Point::new(end.x, end.y),
            )}
        };

        if swap < 0.0 {
            return self.pt_quad(p2,p1,p4,p3);
        } else {
            self.pt_quad(p1,p2,p3,p4)
        }
    }
    pub fn line(&mut self, start: Point, end: Point) {
        let mut mesh = self.pt_line_quad(start,end,self.thickness);
        //mesh.vertices.iter_mut().for_each(|v| {v.rgba(self.color)});
        let kind = self.kind;
        self.draw_raw_vertices(mesh,None,None,Some(kind))
    }
    pub fn rect(&mut self, pos: Point, size: Point) {
        if self.fill {
            let hx = size.x / 2.0;
            let hy = size.y / 2.0;
            let p1 = Point::new(-hx, -hy);
            let p2 = Point::new(-hx, hy);
            let p3 = Point::new(hx, hy);
            let p4 = Point::new(hx, -hy);
            let mesh = self.pt_quad(p1,p2,p3,p4);
            self.draw_raw_vertices(mesh,Some(pos),None,None);
        } else {
            let hx = size.x / 2.0;
            let hy = size.y / 2.0;
            let p1 = Point::new(-hx, -hy);
            let p2 = Point::new(-hx, hy);
            let p3 = Point::new(hx, hy);
            let p4 = Point::new(hx, -hy);
            let t = self.thickness;
            let ip1 = Point::new(-hx + t, -hy + t);
            let ip2 = Point::new(-hx + t, hy - t);
            let ip3 = Point::new(hx - t, hy - t);
            let ip4 = Point::new(hx - t, -hy + t);

            let (c1,c2,c3,c4) = match self.color {
                FillStyle::Solid(color) => {(color,color,color,color)}
                FillStyle::FadeDown(color1, color2) => {(color2,color1,color1,color2)}
                FillStyle::FadeLeft(color1, color2) => {(color1,color1,color2,color2)}
                FillStyle::Corners(c1, c2, c3, c4) => {(c1,c2,c3,c4)}
            };
            let vertices = vec![
                Vertex::new().pt(p1).uv(0.0,0.0).rgba(c1), // 0
                Vertex::new().pt(p2).uv(0.0,1.0).rgba(c2), // 1
                Vertex::new().pt(p3).uv(1.0,1.0).rgba(c3), // 2
                Vertex::new().pt(p4).uv(1.0,0.0).rgba(c4), // 3
                Vertex::new().pt(ip1).uv(0.0,0.0).rgba(c1),// 4
                Vertex::new().pt(ip2).uv(0.0,1.0).rgba(c2),// 5
                Vertex::new().pt(ip3).uv(1.0,1.0).rgba(c3),// 6
                Vertex::new().pt(ip4).uv(1.0,0.0).rgba(c4),// 7
            ];
            let indices = vec![
                0,1,4,   1,5,4,
                1,2,5,   5,2,6,
                2,3,6,   6,3,7,
                4,7,3,   4,3,0,
            ];

            let mesh = Mesh {
                vertices,
                indices,
            };

            self.draw_raw_vertices(mesh,Some(pos),None,None);
        }
    }
    pub fn poly(&mut self, points: &Vec<Point>) {
        if !self.fill  {
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
            Some(1) => SSRMaterial::new().oval(),
            None => SSRMaterial::new(),
            Some(_) => SSRMaterial::new(),
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
            let verts = self.pt_quad(p1,p2,p3,p4);
            self.draw_raw_vertices(verts,Some(pos),None, Some(1));
        } else {

        }
    }
    pub fn finish(&mut self) {
        self.core.cmd(core::NGCommand::Render(0, Box::new(self.data.to_owned())))
    }
}

