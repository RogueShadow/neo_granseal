use crate::math::Vec3;
use crate::prelude::{Color, Mesh, NGError};
use image::{EncodableLayout, ImageResult};
use std::collections::HashMap;
use std::path::Path;
use wgpu::util::{DeviceExt, TextureDataOrder};

#[derive(Default)]
pub struct MeshManager {
    meshes: HashMap<u32, MeshBuffer>,
}
impl MeshManager {
    pub fn add_mesh(&mut self, device: &wgpu::Device, mesh: Mesh) -> MeshHandle {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let mesh_buffer = MeshBuffer {
            vertex_buffer,
            index_buffer,
        };
        let handle = self.new_handle();
        self.meshes.insert(handle, mesh_buffer);
        MeshHandle { handle }
    }
    pub fn destroy_mesh(&mut self, handle: &MeshHandle) -> bool {
        let mesh_buffer = self.meshes.remove(&handle.handle);
        if let Some(mb) = mesh_buffer {
            mb.index_buffer.destroy();
            mb.vertex_buffer.destroy();
            true
        } else {
            false
        }
    }
    pub fn update_mesh(&mut self, device: &wgpu::Device, handle: &MeshHandle, mesh: Mesh) {
        let mesh_buffer = self.meshes.get_mut(&handle.handle);
        if let Some(mb) = mesh_buffer {
            mb.vertex_buffer.destroy();
            mb.index_buffer.destroy();
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });
        let mesh_buffer = MeshBuffer {
            vertex_buffer,
            index_buffer,
        };
        self.meshes.insert(handle.handle, mesh_buffer);
    }
    fn new_handle(&self) -> u32 {
        let mut handle = 0u32;
        while self.meshes.contains_key(&handle) {
            handle += 1;
        }
        handle
    }
}

#[derive(Clone, Copy)]
pub struct MeshHandle {
    handle: u32,
}

pub struct MeshBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

///////////////////////////////////

#[derive(Default)]
pub struct MaterialManager {
    materials: HashMap<u32, Material>,
}
impl MaterialManager {
    pub fn add_material(&mut self, device: &wgpu::Device, material: Material) -> MaterialHandle {
        let handle = self.new_handle();
        self.materials.insert(handle, material);
        MaterialHandle { handle }
    }
    pub fn destroy_material(&mut self, handle: &MaterialHandle) -> bool {
        let material = self.materials.remove(&handle.handle);
        match material {
            Some(_mat) => true,
            None => false,
        }
    }
    pub fn update_material(
        &mut self,
        device: &wgpu::Device,
        handle: &MaterialHandle,
        material: Material,
    ) {
        self.materials.insert(handle.handle, material);
    }
    fn new_handle(&self) -> u32 {
        let mut handle = 0u32;
        while self.materials.contains_key(&handle) {
            handle += 1;
        }
        handle
    }
}

#[derive(Clone, Copy)]
pub struct MaterialHandle {
    handle: u32,
}

pub enum Material {
    Solid { color: Color },
    Texture { texture: TextureHandle, tint: Color },
}

///////////////////////////////////////

#[derive(Default)]
pub struct TextureManager {
    textures: HashMap<u32, TextureBuffer>,
}
impl TextureManager {
    pub fn add_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_load: TextureLoad,
    ) -> Option<TextureBuffer> {
        let (tex, filter) = match texture_load {
            TextureLoad::File { path, filter } => (image::open(path), filter),
            TextureLoad::Data { data, filter } => (image::load_from_memory(&data), filter),
        };
        let tex = if tex.is_ok() {
            tex.unwrap().to_rgba8()
        } else {
            return None;
        };

        let texture_descriptor = wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: wgpu::Extent3d {
                width: tex.width(),
                height: tex.height(),
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
        let texture = device.create_texture_with_data(
            queue,
            &texture_descriptor,
            TextureDataOrder::LayerMajor,
            tex.as_bytes(),
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: filter,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
        let handle = self.new_handle();
        self.textures.insert(
            handle,
            TextureBuffer {
                texture,
                bind_group_layout,
                bind_group,
            },
        )
    }
    pub fn destroy_texture(&mut self, handle: &TextureHandle) -> bool {
        let tex_buffer = self.textures.remove(&handle.handle);
        if let Some(mb) = tex_buffer {
            mb.texture.destroy();
            true
        } else {
            false
        }
    }
    pub fn update_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        handle: &TextureHandle,
        texture_load: TextureLoad,
    ) {
        let texture_buffer = self.textures.get_mut(&handle.handle);
        if let Some(tb) = texture_buffer {
            tb.texture.destroy();
        }
        let (tex, filter) = match texture_load {
            TextureLoad::File { path, filter } => (image::open(path), filter),
            TextureLoad::Data { data, filter } => (image::load_from_memory(&data), filter),
        };
        let tex = if tex.is_ok() {
            tex.unwrap().to_rgba8()
        } else {
            return;
        };

        let texture_descriptor = wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: wgpu::Extent3d {
                width: tex.width(),
                height: tex.height(),
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
        let texture = device.create_texture_with_data(
            queue,
            &texture_descriptor,
            TextureDataOrder::LayerMajor,
            tex.as_bytes(),
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: filter,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
        self.textures.insert(
            handle.handle,
            TextureBuffer {
                texture,
                bind_group_layout,
                bind_group,
            },
        );
    }
    pub fn get_texture(&self, handle: TextureHandle) -> Option<&TextureBuffer> {
        self.textures.get(&handle.handle)
    }
    fn new_handle(&self) -> u32 {
        let mut handle = 0u32;
        while self.textures.contains_key(&handle) {
            handle += 1;
        }
        handle
    }
}

pub struct TextureHandle {
    handle: u32,
}

pub struct TextureBuffer {
    texture: wgpu::Texture,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

pub enum TextureLoad {
    File {
        path: Box<Path>,
        filter: wgpu::FilterMode,
    },
    Data {
        data: Box<[u8]>,
        filter: wgpu::FilterMode,
    },
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct ObjectManager {
    objects: HashMap<u32, Object>,
}

impl ObjectManager {
    pub fn add_object(&mut self, object: Object) -> ObjectHandle {
        let handle = self.new_handle();
        self.objects.insert(handle, object);
        ObjectHandle { handle }
    }
    pub fn update_object(&mut self, handle: ObjectHandle, object: Object) {
        self.objects.insert(handle.handle, object);
    }
    pub fn destroy_object(&mut self, handle: ObjectHandle) {
        self.objects.remove(&handle.handle);
    }
    fn new_handle(&self) -> u32 {
        let mut handle = 0u32;
        while self.objects.contains_key(&handle) {
            handle += 1;
        }
        handle
    }
    pub fn get_object(&mut self, handle: ObjectHandle) -> Option<&mut Object> {
        self.objects.get_mut(&handle.handle)
    }
}

#[derive(Clone, Copy)]
pub struct Object {
    mesh: MeshHandle,
    material: MaterialHandle,
    transform: Transform,
}

#[derive(Clone, Copy)]
pub struct ObjectHandle {
    handle: u32,
}

#[derive(Clone, Copy)]
pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    rot_origin: Vec3,
}
