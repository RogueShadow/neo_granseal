use neo_granseal::{
    events::Event,
    start,
    GransealGameConfig,
    NeoGransealEventHandler,
    shape_pipeline,
    VSyncMode,
    core::{NGCommand,NGCore}
};

fn main() {
    start(
        Game::new(),
          GransealGameConfig::new()
        .vsync(VSyncMode::FastVSync));
}
struct Game {}
impl Game {
    fn new() -> Self {
        Self {}
    }
}
impl NeoGransealEventHandler for Game {
    fn event(&mut self, core: &mut NGCore, e: Event) {
        match e {
            Event::KeyEvent { .. } => {}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            Event::Draw => {}
            Event::Update(_) => {}
            Event::Load => {
                core.cmd(NGCommand::AddPipeline(Box::new(shape_pipeline::SimpleShapeRenderPipeline::new(&core))));
            }
            Event::Resized(_, _) => {}
            Event::Fps(_) => {}
        }
    }
}