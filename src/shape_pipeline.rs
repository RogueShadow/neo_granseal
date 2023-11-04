use crate::core::Image;
use crate::math::Vec2;
use crate::mesh::*;
use crate::{Color, GlobalUniforms, NGCore, NGError, NGRenderPipeline, MSAA};
use bytemuck_derive::{Pod, Zeroable};
use log::{error, warn};
use std::default::Default;
use wgpu::util::DeviceExt;
use wgpu::{MultisampleState, StoreOp, TextureViewDescriptor};

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
    pub fn uv(mut self, u: f32, v: f32) -> Self {
        self.u = u;
        self.v = v;
        self
    }
    pub fn point(p: Vec2) -> Self {
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
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SSRTransform {
    x: f32,
    y: f32,
    r: f32,
    rx: f32,
    ry: f32,
}
impl SSRTransform {
    pub fn new(pos: Vec2, r: f32, origin: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            r,
            rx: origin.x,
            ry: origin.y,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SSRMaterial {
    pub kind: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct SSRObjectInfo {
    pub(crate) bo_slot: Option<usize>,
    pub(crate) texture: Option<usize>,
    pub(crate) start_index: u32,
    pub(crate) end_index: u32,
    pub(crate) start_vertice: u32,
}
pub type BufferedObjectID = usize;
#[derive(Debug)]
pub struct MeshBuffer {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub texture: Option<usize>,
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
    pipeline2: wgpu::RenderPipeline,
    pipeline3: wgpu::RenderPipeline,
}
impl SimpleShapeRenderPipeline {
    const MAX: usize = 1_000_000;
    pub fn new(core: &NGCore) -> Self {
        let max = SimpleShapeRenderPipeline::MAX;
        let get_msaa_tex = |sample_count: u32| -> wgpu::Texture {
            core.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("MultiSample Texture"),
                size: wgpu::Extent3d {
                    width: core.surface_configuration.width,
                    height: core.surface_configuration.height,
                    depth_or_array_layers: 1,
                },
                view_formats: &[core.surface.get_capabilities(&core.adapter).formats[0]],
                mip_level_count: 1,
                sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: core.surface_configuration.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            })
        };
        let multisample = match &core.config.msaa {
            MSAA::Disabled => None,
            MSAA::Enable4x => Some(get_msaa_tex(4)),
            MSAA::Enable8x => Some(get_msaa_tex(8)),
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
            mapped_at_creation: false,
        });
        let trans_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Trans Buffer"),
            size: (max * std::mem::size_of::<SSRTransform>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mats_buffer = core.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSR Materials Buffer"),
            size: (max * std::mem::size_of::<SSRMaterial>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let data_bgl = core
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Transforms Materials BGL"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let data_bind_group = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transforms Materials BG"),
            layout: &data_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: trans_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: mats_buffer.as_entire_binding(),
                },
            ],
        });
        let globals = GlobalUniforms::new(core, (32.0, 32.0));
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
                    &core.textures.first().expect("Texture").bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let multisample_state = match &core.config.msaa {
            MSAA::Disabled => wgpu::MultisampleState::default(),
            MSAA::Enable4x => wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            MSAA::Enable8x => wgpu::MultisampleState {
                count: 8,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
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
        let pipeline2 = core
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("SSR Pipeline Offscreen"),
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
                multisample: MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });
        let pipeline3 = core
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("SSR Pipeline Offscreen"),
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
                multisample: MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::REPLACE),
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
            pipeline2,
            pipeline3,
        }
    }
    fn render_to(
        &mut self,
        core: &mut NGCore,
        texture: Option<&wgpu::Texture>,
        render_target: Option<Image>,
        replace: bool,
    ) {
        let disable_msaa = texture.is_none();
        let texture = match (texture, render_target) {
            (Some(texture), _) => texture,
            (None, Some(image)) => {
                if let Some(texture_info) = core.textures.get(image.texture) {
                    &texture_info.texture
                } else {
                    return;
                }
            }
            (_, _) => return,
        };
        let data = match self.data.as_ref() {
            Some(d) => d,
            None => return,
        };

        self.vertex_buffer = core
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Simple Shape Renderer Pipeline Vertex Buffer"),
                contents: bytemuck::cast_slice(data.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        self.index_buffer = core
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Simple Shape Renderer Pipeline Index Buffer"),
                contents: bytemuck::cast_slice(data.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });

        core.queue.write_buffer(
            &self.trans_buffer,
            0,
            bytemuck::cast_slice(data.transforms.as_slice()),
        );
        core.queue.write_buffer(
            &self.mats_buffer,
            0,
            bytemuck::cast_slice(data.materials.as_slice()),
        );

        let texture_view_descriptor = TextureViewDescriptor {
            label: None,
            format: Some(texture.format()),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        };
        let output_view = texture.create_view(&texture_view_descriptor);
        let (view, resolve_target) = match &self.multisample {
            Some(t) if !disable_msaa => {
                (t.create_view(&texture_view_descriptor), Some(&output_view))
            }
            _ => (output_view, None),
        };
        let mut encoder = core
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("SimpleShapeRenderPipeline Command Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("SimpleShapeRenderPipeline Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target,
                ops: wgpu::Operations {
                    load: if render_target.is_some() {
                        wgpu::LoadOp::Load
                    } else {
                        wgpu::LoadOp::Clear(core.config.clear_color.into())
                    },
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if disable_msaa {
            if replace {
                render_pass.set_pipeline(&self.pipeline3);
            } else {
                render_pass.set_pipeline(&self.pipeline2);
            }
        } else {
            render_pass.set_pipeline(&self.pipeline);
        }
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.set_bind_group(0, &self.globals.bind_group, &[]);
        render_pass.set_bind_group(1, &self.data_bind_group, &[]);
        render_pass.set_bind_group(
            2,
            &core.textures.first().expect("Something").bind_group,
            &[],
        );

        for (i, obj) in data.object_info.iter().enumerate() {
            let index = i as u32..i as u32 + 1;
            match obj.bo_slot {
                None => {
                    if let Some(tex) = obj.texture {
                        render_pass.set_bind_group(2, &core.textures[tex].bind_group, &[]);
                    } else {
                        render_pass.set_bind_group(
                            2,
                            &core.textures.first().expect("Something").bind_group,
                            &[],
                        );
                    }
                    render_pass.draw_indexed(
                        obj.start_index..obj.end_index,
                        obj.start_vertice as i32,
                        index,
                    );
                }
                Some(vbi) => {
                    let vb = match core.mesh_buffers.get(vbi) {
                        Some(vb) => vb,
                        None => {
                            error!("MeshBuffer index {:?} out of bounds.", vbi);
                            return;
                        }
                    };
                    if let Some(tex) = vb.texture {
                        render_pass.set_bind_group(2, &core.textures[tex].bind_group, &[]);
                    } else {
                        render_pass.set_bind_group(
                            2,
                            &core.textures.first().expect("Something").bind_group,
                            &[],
                        );
                    }
                    render_pass.set_vertex_buffer(0, vb.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(vb.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(
                        0..(vb.index_buffer.size() as u32 / std::mem::size_of::<i32>() as u32),
                        0,
                        index,
                    );
                    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                }
            }
        }
        drop(render_pass);
        core.queue.submit(std::iter::once(encoder.finish()));
    }
}
impl NGRenderPipeline for SimpleShapeRenderPipeline {
    fn render(&mut self, core: &mut NGCore) -> Result<(), NGError> {
        match core.surface.get_current_texture() {
            Result::Ok(surface_texture) => {
                self.render_to(core, Some(&surface_texture.texture), None, false);
                core.window.pre_present_notify();
                surface_texture.present();
            }
            Result::Err(_err) => {
                core.surface
                    .configure(&core.device, &core.surface_configuration);
            }
        }
        Ok(())
    }
    fn render_image(&mut self, core: &mut NGCore, texture: Image, replace: bool) {
        self.render_to(core, None, Some(texture), replace);
    }
    fn set_data(&mut self, data: Box<dyn std::any::Any>) {
        let rd = data.downcast::<SSRRenderData>().expect("Get Render Data");
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

pub struct ShapeGfx<'draw> {
    core: &'draw mut NGCore,
    data: SSRRenderData,
    offset: Vec2,
    rotation: f32,
    rotation_origin: Vec2,
}

impl<'draw> ShapeGfx<'draw> {
    pub fn set_offset(&mut self, cursor: Vec2) {
        self.offset = cursor
    }
    pub fn translate_offset(&mut self, t: Vec2) {
        self.offset += t
    }
    pub fn rotate(&mut self, r: f32) {
        self.rotation += r
    }
    pub fn set_rotation(&mut self, r: f32) {
        self.rotation = r
    }
    pub fn set_rotation_origin(&mut self, origin: Vec2) {
        self.rotation_origin = origin
    }

    pub fn data(&self) -> &SSRRenderData {
        self.data.as_ref()
    }
    pub fn new(core: &'draw mut NGCore) -> Self {
        Self {
            core,
            data: SSRRenderData::new(),
            offset: Vec2::ZERO,
            rotation: 0.0,
            rotation_origin: Vec2::ZERO,
        }
    }
    pub fn draw_image(&mut self, image: &Image, pos: Vec2) {
        let size = if let Some(start) = image.start {
            if let Some(end) = image.end {
                end - start
            } else {
                image.size
            }
        } else {
            image.size
        };
        let mut mesh = rect_filled(Vec2::ZERO, size, FillStyle::Solid(Color::WHITE));
        mesh.texture(image, true);
        self.draw_mesh(&mesh, pos);
    }
    pub fn draw_mesh(&mut self, mesh: &Mesh, pos: Vec2) {
        match (mesh.buffer.get(), mesh.dirty.get(), mesh.buffer_id.get()) {
            (true, _, None) => {
                let buffer_id = self.core.buffer_object(mesh);
                mesh.buffer_id.set(Some(buffer_id));
                mesh.buffer.set(true);
                mesh.dirty.set(false);
                self.draw_buffer(buffer_id, pos);
            }
            (true, false, Some(bid)) => {
                self.draw_buffer(bid, pos);
            }
            (true, true, Some(bid)) => {
                self.core.update_buffer_object(bid, mesh);
                self.draw_buffer(bid, pos);
            }
            (false, _, _) => {
                let texture = match mesh.image {
                    Some(image) => Some(image.texture),
                    None => None,
                };
                let start_vertex = self.data.vertices.len();
                let start_index = self.data.indices.len();
                self.data.vertices.extend(mesh.vertices.as_slice());
                self.data.indices.extend(mesh.indices.as_slice());
                let end_index = self.data.indices.len();
                let info = SSRObjectInfo {
                    bo_slot: None,
                    start_vertice: start_vertex as u32,
                    start_index: start_index as u32,
                    end_index: end_index as u32,
                    texture,
                };
                let transform = SSRTransform {
                    x: self.offset.x + pos.x,
                    y: self.offset.y + pos.y,
                    r: self.rotation,
                    rx: self.rotation_origin.x,
                    ry: self.rotation_origin.y,
                };
                let material = SSRMaterial {
                    kind: if mesh.image.is_none() { 0 } else { 1 },
                };
                self.data.transforms.push(transform);
                self.data.materials.push(material);
                self.data.object_info.push(info);
            }
        }
    }
    pub fn draw_buffer(&mut self, buffer_id: usize, pos: Vec2) {
        match self.core.buffered_objects.get_mut(buffer_id) {
            Some(info) => {
                let transform = SSRTransform {
                    x: self.offset.x + pos.x,
                    y: self.offset.y + pos.y,
                    r: self.rotation,
                    rx: self.rotation_origin.x,
                    ry: self.rotation_origin.y,
                };
                let material = SSRMaterial { kind: 0 };

                self.data.transforms.push(transform);
                self.data.materials.push(material);
                self.data.object_info.push(*info);
            }
            None => {
                warn!("No buffer at index {:?}", buffer_id)
            }
        }
    }
    pub fn finish(&mut self) {
        self.core.render(0, Box::new(self.data.to_owned()));
    }
    pub fn render_image(&mut self, image: &Image, replace: bool) {
        self.core
            .render_image(0, Box::new(self.data.to_owned()), image, replace);
    }
}
