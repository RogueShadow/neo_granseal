use neo_granseal::{
    events::Event,
    start,
    GransealGameConfig,
    NeoGransealEventHandler,
    shape_pipeline,
    VSyncMode,
    core::{NGCommand,NGCore}
};
use neo_granseal::shape_pipeline::{SSRGraphics, SSRRenderData};

fn main() {
    start(
        Game::new(),
        GransealGameConfig::new()
              .vsync(VSyncMode::FastVSync)
              .size(128 * 5, 128 * 5)
    );
}
struct Game {}
impl Game { fn new() -> Self { Self {} } }

impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent { .. } => {}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            Event::Draw => {
                let mut gfx = SSRGraphics::new();
                gfx.clear();
                gfx.fill(1.0,0.0,0.0);
                gfx.fill_rect(core.config.width as f32 - 64.0,core.config.height as f32 - 64.0,128.0,128.0);
                gfx.fill(0.0,1.0,0.0);
                gfx.fill_rect(192.0,192.0,128.0,128.0);
                gfx.fill(0.0,0.0,1.0);
                gfx.fill_rect(64.0,64.0,128.0,128.0);
                gfx.fill(1.0,0.0,1.0);
                gfx.fill_rect(320.0,320.0,128.0,128.0);
                gfx.fill(1.0,1.0,0.0);
                gfx.fill_rect(448.0,448.0,128.0,128.0);
                core.cmd(NGCommand::Render(0,Box::new(gfx.data)));
            }
            Event::Update(_) => {}
            Event::Load => {
                core.cmd(NGCommand::AddPipeline(Box::new(shape_pipeline::SimpleShapeRenderPipeline::new(&core))));
            }
            Event::Resized(_, _) => {}
            Event::Fps(_) => {}
        }
    }
}