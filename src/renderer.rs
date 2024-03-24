use crate::core::{Image, NGCore, NGError};
use crate::managers::{MaterialManager, MeshManager, Object, ObjectManager, TextureManager};
use crate::{GlobalUniforms, NGRenderPipeline};
use std::any::Any;

#[derive(Default)]
pub struct RenderData {
    pub texture: TextureManager,
    pub mesh: MeshManager,
    pub material: MaterialManager,
    pub object: ObjectManager,
}

impl RenderData {}

pub struct Renderer {
    data: RenderData,
}

impl Renderer {
    pub(crate) fn render_to(
        &self,
        core: &mut NGCore,
        texture: Option<&wgpu::Texture>,
        render_target: Option<&wgpu::Texture>,
        replace: bool,
    ) {
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            data: RenderData::default(),
        }
    }
}

impl NGRenderPipeline for Renderer {
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

    fn render_image(&mut self, core: &mut NGCore, texture: Image, replace: bool) {}

    fn set_data(&mut self, core: &mut NGCore, data: Box<dyn Any>) {
        let data = data
            .downcast::<Vec<Object>>()
            .expect("Unwrap the object vec");
        for obj in data.as_slice() {
            self.data.object.add_object(obj.clone());
        }
    }

    fn set_globals(&mut self, globals: GlobalUniforms) {}

    fn resized(&mut self, core: &mut NGCore, width: u32, height: u32) {}
}
