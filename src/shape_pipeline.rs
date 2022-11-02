use bytemuck::{Pod, Zeroable};
use wgpu::util::{DeviceExt};
use crate::{Color, GlobalUniforms, MSAA, NGCore, NGError, NGRenderPipeline, Point};
use crate::core::NGCommand;

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
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            z: 0.0,
            u: 0.0,
            v: 0.0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
    pub fn uv(mut self,u: f32, v: f32) -> Self {
        self.u = u;
        self.v = v;
        self
    }
    pub fn point(p: Point) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: 0.0,
            u: 0.0,
            v: 0.0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
    pub fn rgba(mut self, c: Color) -> Self {
        self.r = c.r;
        self.g = c.g;
        self.b = c.b;
        self.a = c.a;
        self
    }
    pub fn set_color(&mut self, c: Color) {
        self.r = c.r;
        self.g = c.g;
        self.b = c.b;
        self.a = c.a;
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
    start_index: u32,
    end_index: u32,
}

pub struct SimpleShapeRenderPipeline {
    multisample: Option<wgpu::Texture>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    trans_buffer: wgpu::Buffer,
    mats_buffer: wgpu::Buffer,
    data_bind_group: wgpu::BindGroup,
    globals: GlobalUniforms,
    data: Option<SSRRenderData>,
    pipeline: wgpu::RenderPipeline,
}
impl SimpleShapeRenderPipeline {
    const MAX: usize = 1_000_000;
    pub fn new(core: &NGCore) -> Self {
        let max = SimpleShapeRenderPipeline::MAX as usize;
        let get_msaa_tex = |sample_count: u32| -> wgpu::Texture {
            core.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("MultiSample Texture"),
                size: wgpu::Extent3d {
                    width: core.surface_configuration.width,
                    height: core.surface_configuration.height,
                    depth_or_array_layers: 1
                },
                mip_level_count: 1,
                sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: core.surface_configuration.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            })
        };
        let multisample = match &core.config.msaa {
            MSAA::Disabled => {None}
            MSAA::Enable4x => {Some(get_msaa_tex(4))}
            //MSAA::Enable8x => {Some(get_msaa_tex(8))}
            //MSAA::Enable16x => {Some(get_msaa_tex(16))}
        };

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
        let multisample_state = match &core.config.msaa {
            MSAA::Disabled => {wgpu::MultisampleState::default()}
            MSAA::Enable4x => {
                wgpu::MultisampleState { count: 4, mask: !0, alpha_to_coverage_enabled: false }}
            //MSAA::Enable8x => {wgpu::MultisampleState { count: 8, mask: !0, alpha_to_coverage_enabled: true }}
            //MSAA::Enable16x => {wgpu::MultisampleState { count: 16, mask: !0, alpha_to_coverage_enabled: true }}
        };
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
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: multisample_state,
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
            multisample,
            vertex_buffer,
            index_buffer,
            trans_buffer,
            mats_buffer,
            data_bind_group,
            globals,
            data: None,
            pipeline,
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

        let output = core.surface.get_current_texture()?;
        let output_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let (view, resolve_target) = match &self.multisample {
            None => {
                (
                    output_view,
                    None,
                )
            }
            Some(t) => {
                (
                    t.create_view(&wgpu::TextureViewDescriptor::default()),
                    Some(&output_view),
                )
            }
        };
        let mut encoder =
            core.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("SimpleShapeRenderPipeline Command Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("SimpleShapeRenderPipeline Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(core.config.clear_color.into()),
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
    pub(crate) fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            transforms: vec![],
            materials: vec![],
            object_info: vec![],
        }
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

#[derive(Copy, Clone, Debug)]
pub struct SRState {
    pub fill: bool,
    pub color: FillStyle,
    pub thickness: f32,
    pub line_style: LineStyle,
    pub pos: Point,
    pub rotation: f32,
    pub kind: i32,
}
impl SRState {
    pub fn new() -> Self {
        Self {
            fill: true,
            color: FillStyle::Solid(Color::TEAL),
            thickness: 1.0,
            line_style: LineStyle::Center,
            pos: Point::new(0.0,0.0),
            rotation: 0.0,
            kind: 0,
        }
    }
}

pub struct ShapeGfx<'draw> {
    core: &'draw mut NGCore,
    pub data: SSRRenderData,
    pub state: SRState,
    pub states: Vec<SRState>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
        }
    }
    pub fn add(mut self, other: &Mesh) -> Mesh {
        let start = self.vertices.len() as u32;
        self.vertices.extend(&other.vertices);
        let indices: Vec<u32> = other.indices.iter().map(|i| i + start).collect();
        self.indices.extend(indices);
        self
    }
    pub fn style(mut self, style: FillStyle) -> Self {
        let minx = self.vertices.iter().fold(f32::MAX,|acc,v| if acc < v.x {acc} else {v.x});
        let maxx = self.vertices.iter().fold(f32::MIN, |acc, v| if acc > v.x {acc} else {v.x});
        let miny = self.vertices.iter().fold(f32::MAX,|acc,v| if acc < v.y {acc} else {v.y});
        let maxy = self.vertices.iter().fold(f32::MIN, |acc, v| if acc > v.y {acc} else {v.y});
        let dx = maxx - minx;
        let dy = maxy - miny;
        let midx = maxx - dx / 2.0;
        let midy = maxy - dy / 2.0;
        let texu = |x: f32| {
            (x - minx) / dx
        };
        let texv = |y: f32| {
            (y - miny) / dy
        };
        match style {
            FillStyle::Solid(c1) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.set_color(c1);
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                });
            }
            FillStyle::FadeDown(c1,c2) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                    if v.y < midy {
                        v.set_color(c1)
                    } else {v.set_color(c2)}
                })
            }
            FillStyle::FadeLeft(c1,c2) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                    if v.x < midx {
                        v.set_color(c1)
                    } else {v.set_color(c2)}
                })
            }
            FillStyle::Corners(c1, c2, c3, c4) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                    match (v.x < midx,v.y < midy) {
                        (true, true) => {v.set_color(c1)}
                        (false,true) => {v.set_color(c2)}
                        (false,false) => {v.set_color(c3)}
                        (true,false) => {v.set_color(c4)}
                    }
                })
            }
        };
        self
    }
}

impl <'draw> ShapeGfx<'draw> {
    pub fn set_position(&mut self, pos: Point) {self.state.pos = pos}
        pub fn p(&mut self, pos: Point){self.set_position(pos)}
    pub fn set_fill_style(&mut self, c: FillStyle) {self.state.color = c}
        pub fn fs(&mut self, fs: FillStyle){self.set_fill_style(fs)}
            pub fn solid(&mut self, c: Color){self.state.color = FillStyle::Solid(c)}
            pub fn fade_d(&mut self, c1: Color, c2: Color){self.state.color = FillStyle::FadeDown(c1,c2)}
            pub fn fade_l(&mut self, c1: Color, c2: Color){self.state.color = FillStyle::FadeLeft(c1,c2)}
            pub fn corners(&mut self, c1: Color, c2: Color, c3: Color, c4: Color){self.state.color = FillStyle::Corners(c1,c2,c3,c4)}
    pub fn set_line_thickness(&mut self, t: f32) {self.state.thickness = t}
        pub fn t(&mut self, lt: f32){self.set_line_thickness(lt)}
    pub fn set_line_style(&mut self, l: LineStyle) {self.state.line_style = l}
        pub fn ls(&mut self, ls: LineStyle){self.set_line_style(ls)}
    pub fn set_fill(&mut self, f: bool) {self.state.fill = f}
        pub fn f(&mut self, f: bool){self.set_fill(f)}
    pub fn set_rotation(&mut self, r: f32) {self.state.rotation = r}
        pub fn r(&mut self, r: f32){self.set_rotation(r)}

    pub fn translate(&mut self, t: Point) {self.state.pos += t}
    pub fn rotate(&mut self, r: f32) {self.state.rotation += r}

    pub fn current_pos(&self) -> Point {
        self.state.pos
    }
    pub fn current_rot(&self) -> f32 {
        self.state.rotation
    }

    pub fn push_state(&mut self) {self.states.push(self.state)}
    pub fn pop_state(&mut self) {self.state = self.states.pop().expect("Pop state")}

    pub fn data(&self) -> &SSRRenderData {
        self.data.as_ref()
    }
    pub fn new(core: &'draw mut NGCore) -> Self {
        Self {
            core,
            data: SSRRenderData::new(),
            state: SRState::new(),
            states: vec![],
        }
    }
    fn pt_quad(&self, p1: Point, p2: Point, p3: Point, p4: Point) -> Mesh {
        let (c1,c2,c3,c4) = self.colors();
        Mesh {
            vertices: vec![
            Vertex::point(p1).uv(0.0,0.0).rgba(c1),
            Vertex::point(p2).uv(0.0,1.0).rgba(c2),
            Vertex::point(p3).uv(1.0,1.0).rgba(c3),
            Vertex::point(p4).uv(1.0,0.0).rgba(c4)],
            indices: vec![0,1,2,2,3,0]
        }
    }
    fn pt_line_quad(&self, start: Point, end: Point, width: f32) -> Mesh {
        let swap = if start.y < end.y {-1.0} else {1.0};
        let pi = std::f32::consts::PI;
        let dx = (start.x - end.x) * 2.0; // width of line
        let dy = (start.y - end.y) * 2.0; // height of line
        let a = (dx/dy).atan(); // line slope in radians
        let sa = a - (pi/2.0);
        let sa2 = a + (pi/2.0);

        let bump2 = Point::new(width * sa.sin(), width * sa.cos());
        let bump = Point::new(width * sa2.sin(), width * sa2.cos());

        let (p1,p2,p3,p4) = match self.state.line_style {
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
        let mesh = self.pt_line_quad(start,end,self.state.thickness / 2.0);
        self.draw_mesh(mesh, Point::new(0.0,0.0))
    }
    pub fn rect(&mut self, pos: Point, size: Point) {
        if self.state.fill {
            let hx = size.x / 2.0;
            let hy = size.y / 2.0;
            let p1 = Point::new(-hx, -hy);
            let p2 = Point::new(-hx, hy);
            let p3 = Point::new(hx, hy);
            let p4 = Point::new(hx, -hy);
            let mesh = self.pt_quad(p1,p2,p3,p4);
            self.draw_mesh(mesh, pos);
        } else {
            let hx = size.x / 2.0;
            let hy = size.y / 2.0;
            let p1 = Point::new(-hx, -hy);
            let p2 = Point::new(-hx, hy);
            let p3 = Point::new(hx, hy);
            let p4 = Point::new(hx, -hy);
            let t = self.state.thickness;
            let ip1 = Point::new(-hx + t, -hy + t);
            let ip2 = Point::new(-hx + t, hy - t);
            let ip3 = Point::new(hx - t, hy - t);
            let ip4 = Point::new(hx - t, -hy + t);

            let (c1,c2,c3,c4) = self.colors();
            let vertices = vec![
                Vertex::point(p1).uv(0.0,0.0).rgba(c1), // 0
                Vertex::point(p2).uv(0.0,1.0).rgba(c2), // 1
                Vertex::point(p3).uv(1.0,1.0).rgba(c3), // 2
                Vertex::point(p4).uv(1.0,0.0).rgba(c4), // 3
                Vertex::point(ip1).uv(0.0,0.0).rgba(c1),// 4
                Vertex::point(ip2).uv(0.0,1.0).rgba(c2),// 5
                Vertex::point(ip3).uv(1.0,1.0).rgba(c3),// 6
                Vertex::point(ip4).uv(1.0,0.0).rgba(c4),// 7
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

            self.draw_mesh(mesh, pos);
        }
    }
    pub fn poly(&mut self, points: &Vec<Point>) {
        if !self.state.fill  {
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
    pub fn circle(&mut self, center: Point, radius: Point, resolution: f32) {
        self.arc(center,radius,0.0,std::f32::consts::TAU,resolution);
    }
    pub fn arc(&mut self, center: Point, radius: Point, arc_begin: f32, arc_end: f32, resolution: f32) {
        if self.state.fill {
            let (c1, _, _, c4) = self.colors();
            let mut vertices = vec![Vertex::new(0.0, 0.0).rgba(c1)];
            let mut indices: Vec<u32> = vec![];
            let start_angle = arc_begin;
            let end_angle = arc_end;
            let arc_length = (end_angle - start_angle).abs() * radius.len();
            let vertex_count = arc_length / resolution;
            let angle_step = (end_angle - start_angle).abs() / vertex_count;
            let mut a = start_angle;
            (0..=(vertex_count as u32 + 1)).for_each(|i| {
                if i <= vertex_count.floor() as u32 {
                    vertices.push(
                    Vertex::new(radius.x * a.cos(), radius.y * a.sin()).rgba(c4)
                   );
                }else{
                    vertices.push(
                        Vertex::new(radius.x * end_angle.cos(),radius.y * end_angle.sin()).rgba(c4)
                    );
                    indices.push(0);
                    indices.push(i+1);
                    indices.push(i);
                }
                if i > 0 {
                    indices.push(0);
                    indices.push(i);
                    indices.push(i - 1);
                }
                a += angle_step;
            });
            let mesh = Mesh {
                vertices,
                indices,
            };
            self.draw_mesh(mesh, center)
        } else {
            let (c1, _, _, c4) = self.colors();
            let mut vertices = vec![];
            let mut indices: Vec<u32> = vec![];
            let start_angle = arc_begin;
            let end_angle = arc_end;
            let arc_length = (end_angle - start_angle).abs() * radius.len();
            let vertex_count = arc_length / resolution;
            let angle_step = (end_angle - start_angle).abs() / vertex_count;
            let mut a = start_angle;
            (0..=(vertex_count as u32 + 1)).for_each(|i| {
                if i <= vertex_count.floor() as u32 {
                    vertices.push(
                        Vertex::new(radius.x * a.cos(), radius.y * a.sin()).rgba(c4)
                    );
                    vertices.push(
                        Vertex::new((radius.x - self.state.thickness) * a.cos(), (radius.y - self.state.thickness) * a.sin()).rgba(c1)
                    );
                }else{
                    vertices.push(
                        Vertex::new(radius.x * end_angle.cos(),radius.y * end_angle.sin()).rgba(c4)
                    );
                    vertices.push(
                        Vertex::new((radius.x - self.state.thickness) * end_angle.cos(), (radius.y - self.state.thickness) * end_angle.sin()).rgba(c1)
                    );


                }
                if i > 0 {
                    let v0 =  i*2 - 2;
                    let v1 = i*2 - 1;
                    let v2 = i*2;
                    let v3 = i*2 + 1;
                    indices.push(v0);
                    indices.push(v1);
                    indices.push(v2);

                    indices.push(v2);
                    indices.push(v1);
                    indices.push(v3);

                }
                a += angle_step;
            });
            let mesh = Mesh {
                vertices,
                indices,
            };
            self.draw_mesh(mesh, center)
        }
    }
    pub fn draw_mesh(
        &mut self,
        mesh: Mesh,
        pos: Point,
    ) {
        let start_vertex = self.data.vertices.len();
        let start_index = self.data.indices.len();
        for v in mesh.vertices { self.data.vertices.push(v); }
        for i in mesh.indices { self.data.indices.push(i); }
        let end_index = self.data.indices.len();
        let info = SSRObjectInfo {
            start_vertice: start_vertex as u32,
            start_index: start_index as u32,
            end_index: end_index as u32,
        };
        let transform = SSRTransform {
            x: self.state.pos.x + pos.x,
            y: self.state.pos.y + pos.y,
            r: self.state.rotation,
        };
        let material = SSRMaterial {kind: self.state.kind };
        self.data.transforms.push(transform);
        self.data.materials.push(material);
        self.data.object_info.push(info);
    }
    fn colors(&self) -> (Color, Color, Color, Color) {
        match self.state.color {
            FillStyle::Solid(color) => {(color,color,color,color)}
            FillStyle::FadeDown(color1, color2) => {(color2,color1,color1,color2)}
            FillStyle::FadeLeft(color1, color2) => {(color1,color1,color2,color2)}
            FillStyle::Corners(c1, c2, c3, c4) => {(c1,c2,c3,c4)}
        }
    }
    pub fn finish(&mut self) {
        self.core.cmd(NGCommand::Render(0, Box::new(self.data.to_owned())))
    }
}


pub struct MeshGen {}
impl MeshGen {
    pub fn rect(top_left: Point, bottom_right: Point) -> Mesh {
        let mut mesh = Mesh::new();
        mesh.vertices = vec![
            Vertex::point(top_left),
            Vertex::new(bottom_right.x,top_left.y),
            Vertex::point(bottom_right),
            Vertex::new(top_left.x,bottom_right.y)
        ];
        mesh.indices = vec![2,1,0, 3,2,0];
        mesh
    }
    pub fn rect_size(pos: Point, size: Point) -> Mesh{
        let halfx = size.x / 2.0;
        let halfy = size.y / 2.0;
        let top_left = Point::new(pos.x - halfx,pos.y - halfy);
        let bottom_right = Point::new(pos.x + halfx,pos.y + halfy);
        MeshGen::rect(top_left,bottom_right)
    }
    pub fn combine(mut meshes: Vec<Mesh>) -> Mesh {
        meshes.iter_mut().fold(Mesh::new(),|mut acc, m|acc.add(m))
    }
}