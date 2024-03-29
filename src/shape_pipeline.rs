use crate::core::Image;
use crate::math::Vec2;
use crate::mesh::*;
use crate::{Color, GlobalUniforms, NGCore, NGError, NGRenderPipeline, MSAA};
use bytemuck_derive::{Pod, Zeroable};
use log::{error, warn};
use std::default::Default;
use wgpu::util::DeviceExt;
use wgpu::{LoadOp, MultisampleState, StoreOp, TextureViewDescriptor};

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
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
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
    depth_stencil: wgpu::Texture,
    multisample: Option<wgpu::Texture>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    trans_buffer: wgpu::Buffer,
    mats_buffer: wgpu::Buffer,
    data_bind_group: wgpu::BindGroup,
    globals: GlobalUniforms,
    objects: Vec<SSRObjectInfo>,
    pipeline: wgpu::RenderPipeline,
    pipeline2: wgpu::RenderPipeline,
    pipeline3: wgpu::RenderPipeline,
    depth_pass: wgpu::RenderPipeline,
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
        let vertex_state = wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2,  2 => Float32x4],
            }],
        };
        let primitive_state = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        };
        let pipeline = core
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("SSR Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: vertex_state.clone(),
                primitive: primitive_state.clone(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32FloatStencil8,
                    depth_write_enabled: false,
                    depth_compare: wgpu::CompareFunction::GreaterEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
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
                vertex: vertex_state.clone(),
                primitive: primitive_state.clone(),
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
                vertex: vertex_state.clone(),
                primitive: primitive_state.clone(),
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
        let depth_pass = core
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Depth Only Pass"),
                layout: Some(&pipeline_layout),
                vertex: vertex_state.clone(),
                primitive: primitive_state.clone(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32FloatStencil8,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::GreaterEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: multisample_state,
                fragment: None,
                multiview: None,
            });
        let depth_stencil = core.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Stencil Texture"),
            size: wgpu::Extent3d {
                width: core.surface_configuration.width,
                height: core.surface_configuration.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: match multisample_state {
                MultisampleState { count, .. } => count,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32FloatStencil8,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        Self {
            depth_stencil,
            multisample,
            vertex_buffer,
            index_buffer,
            trans_buffer,
            mats_buffer,
            data_bind_group,
            globals,
            objects: vec![],
            pipeline,
            pipeline2,
            pipeline3,
            depth_pass,
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

        let output_view = texture.create_view(&TextureViewDescriptor::default());
        let (view, resolve_target) = match &self.multisample {
            Some(t) if !disable_msaa => (
                t.create_view(&TextureViewDescriptor::default()),
                Some(&output_view),
            ),
            _ => (output_view, None),
        };

        if !disable_msaa {
            let depth_view = self
                .depth_stencil
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = core
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("SimpleShapeRenderPipeline Command Encoder"),
                });

            let mut depth_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Depth Pass"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            depth_pass.set_pipeline(&self.depth_pass);

            depth_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            depth_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            depth_pass.set_bind_group(0, &self.globals.bind_group, &[]);
            depth_pass.set_bind_group(1, &self.data_bind_group, &[]);
            depth_pass.set_bind_group(
                2,
                &core.textures.first().expect("Something").bind_group,
                &[],
            );

            self.draw_objects(core, &mut depth_pass);

            drop(depth_pass);

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
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

            self.draw_objects(core, &mut render_pass);

            drop(render_pass);
            core.queue.submit(std::iter::once(encoder.finish()));
        } else {
            let mut encoder = core
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("SimpleShapeRenderPipeline Command Encoder"),
                });
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
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

            self.draw_objects(core, &mut render_pass);

            drop(render_pass);
            core.queue.submit(std::iter::once(encoder.finish()));
        }
    }

    fn draw_objects<'pass, 'draw: 'pass>(
        &'draw self,
        core: &'pass NGCore,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        for (i, obj) in self.objects.iter().enumerate() {
            let index = i as u32..i as u32 + 1;
            match obj.bo_slot {
                None => {
                    if let Some(tex) = obj.texture {
                        pass.set_bind_group(2, &core.textures[tex].bind_group, &[]);
                    } else {
                        pass.set_bind_group(
                            2,
                            &core.textures.first().expect("Something").bind_group,
                            &[],
                        );
                    }
                    pass.draw_indexed(
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
                        pass.set_bind_group(2, &core.textures[tex].bind_group, &[]);
                    } else {
                        pass.set_bind_group(
                            2,
                            &core.textures.first().expect("Something").bind_group,
                            &[],
                        );
                    }
                    pass.set_vertex_buffer(0, vb.vertex_buffer.slice(..));
                    pass.set_index_buffer(vb.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(
                        0..(vb.index_buffer.size() as u32 / std::mem::size_of::<i32>() as u32),
                        0,
                        index,
                    );
                    pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                    pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                }
            }
        }
    }

    // pub fn my_render(
    //     &mut self,
    //     core: &mut NGCore,
    //     render_target: &wgpu::Texture,
    //     multisample_target: Option<&wgpu::Texture>,
    //     depth_buffer: Option<&wgpu::Texture>,
    //     stencil: bool,
    //     clear: Option<Color>,
    // ) {
    //     let data = match self.data.as_ref() {
    //         Some(d) => d,
    //         None => return,
    //     };
    //     self.vertex_buffer = core.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: Some("Simple Shape Renderer Pipeline Vertex Buffer"),
    //         contents: bytemuck::cast_slice(data.vertices.as_slice()),
    //         usage: wgpu::BufferUsages::VERTEX,
    //     });
    //     self.index_buffer = core.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: Some("Simple Shape Renderer Pipeline Index Buffer"),
    //         contents: bytemuck::cast_slice(data.indices.as_slice()),
    //         usage: wgpu::BufferUsages::INDEX,
    //     });
    //     core.queue.write_buffer(
    //         &self.trans_buffer,
    //         0,
    //         bytemuck::cast_slice(data.transforms.as_slice()),
    //     );
    //     core.queue.write_buffer(
    //         &self.mats_buffer,
    //         0,
    //         bytemuck::cast_slice(data.materials.as_slice()),
    //     );
    //
    //     let (view, resolve_target) = if let Some(msaa_tex) = multisample_target {
    //             (
    //                 &msaa_tex.create_view(&TextureViewDescriptor::default()),
    //                 Some(&render_target.create_view(&TextureViewDescriptor::default()))
    //             )
    //         } else {
    //             (
    //                 &render_target.create_view(&TextureViewDescriptor::default()),
    //                 None
    //             )
    //     };
    //
    //     let mut encoder = core.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //         label: Some("SimpleShapeRenderPipeline Command Encoder"),
    //     });
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("SimpleShapeRenderPipeline Render Pass"),
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view,
    //             resolve_target,
    //             ops: wgpu::Operations {
    //                 load: if let Some(color) = clear {
    //                     wgpu::LoadOp::Clear(color.into())
    //                 } else {
    //                     wgpu::LoadOp::Load
    //                 },
    //                 store: StoreOp::Store,
    //             },
    //         })],
    //         depth_stencil_attachment: if depth_buffer.is_some() {
    //             let view = depth_buffer.unwrap().create_view(&TextureViewDescriptor::default());
    //             Some(wgpu::RenderPassDepthStencilAttachment {
    //                 view: &view,
    //                 depth_ops: Some(wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(0.0),
    //                     store: wgpu::StoreOp::Store,
    //                 }),
    //                 stencil_ops: None,
    //             })
    //         } else {
    //             None
    //         },
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     });
    //     if multisample_target.is_none() {
    //         render_pass.set_pipeline(&self.pipeline2);
    //     } else {
    //         render_pass.set_pipeline(&self.pipeline);
    //     }
    //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    //     render_pass.set_bind_group(0, &self.globals.bind_group, &[]);
    //     render_pass.set_bind_group(1, &self.data_bind_group, &[]);
    //     render_pass.set_bind_group(
    //         2,
    //         &core.textures.first().expect("Something").bind_group,
    //         &[],
    //     );
    //
    //     for (i, obj) in data.object_info.iter().enumerate() {
    //         let index = i as u32..i as u32 + 1;
    //         match obj.bo_slot {
    //             None => {
    //                 if let Some(tex) = obj.texture {
    //                     render_pass.set_bind_group(2, &core.textures[tex].bind_group, &[]);
    //                 } else {
    //                     render_pass.set_bind_group(
    //                         2,
    //                         &core.textures.first().expect("Something").bind_group,
    //                         &[],
    //                     );
    //                 }
    //                 render_pass.draw_indexed(
    //                     obj.start_index..obj.end_index,
    //                     obj.start_vertice as i32,
    //                     index,
    //                 );
    //             }
    //             Some(vbi) => {
    //                 let vb = match core.mesh_buffers.get(vbi) {
    //                     Some(vb) => vb,
    //                     None => {
    //                         error!("MeshBuffer index {:?} out of bounds.", vbi);
    //                         return;
    //                     }
    //                 };
    //                 if let Some(tex) = vb.texture {
    //                     render_pass.set_bind_group(2, &core.textures[tex].bind_group, &[]);
    //                 } else {
    //                     render_pass.set_bind_group(
    //                         2,
    //                         &core.textures.first().expect("Something").bind_group,
    //                         &[],
    //                     );
    //                 }
    //                 render_pass.set_vertex_buffer(0, vb.vertex_buffer.slice(..));
    //                 render_pass
    //                     .set_index_buffer(vb.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    //                 render_pass.draw_indexed(
    //                     0..(vb.index_buffer.size() as u32 / std::mem::size_of::<i32>() as u32),
    //                     0,
    //                     index,
    //                 );
    //                 render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //                 render_pass
    //                     .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    //             }
    //         }
    //     }
    //     drop(render_pass);
    //     core.queue.submit(std::iter::once(encoder.finish()));
    // }
}
impl NGRenderPipeline for SimpleShapeRenderPipeline {
    fn render(&mut self, core: &mut NGCore) -> Result<(), NGError> {
        match core.surface.get_current_texture() {
            Result::Ok(surface_texture) => {
                self.render_to(core, Some(&surface_texture.texture), None, false);
                core.window.pre_present_notify();
                surface_texture.present();
            }
            Result::Err(_err) => match core.window.is_minimized() {
                None => {
                    core.surface
                        .configure(&core.device, &core.surface_configuration);
                }
                Some(true) => {
                    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
                }
                Some(false) => {
                    core.surface
                        .configure(&core.device, &core.surface_configuration);
                }
            },
        }
        Ok(())
    }
    fn render_image(&mut self, core: &mut NGCore, texture: Image, replace: bool) {
        self.render_to(core, None, Some(texture), replace);
    }
    fn set_data(&mut self, core: &mut NGCore, data: Box<dyn std::any::Any>) {
        let rd = data.downcast::<SSRRenderData>().expect("Get Render Data");
        let data = *rd;

        self.objects.clear();
        self.objects.extend(data.object_info);

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
    }

    fn set_globals(&mut self, globals: GlobalUniforms) {
        self.globals = globals;
    }

    fn resized(&mut self, core: &mut NGCore, width: u32, height: u32) {
        match &self.multisample {
            Some(msaa_tex) => {
                msaa_tex.destroy();
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
                self.multisample = match &core.config.msaa {
                    MSAA::Disabled => None,
                    MSAA::Enable4x => Some(get_msaa_tex(4)),
                    MSAA::Enable8x => Some(get_msaa_tex(8)),
                    //MSAA::Enable16x => {Some(get_msaa_tex(16))}
                };
                self.depth_stencil.destroy();
                self.depth_stencil = core.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("Depth Stencil Texture"),
                    size: wgpu::Extent3d {
                        width: core.surface_configuration.width,
                        height: core.surface_configuration.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: match core.config.msaa {
                        MSAA::Disabled => 1,
                        MSAA::Enable4x => 4,
                        MSAA::Enable8x => 8,
                    },
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Depth32FloatStencil8,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING
                        | wgpu::TextureUsages::COPY_DST
                        | wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                });
            }
            None => {}
        }
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
    tint: Color,
    depth: f32,
}

impl<'draw> ShapeGfx<'draw> {
    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth;
    }
    pub fn adjust_depth(&mut self, depth_adjustment: f32) {
        self.depth += depth_adjustment;
    }
    pub fn set_tint(&mut self, color: Color) {
        self.tint = color;
    }
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
            tint: Color::WHITE,
            depth: 0.0,
        }
    }
    pub fn draw_image(&mut self, image: &Image, pos: Vec2) {
        let mut mesh = rect_filled(Vec2::ZERO, image.size(), FillStyle::Solid(Color::WHITE));
        mesh.texture(image, true);
        mesh.set_z_depth(self.depth);
        self.draw_mesh(&mesh, pos);
    }
    pub fn draw_image_sized(&mut self, image: &Image, size: Vec2, pos: Vec2) {
        let mut mesh = rect_filled(Vec2::ZERO, size, FillStyle::Solid(Color::WHITE));
        mesh.texture(image, true);
        mesh.set_z_depth(self.depth);
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
                    Some(image) => {
                        if let Some(atlas) = image.atlas {
                            Some(atlas.0)
                        } else {
                            Some(image.texture)
                        }
                    }
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
                    r: self.tint.r,
                    g: self.tint.g,
                    b: self.tint.b,
                    a: self.tint.a,
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
                let material = SSRMaterial {
                    kind: 0,
                    r: self.tint.r,
                    g: self.tint.g,
                    b: self.tint.b,
                    a: self.tint.a,
                };

                self.data.transforms.push(transform);
                self.data.materials.push(material);
                self.data.object_info.push(*info);
            }
            None => {
                warn!("No buffer at index {:?}", buffer_id)
            }
        }
    }
    fn submit(&mut self) {
        self.core.render(0, Box::new(self.data.to_owned()));
    }
    pub fn render_image(&mut self, image: &Image, replace: bool) {
        self.core
            .render_image(0, Box::new(self.data.to_owned()), image, replace);
        self.data = SSRRenderData::new();
    }
}
impl Drop for ShapeGfx<'_> {
    fn drop(&mut self) {
        self.submit();
    }
}
